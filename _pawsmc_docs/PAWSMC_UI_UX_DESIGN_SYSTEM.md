# 🎨 PawsMC UI/UX Design System Specification

A modern, high-tier visual design system tailored for Minecraft gamers and power users. Designed with a **Sleek Cyber-Dark / Glassmorphic Aesthetic**, glowing neon accents, tactile micro-interactions, and ultra-smooth animations.

---

## 1. Design Philosophy

1. **Gamer-Centric & Modern**: High contrast, true dark mode, glowing accents, rich hero imagery for Minecraft worlds, and clean typography.
2. **Tactile Feedback**: Hover elevation, subtle borders that glow on active focus, smooth state transitions (`cubic-bezier(0.16, 1, 0.3, 1)`).
3. **Information Density with Breathing Room**: Power users get compact lists, while casual players enjoy rich grid cards with animated banners and quick-play buttons.
4. **Fluid Glassmorphism**: Frosted background panels (`backdrop-filter: blur(20px)`) layered over subtle dark gradients.

---

## 2. Color Palette & Design Tokens (`theme.css`)

```css
:root {
  /* Brand Accents */
  --paws-primary: #7C3AED;         /* Deep Paws Purple */
  --paws-primary-hover: #9055FF;   /* Bright Violet */
  --paws-primary-glow: rgba(124, 58, 237, 0.4);

  --paws-cyan: #06B6D4;            /* Cyber Cyan */
  --paws-cyan-glow: rgba(6, 182, 212, 0.35);
  
  --paws-emerald: #10B981;         /* Fabric / Success Green */
  --paws-amber: #F59E0B;           /* Forge / Warning Orange */
  --paws-rose: #F43F5E;            /* Error / NeoForge Pink */

  /* Dark Theme Surfaces */
  --bg-app: #090A10;               /* OLED Base Background */
  --bg-surface: #11131F;           /* Elevated Panels & Sidebar */
  --bg-card: #181B2B;              /* Interactive Cards */
  --bg-card-hover: #22263D;        /* Hover Card State */
  --bg-input: #0D0F18;             /* Text Inputs & Selectors */

  /* Glassmorphism Overlays */
  --glass-panel: rgba(17, 19, 31, 0.75);
  --glass-border: rgba(255, 255, 255, 0.08);
  --glass-border-focus: rgba(124, 58, 237, 0.6);

  /* Typography Colors */
  --text-primary: #F8FAFC;         /* High-contrast Titles */
  --text-secondary: #94A3B8;       /* Body / Descriptions */
  --text-muted: #64748B;           /* Version tags / Timestamps */
  --text-accent: #C4B5FD;          /* Light Lavender Highlights */

  /* Geometry & Shadows */
  --radius-sm: 8px;
  --radius-md: 12px;
  --radius-lg: 18px;
  --radius-full: 9999px;

  --shadow-sm: 0 2px 8px rgba(0, 0, 0, 0.4);
  --shadow-glow: 0 0 24px var(--paws-primary-glow);
  --shadow-cyan-glow: 0 0 24px var(--paws-cyan-glow);

  /* Transitions */
  --ease-spring: cubic-bezier(0.16, 1, 0.3, 1);
  --transition-fast: 150ms var(--ease-spring);
  --transition-normal: 250ms var(--ease-spring);
}
```

---

## 3. Core Component Designs

### 1. Custom Titlebar (`PawsTitlebar.vue`)
- **Height**: `44px`
- **Left**:
  - PawsMC Logo SVG (purple glowing paw emblem with subtle hover pulse).
  - Breadcrumb / View title (e.g. `PawsMC / Library / Prominence II RPG`).
- **Center**:
  - Global Search Input: Pill-shaped with magnifying glass icon and keyboard shortcut badge `Ctrl + K`.
- **Right**:
  - Active Minecraft Profile Chip: 3D head render (from Crafatar/Minotar) + Player Username + Switch Account arrow.
  - Window Controls: Minimalist Minimize, Maximize, Close buttons with red hover on close.

### 2. Modern Navigation Sidebar (`PawsSidebar.vue`)
- **Layout**: Slim icon + label vertical bar with smooth collapse option.
- **Nav Buttons**:
  - Hover: Background tint with glowing left border indicator.
  - Active: Filled gradient background (`linear-gradient(135deg, rgba(124, 58, 237, 0.25), rgba(6, 182, 212, 0.1))`) with glowing icon.
  - Badges: Animated badge showing active downloading mod count.
