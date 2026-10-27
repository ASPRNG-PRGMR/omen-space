use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=../../.git/HEAD");
    println!("cargo:rerun-if-changed=../../.git/refs/heads/main");

    let full_sha = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .ok()
        .filter(|o| o.status.success())
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| "unknown".to_string());

    let short_sha = if full_sha != "unknown" && full_sha.len() >= 7 {
        full_sha[..7].to_string()
    } else {
        "unknown".to_string()
    };

    println!("cargo:rustc-env=OMEN_GIT_SHA={}", full_sha);
    println!("cargo:rustc-env=OMEN_GIT_SHORT_SHA={}", short_sha);
}
