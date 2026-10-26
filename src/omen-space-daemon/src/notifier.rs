use log::{info, warn};
use std::process::Command;
use glob::glob;
use zbus::Connection;

/// Last notification id per condition tag, so a repeated/updated notice replaces the old one (#327).
static NOTIFICATION_IDS: std::sync::Mutex<Option<std::collections::HashMap<String, u32>>> = std::sync::Mutex::new(None);

pub struct DesktopNotifier;

impl DesktopNotifier {
    /// Discover the active graphical desktop user's session environment `(user, display, dbus_addr, xdg_runtime, home)`.
    fn find_user_session_env() -> Option<(String, String, String, String, String)> {
        let entries = glob("/proc/*/environ").ok()?;
        for entry in entries.flatten() {
            let Ok(content) = std::fs::read(&entry) else { continue };
            let env_str = String::from_utf8_lossy(&content);
            if !env_str.contains("DISPLAY=") && !env_str.contains("WAYLAND_DISPLAY=") {
                continue;
            }
            let mut display = ":0".to_string();
            let mut user = String::new();
            let mut dbus_addr = String::new();
            let mut xdg_runtime = String::new();
            let mut home = String::new();

            for item in env_str.split('\0') {
                if let Some((k, v)) = item.split_once('=') {
                    match k {
                        "DISPLAY" => display = v.to_string(),
                        "USER" | "LOGNAME" if user.is_empty() => user = v.to_string(),
                        "DBUS_SESSION_BUS_ADDRESS" => dbus_addr = v.to_string(),
                        "XDG_RUNTIME_DIR" => xdg_runtime = v.to_string(),
                        "HOME" => home = v.to_string(),
                        _ => {}
                    }
                }
            }

            let is_valid_username = !user.is_empty()
                && user != "root"
                && user.chars().all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-');

            if is_valid_username && (!dbus_addr.is_empty() || !xdg_runtime.is_empty()) {
                if dbus_addr.is_empty() && !xdg_runtime.is_empty() {
                    let bus_path = format!("{}/bus", xdg_runtime);
                    if std::path::Path::new(&bus_path).exists() {
                        dbus_addr = format!("unix:path={}", bus_path);
                    }
                }
                if home.is_empty() {
                    home = format!("/home/{}", user);
                }
                return Some((user, display, dbus_addr, xdg_runtime, home));
            }
        }
        None
    }

    /// Check whether the user has 90°C thermal desktop notifications enabled in their GUI settings (#313).
    pub fn are_thermal_alerts_enabled() -> bool {
        if let Some((_, _, _, _, home)) = Self::find_user_session_env() {
            for rel in [".config/omenspace/settings.json", ".config/omen-space/settings.json"] {
                let settings_path = format!("{}/{}", home, rel);
                if let Ok(content) = std::fs::read_to_string(&settings_path) {
                    if let Ok(json) = serde_json::from_str::<serde_json::Value>(&content) {
                        if let Some(enabled) = json.get("thermal_alerts").and_then(|v| v.as_bool()) {
                            return enabled;
                        }
                    }
                }
            }
        }
        true
    }

    fn send_via_user_session(title: &str, body: &str, urgency: u8, icon: &str, timeout_ms: i32) -> bool {
        let Some((user, display, dbus_addr, xdg_runtime, _)) = Self::find_user_session_env() else {
            return false;
        };
        let urgency_str = match urgency {
            0 => "low",
            2 => "critical",
            _ => "normal",
        };
        let mut cmd = Command::new("sudo");
        cmd.arg("-u").arg(&user).arg("env").arg(format!("DISPLAY={}", display));
        if !dbus_addr.is_empty() {
            cmd.arg(format!("DBUS_SESSION_BUS_ADDRESS={}", dbus_addr));
        }
        if !xdg_runtime.is_empty() {
            cmd.arg(format!("XDG_RUNTIME_DIR={}", xdg_runtime));
        }
        cmd.args([
            "notify-send",
            "-a",
            "OMENSpace",
            "-u",
            urgency_str,
            "-i",
            icon,
            "-t",
            &timeout_ms.to_string(),
        ]);
        if urgency < 2 {
            cmd.arg("--transient");
        }
        cmd.args([title, body]);
        match cmd.output() {
            Ok(out) if out.status.success() => true,
            Ok(out) => {
                warn!(
                    "notify-send in user session exited with {}: {}",
                    out.status,
                    String::from_utf8_lossy(&out.stderr).trim()
                );
                false
            }
            Err(e) => {
                warn!("Failed to spawn notify-send in user session: {}", e);
                false
            }
        }
    }

    pub async fn send_notification(title: &str, body: &str, urgency: u8) {
        Self::send_notification_tagged(None, title, body, urgency).await;
    }

