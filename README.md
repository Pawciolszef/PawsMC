# PawsMC

PawsMC is a lightweight, ad-free Minecraft launcher built with Tauri v2 and Rust, forked from Modrinth's Theseus client. It provides direct support for Modrinth, Feed The Beast (FTB), and CurseForge modpacks without telemetry or promotional clutter.

[![Website](https://img.shields.io/badge/Website-pawsmc.pawciol.eu-2563eb)](https://pawsmc.pawciol.eu)
[![Release](https://img.shields.io/github/v/release/Pawciolszef/PawsMC?color=2563eb)](https://github.com/Pawciolszef/PawsMC/releases)
[![License: GPL-3.0](https://img.shields.io/badge/License-GPL--3.0-blue.svg)](LICENSE)
[![Platform](https://img.shields.io/badge/Platform-Windows%20%7C%20Linux-lightgrey)](https://github.com/Pawciolszef/PawsMC/releases)

---

## Overview

PawsMC is an independent fork of the open-source [Modrinth App (Theseus)](https://github.com/modrinth/theseus). It maintains full compatibility with the Modrinth content ecosystem while removing tracking and promotional components, and expanding launcher capabilities to other major modpack repositories.

### Key Changes from Upstream

- **Extended Modpack Providers**: Native search, filtering, and installation for Feed The Beast (FTB) modpacks (including CurseForge-hosted FTB packs such as FTB Skies 2) alongside standard Modrinth and CurseForge content.
- **Removed Telemetry & Ads**: Analytics services (PostHog, Sentry) and server hosting promotions have been stripped out.
- **Theme Customization**: Additional UI color themes (OLED Black, Charcoal Gray, Forest Green) and custom accent palettes with real-time switching.
- **Interface Improvements**: Quick Play button for recently played instances, local screenshot preview widget with lightbox, and enhanced Discord Rich Presence.

---

## Features

- **Multi-Source Modpacks**: Browse and install packs from Modrinth, FTB, and CurseForge directly inside the app.
- **Mod Loaders**: Automatic setup and version resolution for Fabric, NeoForge, Forge, and Quilt, as well as Vanilla profiles.
- **Instance Isolation**: Separate directories, configurations, and worlds per instance to avoid mod conflicts.
- **Java Management**: Automatic detection and installation of required Java runtimes (Java 8 through 21+).
- **Account & Skin Manager**: Microsoft / Minecraft Java Edition authentication with in-launcher skin preview and switching.
- **Resource Management**: In-app installation and management of mods, resource packs, and shader packs.

---

## Installation

Pre-built binaries for Windows are available on the [Releases](https://github.com/Pawciolszef/PawsMC/releases) page and the [official website](https://pawsmc.pawciol.eu).

### Windows SmartScreen Note

Because PawsMC is a free, open-source project without an expensive commercial code-signing certificate, Windows Defender SmartScreen may display a warning on the first launch of the installer.

To proceed:
1. Click **More info**.
2. Click **Run anyway**.

All builds are compiled directly from source and can be verified independently.

---

## Building from Source

### Prerequisites

- [Node.js](https://nodejs.org/) (v20 or v22 LTS recommended)
- [pnpm](https://pnpm.io/) (v10+)
- [Rust & Cargo](https://rustup.rs/) (latest stable)
- Visual Studio C++ Build Tools (Windows) or standard build essentials (Linux)

### Build Steps

1. Clone the repository:
   ```bash
   git clone https://github.com/Pawciolszef/PawsMC.git
   cd PawsMC
   ```

2. Install dependencies:
   ```bash
   pnpm install
   ```

3. Run in development mode:
   ```bash
   pnpm --filter @modrinth/app dev
   ```

4. Build the production installer:
   ```bash
   pnpm --filter @modrinth/app build
   ```

---

## Project Status & Contributing

PawsMC is maintained as a personal open-source project. 

- **Bug Reports**: If you encounter an issue or an unexpected crash, please open an issue in the [GitHub Issues](https://github.com/Pawciolszef/PawsMC/issues) tracker with relevant logs.
- **Pull Requests**: Unsolicited feature pull requests are generally not accepted. If you want to customize or extend the launcher for your own use, feel free to fork the repository under the GPL-3.0 license.

---

## License & Credits

- PawsMC is licensed under the [GNU General Public License v3.0](LICENSE).
- Based on the open-source **Modrinth App (Theseus)** developed by Rinth, Inc. and community contributors.