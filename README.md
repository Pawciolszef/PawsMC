<div align="center">

# 🐾 PawsMC Launcher

### A clean, debloated, and ad-free Minecraft launcher

[![GitHub License](https://img.shields.io/github/license/Pawciolszef/PawsMC?style=for-the-badge&color=76b9e8)](LICENSE)
[![GitHub Release](https://img.shields.io/github/v/release/Pawciolszef/PawsMC?style=for-the-badge&color=76b9e8)](https://github.com/Pawciolszef/PawsMC/releases)
[![Build Status](https://img.shields.io/badge/Build-Passing-76b9e8?style=for-the-badge)](https://github.com/Pawciolszef/PawsMC)

<p align="center">
  <b>PawsMC</b> is a lightweight, privacy-respecting Minecraft launcher built for players who want a fast, beautiful, and distraction-free experience.
</p>

---

</div>

> [!IMPORTANT]
> ### 📌 Fork Attribution & Transparency
> **PawsMC is an independent, open-source fork of the official [Modrinth App (Theseus)](https://github.com/modrinth/theseus / [modrinth/code](https://github.com/modrinth/code)).**  
> We have immense respect for the Modrinth team and their work. PawsMC builds upon the rock-solid foundation of the Modrinth App codebase while reshaping it into a fully debloated, ad-free, and community-first client.

---

## ✨ What Makes PawsMC Different?

| Feature | Official Modrinth App | 🐾 PawsMC Launcher |
| :--- | :---: | :---: |
| **Advertisements** | Included (Sidebar/Popups) | 🚫 **Completely Removed** |
| **Telemetry & Analytics** | Enabled (PostHog / Sentry) | 🛡️ **Disabled & Stripped** |
| **Server Hosting Upsells** | Promoted in UI | 🧹 **Debloated & Cleaned** |
| **UI Clutter & Checklists** | Onboarding naggings / News feed | 💎 **Minimal & Focused** |
| **Theme & Aesthetic** | Standard Green/Dark | 🌸 **Baby Blue Pastel Palette** |
| **Mod & Modpack Support** | Full Modrinth Ecosystem | ⚡ **100% Fully Compatible** |
| **License** | GNU GPL v3.0 | 📜 **GNU GPL v3.0** |

---

## 🚀 Key Features

- **🚫 100% Ad-Free Experience**: No ad windows, no sponsored blocks, no consent popups, no distraction.
- **⚡ Supercharged Performance**: Stripped unnecessary background telemetry and background network polls for snappy UI and faster instance loading.
- **🎨 Custom Aesthetic**: Clean Pastel Baby Blue theme, custom branding, and a streamlined sidebar that keeps your Minecraft instances front and center.
- **📦 Full Ecosystem Compatibility**:
  - Install and update any Modrinth modpack, mod, resource pack, shader, or datapack with one click.
  - Multi-version support for **Fabric, NeoForge, Forge, and Quilt**.
  - Integrated Minecraft Java Edition account management and skin changer.
  - Quick instance switcher and isolated game profiles.

---

## 🛠️ Building From Source

### Prerequisites

Ensure you have the following installed:
- [Node.js](https://nodejs.org/) (v20+ or v22+)
- [pnpm](https://pnpm.io/) (v10+)
- [Rust & Cargo](https://rustup.rs/) (latest stable)
- [C++ Build Tools for Windows](https://visualstudio.microsoft.com/visual-cpp-build-tools/) / standard Linux build essentials

### Clone and Setup

```bash
# Clone the repository
git clone https://github.com/Pawciolszef/PawsMC.git
cd PawsMC

# Checkout development branch
git checkout dev

# Install all dependencies
pnpm install
```

### Run in Development Mode

```powershell
# Run the desktop app with hot reload
pnpm app:dev
```

### Build Production Binary

```powershell
# Build the production executable
pnpm app:build
```

---

## 🤝 Contributing

Contributions, bug reports, and suggestions are warmly welcome!
- Please submit all pull requests against the **`dev`** branch.
- Keep the code clean, fast, and free of trackers or bloatware.

---

## 📜 License & Acknowledgments

- **PawsMC** is licensed under the [GNU General Public License v3.0 (GPL-3.0)](LICENSE).
- Based on the open-source **[Modrinth Launcher (Theseus)](https://github.com/modrinth/code)** created by Rinth, Inc. and its wonderful open-source contributors.
