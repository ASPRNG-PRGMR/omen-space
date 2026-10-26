<div align="center">
  <img src="images/omenspace.png" alt="OMEN Space Logo" width="120" />

  # OMEN Space

  **The ultimate, lightweight Linux control center for HP Omen, Victus & Transcend.**  
  *Written entirely in Rust for zero-overhead, native GTK4 performance.*

  [![Version](https://img.shields.io/badge/Release-v2.1.8-red.svg?style=flat-square)](https://github.com/yunusemreyl/omen-space/releases)
  [![License](https://img.shields.io/badge/License-GPL%203.0-green.svg?style=flat-square)](LICENSE)
  [![Platform](https://img.shields.io/badge/Platform-Linux-lightgrey.svg?style=flat-square)]()
  [![Built with Rust](https://img.shields.io/badge/Language-Rust-orange.svg?style=flat-square)]()

  **English** · [Español](README.es.md)
</div>

> **🎉 Special Thanks:** A huge shoutout to **[@aloshy0](https://github.com/aloshy0)** for the incredible "Quick HUD Overlay" PR! Your architectural design and contributions make OMEN Space the ultimate Linux gaming tool.

> [!IMPORTANT]
> **Status — moving to v2.1.8.** The published v2.1.8 release is **being refreshed** and will be updated once the current round of testing is finished. Until then, the fixes and features listed below only exist on the `main` branch.
> **We recommend that everyone installs the bleeding-edge (canary) build** to get the v2.1.8 fixes — see [Quick Install](#-quick-install). Please report anything odd in the [issue tracker](https://github.com/yunusemreyl/omen-space/issues).

---

## ✨ Features

OMEN Space provides everything you need to unlock the full potential of your laptop on Linux, without the bloat.

- 🎛️ **Fan & Thermal Mastery:** Create custom fan curve splines for near-silent operation without thermal throttling. Includes a dedicated **Fan Cleaning Mode** and **Automatic Fan-Stall Detection** (emergency Max Fan recovery & desktop alert if fans stall under high temperatures).
- 🎮 **Quick HUD Overlay (Shift+F2):** A floating GTK4 panel for switching fan and power modes with live CPU/GPU/Fan/RAM telemetry without opening the main app. It needs `omen-tray` running and opens as a normal window, so on Wayland it can sit behind a fullscreen game (see [Known Issues](knownissues.md)).
- ⚡ **Performance Profiles:** Seamlessly switch between `power-saver`, `balanced`, and `performance` ACPI/WMI modes with active EC keep-alive protection and automatic WMI `WMBA` → `WMAA` fallback across OMEN 15/16/17, OMEN Max 16, Transcend 14, and Victus 15/16 series.
- 🚀 **Intel RAPL, Ryzen SMU & Undervolting:** Intel RAPL PL1/PL2 power limits (up to 150W/200W for HX & Core Ultra processors), direct MSR-based undervolting, TCC offset control, GPU TGP limits, and AMD Ryzen SMU tuning.
- 🎮 **MUX Switch:** Native Optimus / dGPU routing switching for maximum gaming performance.
- 🔍 **Scalable Interface (Ctrl+Shift+Plus / Ctrl+Shift+Minus):** Zoom the whole window between 80 % and 200 % (`Ctrl+Shift+0` resets) so the app stays readable on large or high-DPI monitors without going fullscreen. The factor is remembered and can also be set from Settings → Appearance.
- 🔋 **Battery Telemetry:** Battery charge and health are shown in System Monitor, the Quick HUD and the tray.
- 🌈 **RGB Studio:** Hardware-accelerated 4-Zone, Single-Zone, Light Bar, and USB HID Per-Key (`0d62:54bf`) keyboard lighting with wave, breathing, cycle, and static effects. On Darfon per-key boards an **experimental *Hardware effects* switch** lets the keyboard's own MCU run rainbow, cycle, breathing, starlight and raindrop with near-zero CPU use (the MCU ignores the brightness slider for these; see [#322](https://github.com/yunusemreyl/omen-space/issues/322)).
- 🔔 **Calmer Thermal Alerts:** Desktop notifications are transient (no more pile-up in your notification history), replace each other, and only appear when a hot condition lasts a few seconds. Fan protection itself still reacts instantly.
- 🔄 **Smart BIOS Checker:** Automatically checks HP servers for your specific motherboard's latest firmware.

---

## ⚡ Quick Install

The easiest way to install OMEN Space is via our 1-line web installer. It detects your distro, handles dependencies, and compiles everything automatically.

**Recommended for now — bleeding-edge (canary), includes all v2.1.8 fixes:**
```bash
curl -sSL https://raw.githubusercontent.com/yunusemreyl/omen-space/main/install.sh | sudo bash -s -- --canary
```

**Stable release** (will be refreshed after testing; does *not* yet contain the latest v2.1.8 fixes):
```bash
curl -sSL https://raw.githubusercontent.com/yunusemreyl/omen-space/main/install.sh | sudo bash -s -- --stable
```

> **Note:** Running the installer without a flag asks you to choose a channel interactively.

<details>
<summary><b>📦 Alternative Installation Methods (Manual, AUR, NixOS)</b></summary>

**Via Git Clone (Ubuntu/Debian, Fedora, Arch, openSUSE):**
```bash
git clone https://github.com/yunusemreyl/omen-space.git
cd omen-space
sudo ./setup.sh install
```

**Arch Linux (AUR):**
```bash
git clone https://github.com/yunusemreyl/omen-space.git
cd omen-space
makepkg -si
```

**NixOS (Flakes):**
```bash
nix profile install github:yunusemreyl/omen-space
```

**Uninstall:**
If you installed via the quick install script or `setup.sh`:
```bash
cd omen-space
sudo ./setup.sh uninstall
```
*(If you deleted the folder, just run `git clone https://github.com/yunusemreyl/omen-space.git` again first).*
</details>

---

## 🆕 What's New in v2.1.8 (on `main`)

- **Fixed:** GUI closing itself every ~30 seconds (heartbeat crash) and related Quick Access panel desync.
- **Fixed:** False overheating alerts from brief CPU spikes; thermal notices are transient, debounced and replaceable.
- **Fixed:** System Monitor total power no longer counts GPU "sleeping/idle" markers; GPU load is reported; an unavailable GPU temperature shows `--` instead of a made-up value; FAN label now shows *Custom* and *EC* correctly.
- **New:** Battery charge/health telemetry, UI scaling shortcuts, experimental MCU hardware effects for Darfon per-key keyboards.

## 🧪 Help Wanted

- **OMEN MAX 16 owners (8D41 / 8D87):** please try *Hardware effects* in RGB Studio and tell us how it behaves on [#322](https://github.com/yunusemreyl/omen-space/issues/322).
- **Board verification:** open a *"Verify my laptop"* issue with the diagnostics report and your `board_name` to get your laptop added to the verified list.
- Still open: independent CPU/GPU fan curves, GPU idle-state reporting during short workloads, and a few items tracked in [#316](https://github.com/yunusemreyl/omen-space/issues/316).

---

## 📸 Screenshots

<p align="center">
  <img src="images/perf.png" width="48%" alt="Performance & Fan Curve Editor" />
  <img src="images/appprofiles.png" width="48%" alt="App Profiles" />
</p>
<p align="center">
  <img src="images/keyboard.png" width="48%" alt="RGB Settings" />
  <img src="images/undervolt.png" width="48%" alt="Ryzen Undervolting" />
</p>

<details>
<summary><b>🔍 View More Screenshots (MUX, Diagnostics, Settings, Updater, CLI)</b></summary>
<br>
<p align="center">
  <img src="images/mux.png" width="48%" alt="MUX Switch" />
  <img src="images/diagnostic.png" width="48%" alt="Diagnostics" />
</p>
<p align="center">
  <img src="images/settings.png" width="48%" alt="Settings" />
  <img src="images/updater.png" width="48%" alt="Updater" />
</p>
<p align="center">
  <img src="images/cli.png" width="48%" alt="CLI" />
</p>
</details>

---

## 🏗️ Architecture

OMEN Space is a complete rewrite of the legacy Python *OmenCtl*, moving to **Rust** to achieve a ~3MB footprint, less than 5MB of RAM usage, and instant responsiveness.

- **`omen-space-daemon`**: The backend. Runs as a systemd service (root), managing WMI, ACPI, Sysfs, and MSR interactions over secure D-Bus.
- **`omen-gui`**: A beautifully fast GTK4 + Libadwaita frontend running in user-space.
- **`omen-overlay`**: The on-demand GTK4 Quick HUD Overlay (`Shift+F2`) for instant profile and fan mode switching.
- **`omen-tray`**: A lightweight desktop panel applet for quick profile toggling and hotkey handling.
- **`omen-cli`**: A fast scriptable terminal interface. Includes a HUD command (`omen-cli overlay toggle`).
- **`hp-omen-extra`**: The underlying DKMS kernel driver extending standard kernel capabilities.

---

## 🌐 Localization

The interface (`omen-gui`, `omen-overlay`, and `omen-tray`) is available in **English, Turkish (Türkçe), and Spanish (Español)**. Change the language anytime under *Settings → Appearance & Language* (or leave it on *Auto* to follow your system locale).

---

## 👨‍💻 Credits & License

OMEN Space is licensed under the **GPL-3.0 License** and is driven by an incredible community.

- **[yunusemreyl](https://github.com/yunusemreyl)** - Lead Developer
- **[tuxov](https://github.com/tuxov)** - Kernel Module Lead

Thanks to all our contributors: [@aloshy0](https://github.com/aloshy0), [@CodesRahul96](https://github.com/CodesRahul96), [@xcellsior](https://github.com/xcellsior), [@TitoTFP](https://github.com/TitoTFP), [@SafSaf0999](https://github.com/SafSaf0999), [@yijean34-source](https://github.com/yijean34-source), [@ManOfDiamond](https://github.com/ManOfDiamond), [@ASPRNG-PRGMR](https://github.com/ASPRNG-PRGMR), [@WoofahRayetCode](https://github.com/WoofahRayetCode), [@henfiber](https://github.com/henfiber) (detailed reviews), [@ktarxhun](https://github.com/ktarxhun), and the projects `omencore`, `omen-rgb-keyboard` & [`omen-rgb-linux`](https://github.com/arfelious/omen-rgb-linux) (Darfon MCU protocol).

*Disclaimer: OMEN Space is an independent project and is NOT affiliated with or endorsed by HP.*