- **Bottom Section**:
  - Mini System Monitor: Circular progress bar displaying launcher RAM allocation.

### 3. Instance Hero Card (`InstanceCard.vue`)
- **Visuals**:
  - Dynamic Minecraft banner wallpaper with bottom-to-top gradient fade.
  - Floating version pill: `1.21.1` (Gray) + `Fabric 0.16` (Emerald Green) or `Forge` (Amber).
  - Mod count pill: `📦 142 mods`.
  - Last played timestamp and total hours played tracker (`⏱️ 48.5 hrs`).
- **Play Action Button**:
  - Large glowing primary button:
    - Normal: Gradient `linear-gradient(135deg, #7C3AED, #6366F1)` with text `"PLAY"`.
    - Loading/Launching: Animated shimmer bar with text `"PREPARING..."`.
    - Running: Cyan pulsing badge `"RUNNING (PID: 12044)"` with `"KILL"` / `"LOGS"` actions.

### 4. Marketplace Explorer (`MarketplaceView.vue`)
- **Search Header**:
  - Sticky glass header with search input, dropdowns for Game Version (`1.21`, `1.20.1`, etc.), Loader (`Fabric`, `Forge`, `NeoForge`), and Sort filter.
  - Quick Category Tags: horizontal scrolling pills with icons (`⚔️ Adventure`, `⚡ Performance`, `🔮 Magic`, `🛠️ Tech`, `✨ Shaders`).
- **Project Card (`ProjectCard.vue`)**:
  - Project Icon with rounded corners and subtle border.
  - Download count (`1.2M`) + Follower count.
  - Author with verified checkmark.
  - Two buttons:
    - `"View Details"` (Opens sliding drawer).
    - `"Install"` (Dropdown list of compatible instances to inject the mod in 1 click).

### 5. Instance Detail View (`InstanceDashboard.vue`)
- **Tabs**:
  1. 📦 **Mods**: Searchable list with instant toggle switch, drag & drop zone for external `.jar` files, and "Update All" button.
  2. 🎨 **Resource Packs & Shaders**: Visual grid preview of shaders with active checkmarks.
  3. 🗺️ **Worlds & Saves**: World size, last modified, "Export Backup (.zip)" and "Open World Folder".
  4. 📸 **Screenshots**: High-res gallery with 1-click "Open Fullscreen" and "Copy to Clipboard".
  5. 📟 **Live Console**: Monospace code terminal with colored tags (`[INFO]`, `[WARN]`, `[ERROR]`), sticky auto-scroll toggle, search filter, and "Upload to Pastebin / Export" button.
  6. ⚙️ **Settings**: Memory slider (e.g. `2 GB` to `16 GB` with colored safety zones), Java path picker, custom JVM launch flags.

---

## 4. Typography Hierarchy

| Element | Font Family | Size | Weight | Line Height | Color |
| :--- | :--- | :--- | :--- | :--- | :--- |
| **Page Header (H1)** | `Outfit`, sans-serif | `28px` | `700 (Bold)` | `1.2` | `#F8FAFC` |
| **Section Title (H2)**| `Outfit`, sans-serif | `20px` | `600 (SemiBold)`| `1.3` | `#F1F5F9` |
| **Card Title (H3)** | `Inter`, sans-serif | `15px` | `600 (SemiBold)`| `1.4` | `#F8FAFC` |
| **Body Regular** | `Inter`, sans-serif | `13px` | `400 (Regular)` | `1.5` | `#94A3B8` |
| **Pill / Badge Text** | `Inter`, sans-serif | `11px` | `600 (SemiBold)`| `1.0` | `#E2E8F0` |
| **Code / Console** | `JetBrains Mono` | `12px` | `500 (Medium)` | `1.6` | `#A7F3D0` |

---

## 5. Micro-Interactions & Animations

1. **Button Hover**: Scales by `1.02` with an elevated box-shadow glow.
2. **Card Hover**: Subtle `translateY(-3px)` with a glowing border transition from `rgba(255,255,255,0.08)` to `var(--paws-primary-glow)`.
3. **Modal Transition**: Backdrop fades in (`opacity: 0 -> 1`), modal content scales in smoothly (`scale: 0.95 -> 1.0`).
4. **Toast Notifications**: Slide in from top-right corner with a countdown progress bar.