    /// Like `send_notification`, but notices sharing the same `tag` replace each other
    /// instead of accumulating in the notification history.
    pub async fn send_notification_tagged(tag: Option<&str>, title: &str, body: &str, urgency: u8) {
        let replaces_id: u32 = tag
            .and_then(|t| NOTIFICATION_IDS.lock().ok().and_then(|g| g.as_ref().and_then(|m| m.get(t).copied())))
            .unwrap_or(0);
        info!("Sending desktop notification: {} - {}", title, body);
        if let Ok(connection) = Connection::session().await {
            let mut hints = std::collections::HashMap::new();
            hints.insert("urgency", zbus::zvariant::Value::U8(urgency));
            // Routine notices must not pile up in the notification history (#327).
            // Critical (urgency 2) notices stay persistent.
            if urgency < 2 {
                hints.insert("transient", zbus::zvariant::Value::Bool(true));
            }

            match connection.call_method(
                Some("org.freedesktop.Notifications"),
                "/org/freedesktop/Notifications",
                Some("org.freedesktop.Notifications"),
                "Notify",
                &(
                    "OMENSpace",
                    replaces_id,
                    "preferences-desktop-display",
                    title,
                    body,
                    Vec::<&str>::new(),
                    hints,
                    5000i32, // expire_timeout in ms
                ),
            ).await {
                Ok(reply) => {
                    if let (Some(t), Ok(id)) = (tag, reply.body().deserialize::<u32>()) {
                        if let Ok(mut g) = NOTIFICATION_IDS.lock() {
                            g.get_or_insert_with(Default::default).insert(t.to_string(), id);
                        }
                    }
                    return;
                }
                Err(e) => warn!("D-Bus Notify call failed on session bus: {}", e),
            }
        }
        if !Self::send_via_user_session(title, body, urgency, "preferences-desktop-display", 5000) {
            warn!("Could not deliver desktop notification via session bus or user session helper");
        }
    }

    /// Send an error notification (urgency=critical, icon=dialog-error, 10s timeout).
    /// Use for hardware command failures, permission errors, D-Bus method errors.
    pub async fn notify_error(context: &str, err_msg: &str) {
        let title = "OMEN Space — Hata";
        let body = format!("{}\n\nHata kodu: {}", context, err_msg);
        warn!("notify_error: {} | {}", context, err_msg);

        if let Ok(connection) = Connection::session().await {
            let mut hints = std::collections::HashMap::new();
            hints.insert("urgency", zbus::zvariant::Value::U8(2u8)); // critical

            match connection.call_method(
                Some("org.freedesktop.Notifications"),
                "/org/freedesktop/Notifications",
                Some("org.freedesktop.Notifications"),
                "Notify",
                &(
                    "OMEN Space",
                    0u32,
                    "dialog-error",
                    title,
                    body.as_str(),
                    Vec::<&str>::new(),
                    hints,
                    10000i32, // 10 seconds — errors should be readable
                ),
            ).await {
                Ok(_) => return,
                Err(e) => warn!("D-Bus Notify error call failed on session bus: {}", e),
            }
        }
        let _ = Self::send_via_user_session(title, &body, 2, "dialog-error", 10000);
    }


    /// Open path or URL in the logged in user's graphical desktop session
    pub fn open_in_user_session(target: &str) {
        info!("Opening target in user desktop session: {}", target);
        if let Ok(entries) = glob("/proc/*/environ") {
            for entry in entries.filter_map(Result::ok) {
                if let Ok(content) = std::fs::read(&entry) {
                    let env_str = String::from_utf8_lossy(&content);
                    if env_str.contains("DISPLAY=") || env_str.contains("WAYLAND_DISPLAY=") {
                        let mut display = ":0".to_string();
                        let mut user = String::new();
                        let mut dbus_addr = String::new();
                        let mut xdg_runtime = String::new();

                        for item in env_str.split('\0') {
                            if let Some((k, v)) = item.split_once('=') {
                                match k {
                                    "DISPLAY" => display = v.to_string(),
                                    "USER" | "LOGNAME" => user = v.to_string(),
                                    "DBUS_SESSION_BUS_ADDRESS" => dbus_addr = v.to_string(),
                                    "XDG_RUNTIME_DIR" => xdg_runtime = v.to_string(),
                                    _ => {}
                                }
                            }
                        }

                        // Sanitize username to prevent privilege escalation or command argument injection
                        let is_valid_username = !user.is_empty() 
                            && user != "root" 
                            && user.chars().all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-');

                        if is_valid_username {
                            let mut cmd = Command::new("sudo");
                            cmd.arg("-u").arg(&user).arg("env").arg(format!("DISPLAY={}", display));
                            if !dbus_addr.is_empty() {
                                cmd.arg(format!("DBUS_SESSION_BUS_ADDRESS={}", dbus_addr));
                            }
                            if !xdg_runtime.is_empty() {
                                cmd.arg(format!("XDG_RUNTIME_DIR={}", xdg_runtime));
                            }
                            cmd.args(["xdg-open", target]);
                            let _ = cmd.output();
                            break;
                        }
                    }
                }
            }
        }
    }

    /// Open browser directly to GitHub Issue creation page with pre-filled title and body
    pub fn open_github_issue(title: &str, body: &str) {
        let repo_url = "https://github.com/yunusemreyl/omen-space/issues/new";
        let encoded_title = url_encode(title);
        let encoded_body = url_encode(body);
        let full_url = format!("{}?title={}&body={}", repo_url, encoded_title, encoded_body);
        
        info!("Launching GitHub Issue creation URL: {}", full_url);
        Self::open_in_user_session(&full_url);
    }
}

fn url_encode(s: &str) -> String {
    let mut encoded = String::new();
    for b in s.bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                encoded.push(b as char);
            }
            b' ' => encoded.push_str("%20"),
            b'\n' => encoded.push_str("%0A"),
            _ => {
                encoded.push_str(&format!("%{:02X}", b));
            }
        }
    }
    encoded
}
