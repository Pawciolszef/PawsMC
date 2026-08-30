# 🐾 PawsMC - Master Prompt for Claude Opus

> **Usage Instructions:** Copy and paste the entire prompt below into Claude Opus (or your agent workflow powered by Claude Opus). It contains all context, technical constraints, architectural mappings, and step-by-step instructions needed to fork and completely redesign the Modrinth Launcher into **PawsMC**.

---

```markdown
# MISSION: Fork Modrinth App ("theseus") into "PawsMC" with a Complete UI/UX Overhaul

## 1. Executive Summary & Objective
You are an expert full-stack systems engineer specializing in **Rust**, **Tauri (v1/v2)**, **Vue 3 (Composition API, `<script setup lang="ts">`)**, **Pinia**, and **Modern UI/UX Design Systems**.

Your objective is to create **PawsMC**, a premium, high-performance, community-oriented Minecraft launcher based on the official open-source Modrinth launcher repository (`modrinth/theseus`).

### Core Requirements:
1. **Preserve 100% of Core Backend Capabilities:**
   - Microsoft / Mojang / Modrinth OAuth authentication.
   - Instance management (create, clone, export, import `.mrpack` & `.zip`, delete, backup).
   - Minecraft version installation (Vanilla, Fabric, Quilt, Forge, NeoForge).
   - Modrinth API and CurseForge API mod/pack/shader/resource pack browsing & one-click dependency installation.
   - Automatic Java runtime resolution (Adoptium / Zulu JDK download & allocation).
   - Game launch lifecycle, JVM arguments, RAM allocation, resolution, and real-time process log capture.
2. **Complete UI/UX Redesign:**
   - Replace the default Modrinth app interface with a custom, sleek, modern **PawsMC Design System** (Dark OLED / Deep Indigo aesthetics, glassmorphism, glowing accents, paw/cyber-themed micro-interactions, silky smooth transitions).
   - Redesign every major view: Window Shell/Titlebar, Dashboard/Instance Grid, Mod/Modpack Marketplace Explorer, Instance Detail View, Real-Time Crash/Console Log Analyzer, and Settings.
3. **Comprehensive Rebranding:**
   - App identifier: `com.pawsmc.launcher`
   - App Name: `PawsMC Launcher` / `PawsMC`
   - Data Directory: `%APPDATA%/PawsMC` (Windows), `~/.config/pawsmc` (Linux), `~/Library/Application Support/PawsMC` (macOS).
   - Remove Modrinth-specific branding and telemetry endpoints, while maintaining valid API client identification headers to Modrinth / Curseforge APIs.

---

## 2. Technical Stack & Repository Architecture

### Backend: Rust + Tauri
- **Workspace Crates**:
  - `theseus_gui` (or renamed `pawsmc_gui`): Main Tauri application binary, system tray, window management, and Tauri IPC command handlers (`src/commands/`).
  - `theseus_core` / `launcher_core`: Game installation, file integrity hashing (SHA1/SHA512), manifest parsing, Java discovery, process spawning.
  - `modrinth_api` / `curseforge_api`: API clients for querying mods, versions, authors, search indexes.
  - `auth`: Microsoft OAuth2 flow via loopback HTTP server + Mojang / Xbox Live token exchange.
- **Tauri IPC Command Contract**:
  - You **MUST NOT** break existing Tauri IPC signatures (`invoke('command_name', { ... })`).
  - If frontend requires new endpoints, extend Rust command handlers cleanly in the commands module and register them in `tauri::generate_handler![]`.

### Frontend: Vue 3 + TypeScript
- **Framework**: Vue 3 with `<script setup lang="ts">`.
- **State Management**: Pinia stores (`useInstanceStore`, `useAuthStore`, `useModrinthStore`, `useSettingsStore`, `useLogStore`).
- **Styling**: Modern CSS / TailwindCSS / UnoCSS with CSS variables for dynamic theming (Dark OLED, Midnight Paws, Neon Cyber, Cozy Forest).
- **Icons**: Lucide Icons (`lucide-vue-next`) + Custom PawsMC SVG iconography.
- **Routing**: `vue-router` for view transitions.

---

## 3. Step-by-Step Implementation Phases

### PHASE 1: Rebranding & Metadata Configuration
1. Update `tauri.conf.json`:
   - `package.productName` -> `"PawsMC"`
   - `tauri.bundle.identifier` -> `"com.pawsmc.launcher"`
   - `tauri.windows[0].title` -> `"PawsMC Launcher"`
   - Window properties: custom titlebar (`decorations: false` / `titleBarStyle: "Overlay"`), transparent/frosted background capabilities if supported.
2. Update `Cargo.toml` and crate manifests:
   - Rename root package to `pawsmc`.
   - Update authors, description, repository URLs.
3. Update `package.json`:
   - Name: `pawsmc-launcher`
4. Update App Data directories in Rust path resolvers (`dirs::data_dir().join("PawsMC")` or Tauri default app data path).
5. Replace brand assets (icons, logo SVGs, tray icons in `src-tauri/icons/` and `src/assets/`).

---

### PHASE 2: PawsMC Design System & Base Theme Engine
1. Create `src/assets/styles/theme.css` with a full CSS variable token design system:
   - Primary: `#7C3AED` (Paws Purple / Electric Violet)
   - Secondary / Accent: `#06B6D4` (Cyan Glow) / `#F43F5E` (Rose Paws)
   - Backgrounds: `#090A0F` (True OLED Dark), `#12141F` (Surface Base), `#1A1D2E` (Card Surface)
   - Borders: `rgba(255, 255, 255, 0.08)` / `rgba(124, 58, 237, 0.2)` on hover
   - Glassmorphism: `backdrop-filter: blur(16px); background: rgba(18, 20, 31, 0.75);`
   - Typography: Font family `Inter`, `Outfit` or system fonts, with `JetBrains Mono` for code/logs.
