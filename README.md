<div align="center">

#  PawsMC Launcher

### A clean, debloated, and ad-free Minecraft launcher

[![Website: pawsmc.pawciol.eu](https://img.shields.io/badge/🌐_Website-pawsmc.pawciol.eu-76b9e8?style=for-the-badge&logo=googlechrome&logoColor=white)](https://pawsmc.pawciol.eu)
[![License: GPL-3.0](https://img.shields.io/badge/License-GPL--3.0-76b9e8?style=for-the-badge&logo=gnu&logoColor=white)](https://github.com/Pawciolszef/PawsMC/blob/main/LICENSE)
[![Built with AI Assistance](https://img.shields.io/badge/Developed-with%20AI%20Assistance-8a2be2?style=for-the-badge&logo=sparkles&logoColor=white)](https://github.com/Pawciolszef/PawsMC)
[![GitHub Release](https://img.shields.io/github/v/release/Pawciolszef/PawsMC?style=for-the-badge&color=76b9e8)](https://github.com/Pawciolszef/PawsMC/releases)
[![Build Status](https://img.shields.io/badge/Build-Passing-76b9e8?style=for-the-badge)](https://github.com/Pawciolszef/PawsMC)

<p align="center">
  <b>PawsMC</b> is a lightweight, privacy-respecting Minecraft launcher built for players who want a fast, beautiful, and distraction-free experience.
</p>

[**🌐 Visit Official Website**](https://pawsmc.pawciol.eu) • [**📥 Download Releases**](https://github.com/Pawciolszef/PawsMC/releases) • [**💬 Report an Issue**](https://github.com/Pawciolszef/PawsMC/issues)

---

</div>

> [!IMPORTANT]
> ### 📌 Fork Attribution & Transparency
> **PawsMC is an independent, open-source fork of the official [Modrinth App (Theseus)](https://github.com/modrinth/theseus / [modrinth/code](https://github.com/modrinth/code)).**  
> We have immense respect for the Modrinth team and their work. PawsMC builds upon the rock-solid foundation of the Modrinth App codebase while reshaping it into a fully debloated, ad-free, and community-first client.

> [!NOTE]
> ### 🤖 AI Collaboration Notice
> This project is **developed and maintained with AI assistance** (pair programming for refactoring, debloating, optimization, and feature engineering) in collaboration with human maintainers.

---

## ✨ What Makes PawsMC Different?

| Feature | Official Modrinth App | 🐾 PawsMC Launcher |
| :--- | :---: | :---: |
| **Advertisements** | Included (Sidebar/Popups) | 🚫 **Completely Removed** |
| **Telemetry & Analytics** | Enabled (PostHog / Sentry) | 🛡️ **Disabled & Stripped** |
| **Server Hosting Upsells** | Promoted in UI / Browse tab | 🧹 **Debloated & Cleaned** |
| **UI Clutter & Checklists** | Onboarding naggings / News feed | 💎 **Minimal & Focused** |
| **Accent Theme Palettes** | Single Green accent | 🎨 **4 Custom Accent Palettes** |
| **Discord Rich Presence** | Generic Modrinth App | 👾 **Custom PawsMC + Live Playtime** |
| **Quick Play & Gallery** | Not available | ⚡ **1-Click Launch & Screenshot Hub** |
| **Mod & Modpack Support** | Full Modrinth Ecosystem | ⚡ **100% Fully Compatible** |
| **License** | GNU GPL v3.0 | 📜 **[GNU GPL v3.0](LICENSE)** |

---

## 🚀 Key Features

- **🚫 100% Ad-Free Experience**: No ad windows, no sponsored blocks, no consent popups, no distraction.
- **⚡ Supercharged Performance**: Stripped unnecessary background telemetry and background network polls for snappy UI and faster instance loading.
- **🎨 Dynamic Accent Themes**: Switch effortlessly between 4 customizable accent color palettes:
  - 🩵 **Baby Blue** *(Default PawsMC)*
  - 🌸 **Sakura Pink** *(Pastel Pink)*
  - 💜 **Cyber Violet** *(Neon Lavender)*
  - 🌿 **Mint Green** *(Fresh Emerald)*
- **⚡ Quick Play Widget**: 1-click button in the sidebar to immediately launch or stop your most recently played instance.
- **🖼️ Quick Gallery & Screenshot Lightbox**:
  - Live thumbnail of your latest Minecraft screenshot in the sidebar.
  - 1-click copy image directly to system clipboard.
  - Theme-adaptive full-screen lightbox viewer with smooth zoom and controls.
- **📊 Playtime & Library Statistics**: Real-time stats bar on the home screen tracking total playtime across all instances, time played today, and installed instance counters.
- **👾 Enhanced Discord Rich Presence**: Shows PawsMC mascot branding, the exact modpack or instance you are playing, and live elapsed in-game playtime.
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
- Based on the open-source **[Modrinth Launcher (Theseus)](https://github.com/modrinth/code)** created by Rinth, Inc. and its open-source contributors.