2. Implement Theme Switcher Store (Dark OLED, Nebula Purple, Emerald Forest, Cyber Glow).

---

### PHASE 3: App Shell, Custom Titlebar & Navigation
1. **Custom Frameless Titlebar**:
   - Integrated PawsMC animated logo & version badge.
   - Quick search input (`Ctrl + K` spotlight launcher).
   - Discord Rich Presence toggle status indicator.
   - Active user profile avatar chip with dropdown (switch account, skin preview, logout).
   - Window control buttons (Minimize, Maximize/Restore, Close) with native Tauri window API bindings.
2. **Left Navigation Sidebar**:
   - Modern floating or dock style sidebar.
   - Nav items:
     - 🏠 **Home / Instances** (Library of all installed MC instances).
     - 🧭 **Explore / Marketplace** (Modrinth + CurseForge mods, modpacks, shaders, resource packs).
     - ⚡ **Quick Play** (Instant launch for last played instance).
     - 📥 **Downloads / Tasks Queue** (Active downloading progress with speeds and pause/resume).
     - ⚙️ **Settings** (Global Java, memory, launcher graphics, themes, audio feedback).
   - Bottom status area: Total RAM usage indicator, connection status.

---

### PHASE 4: Home & Instances Dashboard Redesign
1. **Instances View Modes**:
   - **Rich Grid View**: Hero card for favorite/recent instance with animated banners, playtime tracker, Minecraft version chip (e.g., `1.21.1 Fabric`), mod count badge, and a glowing **"LAUNCH"** action button.
   - **Compact List View**: Sleek table layout for power users with many instances.
2. **Instance Creation Modal**:
   - Modern 3-step wizard:
     1. Choose type: Standard Vanilla / Modded (Fabric, Forge, Quilt, NeoForge) or Modpack.
     2. Select Minecraft & Loader versions (with auto-fetched latest release/snapshot chips).
     3. Allocate RAM & Custom Icon/Banner picker.
3. **One-Click Play Button**:
   - Dynamic states: `IDLE` (Launch), `DOWNLOADING` (Progress bar on button), `LAUNCHING` (Pulsing glow), `RUNNING` (Kill / Open Logs), `CRASHED` (View Crash Report).

---

### PHASE 5: Explore & Marketplace Redesign
1. **Unified Search & Filter Header**:
   - Search bar with instant autocomplete.
   - Category tags (Adventure, Magic, Tech, Performance, Optimization, PvP).
   - Minecraft Version filter dropdown + Loader filter pills (Fabric / NeoForge / Forge / Quilt).
   - Sort by: Relevance, Downloads, Latest Updated, Newest.
2. **Project Card Component**:
   - High-res icon, verified badge, download counter, author, short description.
   - Fast "Install to Instance" dropdown button allowing direct injection into any compatible instance without leaving the search view.
3. **Project Details Drawer / Page**:
   - Rich Markdown render of README/Description.
   - Gallery with lightbox screenshot viewer.
   - Version history list with changelogs and dependencies breakdown.

---

### PHASE 6: Instance Management & Details View
1. **Tabbed Instance Dashboard**:
   - **Overview**: Play button, playtime statistics, custom banner upload, notes, quick folder shortcuts (open `.minecraft`, open screenshots, open mods).
   - **Mods Manager**: Toggle active/inactive with switches, bulk update checker, drag-and-drop `.jar` files to install, delete, filter by enabled/disabled.
   - **Resource Packs & Shaders**: Visual list with toggle controls.
   - **Worlds & Saves**: Backup/restore world zip, seed viewer, open folder.
   - **Screenshots Gallery**: In-app image viewer with "Copy to clipboard" and "Share".
   - **Live Console / Logs**: Real-time streaming log with colored syntax highlighting (INFO in cyan, WARN in amber, ERROR in glowing red), regex search filter, "Auto-scroll" lock, and "Export Crash Log" button.
   - **Settings Override**: Per-instance Java version selector, min/max RAM slider, custom JVM arguments.

---

### PHASE 7: Launcher Settings & Polish
1. **General Settings**:
   - Default Java Path & RAM Allocation default presets.
   - Launcher behavior on game start (Keep open, Hide to tray, Close).
   - Custom background wallpaper & opacity slider.
   - Discord RPC toggle (Show currently playing instance, server IP, time elapsed).
2. **Accounts Manager**:
   - Multiple Microsoft account switching with 1-click active toggle.
   - Offline / Skin preview 3D render.

---

## 4. Coding Standards & Guarantees

- **No Placeholders**: Write full, working implementations. Do not leave `// TODO: implement later` or placeholder stubs.
- **Type Safety**: Strictly typed TypeScript with comprehensive interfaces matching Rust struct serializations.
- **State Integrity**: All backend state changes must flow through Pinia stores and handle asynchronous errors gracefully with user-friendly Toast notifications.
- **Responsive & Accessible**: Clean flex/grid layouts with full keyboard navigation support and accessible ARIA attributes.
- **IPC Safety**: Always wrap Tauri `invoke()` calls in `try/catch` with structured error reporting.

---

## 5. Execution Verification
When completing each phase:
1. Verify `tauri.conf.json` and Rust compilation (`cargo check`).
2. Verify TypeScript compilation (`vue-tsc --noEmit` or `pnpm build`).
3. Test IPC endpoints for instance creation, mod search, and launch hooks.
4. Ensure all styling tokens are scoped and properly inherited.
```
