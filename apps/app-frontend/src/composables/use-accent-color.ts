import { computed, ref, watch } from 'vue'

export type StandardAccent = 'blue' | 'pink' | 'purple' | 'mint' | 'orange' | 'yellow' | 'red' | 'green' | 'custom'

export interface AccentOption {
	id: StandardAccent
	name: string
	emoji: string
	color: string
	previewBg: string
	border: string
}

export interface BgOption {
	id: string
	name: string
	emoji: string
	hex: string
	previewBg: string
}

export const ACCENT_OPTIONS: AccentOption[] = [
	{
		id: 'blue',
		name: 'Baby Blue',
		emoji: '🧊',
		color: '#38bdf8',
		previewBg: 'linear-gradient(135deg, #38bdf8 0%, #0284c7 100%)',
		border: '#38bdf8',
	},
	{
		id: 'pink',
		name: 'Sakura Pink',
		emoji: '🌸',
		color: '#f472b6',
		previewBg: 'linear-gradient(135deg, #f472b6 0%, #db2777 100%)',
		border: '#f472b6',
	},
	{
		id: 'purple',
		name: 'Cyber Violet',
		emoji: '🔮',
		color: '#c084fc',
		previewBg: 'linear-gradient(135deg, #c084fc 0%, #9333ea 100%)',
		border: '#c084fc',
	},
	{
		id: 'mint',
		name: 'Mint Green',
		emoji: '🍃',
		color: '#2dd4bf',
		previewBg: 'linear-gradient(135deg, #2dd4bf 0%, #0d9488 100%)',
		border: '#2dd4bf',
	},
	{
		id: 'orange',
		name: 'Sunset Orange',
		emoji: '🌅',
		color: '#fb923c',
		previewBg: 'linear-gradient(135deg, #fb923c 0%, #ea580c 100%)',
		border: '#fb923c',
	},
	{
		id: 'yellow',
		name: 'Golden Sun',
		emoji: '⚡',
		color: '#facc15',
		previewBg: 'linear-gradient(135deg, #facc15 0%, #ca8a04 100%)',
		border: '#facc15',
	},
	{
		id: 'red',
		name: 'Crimson Red',
		emoji: '🩸',
		color: '#f87171',
		previewBg: 'linear-gradient(135deg, #f87171 0%, #dc2626 100%)',
		border: '#f87171',
	},
	{
		id: 'green',
		name: 'Neon Emerald',
		emoji: '🌲',
		color: '#4ade80',
		previewBg: 'linear-gradient(135deg, #4ade80 0%, #16a34a 100%)',
		border: '#4ade80',
	},
	{
		id: 'custom',
		name: 'Custom RGB',
		emoji: '🎨',
		color: '#ec4899',
		previewBg: 'linear-gradient(135deg, #ec4899 0%, #8b5cf6 50%, #06b6d4 100%)',
		border: '#ec4899',
	},
]

export const BG_PRESETS: BgOption[] = [
	{
		id: 'default',
		name: 'Midnight Navy',
		emoji: '🌌',
		hex: '#090e17',
		previewBg: 'linear-gradient(135deg, #090e17 0%, #162238 100%)',
	},
	{
		id: 'oled',
		name: 'Pure OLED',
		emoji: '🖤',
		hex: '#000000',
		previewBg: 'linear-gradient(135deg, #000000 0%, #0c0c0c 100%)',
	},
	{
		id: 'cyber-purple',
		name: 'Cyber Abyss',
		emoji: '🔮',
		hex: '#0d0716',
		previewBg: 'linear-gradient(135deg, #0d0716 0%, #1d0f30 100%)',
	},
	{
		id: 'crimson-black',
		name: 'Blood Obsidian',
		emoji: '🩸',
		hex: '#120508',
		previewBg: 'linear-gradient(135deg, #120508 0%, #2a0b12 100%)',
	},
	{
		id: 'emerald-abyss',
		name: 'Emerald Matrix',
		emoji: '🌲',
		hex: '#04120a',
		previewBg: 'linear-gradient(135deg, #04120a 0%, #092816 100%)',
	},
	{
		id: 'slate',
		name: 'Deep Slate',
		emoji: '⛰️',
		hex: '#0f172a',
		previewBg: 'linear-gradient(135deg, #0f172a 0%, #1e293b 100%)',
	},
	{
		id: 'light',
		name: 'Clean Light',
		emoji: '☀️',
		hex: '#f8fafc',
		previewBg: 'linear-gradient(135deg, #f8fafc 0%, #e2e8f0 100%)',
	},
	{
		id: 'custom-bg',
		name: 'Custom Window RGB',
		emoji: '🎛️',
		hex: '#140c24',
		previewBg: 'linear-gradient(135deg, #140c24 0%, #2a1548 100%)',
	},
]

const ACCENT_KEY = 'pawsmc-accent-color'
const CUSTOM_HEX_KEY = 'pawsmc-custom-accent-hex'
const BG_THEME_KEY = 'pawsmc-bg-theme'
const CUSTOM_BG_HEX_KEY = 'pawsmc-custom-bg-hex'

export function hexToRgb(hex: string): { r: number; g: number; b: number } | null {
	if (!hex) return null
	const sanitized = hex.replace('#', '').trim()
	if (sanitized.length === 3) {
		const r = parseInt(sanitized[0] + sanitized[0], 16)
		const g = parseInt(sanitized[1] + sanitized[1], 16)
		const b = parseInt(sanitized[2] + sanitized[2], 16)
		if (isNaN(r) || isNaN(g) || isNaN(b)) return null
		return { r, g, b }
	}
	if (sanitized.length === 6) {
		const r = parseInt(sanitized.slice(0, 2), 16)
		const g = parseInt(sanitized.slice(2, 4), 16)
		const b = parseInt(sanitized.slice(4, 6), 16)
		if (isNaN(r) || isNaN(g) || isNaN(b)) return null
		return { r, g, b }
	}
	return null
}

export function rgbToHex(r: number, g: number, b: number): string {
	const clamp = (n: number) => Math.max(0, Math.min(255, Math.round(Number(n) || 0)))
	return (
		'#' +
		[clamp(r), clamp(g), clamp(b)]
			.map((x) => x.toString(16).padStart(2, '0'))
			.join('')
			.toLowerCase()
	)
}

function rgbToHsl(r: number, g: number, b: number): { h: number; s: number; l: number } {
	r /= 255
	g /= 255
	b /= 255
	const max = Math.max(r, g, b)
	const min = Math.min(r, g, b)
	let h = 0
	let s = 0
	const l = (max + min) / 2

	if (max !== min) {
		const d = max - min
		s = l > 0.5 ? d / (2 - max - min) : d / (max + min)
		switch (max) {
			case r:
				h = (g - b) / d + (g < b ? 6 : 0)
				break
			case g:
				h = (b - r) / d + 2
				break
			case b:
				h = (r - g) / d + 4
				break
		}
		h /= 6
	}

	return { h, s, l }
}

function hslToHex(h: number, s: number, l: number): string {
	l = Math.max(0, Math.min(1, l))
	s = Math.max(0, Math.min(1, s))

	const hue2rgb = (p: number, q: number, t: number) => {
		if (t < 0) t += 1
		if (t > 1) t -= 1
		if (t < 1 / 6) return p + (q - p) * 6 * t
		if (t < 1 / 2) return q
		if (t < 2 / 3) return p + (q - p) * (2 / 3 - t) * 6
		return p
	}

	let r: number, g: number, b: number
	if (s === 0) {
		r = g = b = l
	} else {
		const q = l < 0.5 ? l * (1 + s) : l + s - l * s
		const p = 2 * l - q
		r = hue2rgb(p, q, h + 1 / 3)
		g = hue2rgb(p, q, h)
		b = hue2rgb(p, q, h - 1 / 3)
	}

	return rgbToHex(Math.round(r * 255), Math.round(g * 255), Math.round(b * 255))
}

export function generateSurfacePalette(bgHex: string) {
	const rgb = hexToRgb(bgHex) || { r: 9, g: 14, b: 23 }
	const hsl = rgbToHsl(rgb.r, rgb.g, rgb.b)
	const isLight = hsl.l > 0.55

	if (bgHex.toLowerCase() === '#000000') {
		return {
			surface1: '#000000',
			surface1_5: '#04070c',
			surface2: '#080d17',
			surface2_5: '#0d1524',
			surface3: '#131e33',
			surface4: '#1a2944',
			surface5: '#273b60',
			isLight: false,
		}
	}

	const surface1 = bgHex
	const surface1_5 = hslToHex(hsl.h, hsl.s, isLight ? hsl.l - 0.03 : hsl.l + 0.03)
	const surface2 = hslToHex(hsl.h, hsl.s, isLight ? hsl.l - 0.06 : hsl.l + 0.06)
	const surface2_5 = hslToHex(hsl.h, hsl.s, isLight ? hsl.l - 0.09 : hsl.l + 0.1)
	const surface3 = hslToHex(hsl.h, hsl.s, isLight ? hsl.l - 0.13 : hsl.l + 0.14)
	const surface4 = hslToHex(hsl.h, hsl.s, isLight ? hsl.l - 0.18 : hsl.l + 0.2)
	const surface5 = hslToHex(hsl.h, hsl.s, isLight ? hsl.l - 0.25 : hsl.l + 0.28)

	return {
		surface1,
		surface1_5,
		surface2,
		surface2_5,
		surface3,
		surface4,
		surface5,
		isLight,
	}
}

function loadSavedAccent(): StandardAccent {
	try {
		const stored = localStorage.getItem(ACCENT_KEY)
		if (stored && ACCENT_OPTIONS.some((opt) => opt.id === stored)) {
			return stored as StandardAccent
		}
	} catch {
		// ignored
	}
	return 'blue'
}

function loadSavedCustomHex(): string {
	try {
		const stored = localStorage.getItem(CUSTOM_HEX_KEY)
		if (stored && hexToRgb(stored)) {
			return stored
		}
	} catch {
		// ignored
	}
	return '#ec4899'
}

function loadSavedBgTheme(): string {
	try {
		const stored = localStorage.getItem(BG_THEME_KEY)
		if (stored && BG_PRESETS.some((opt) => opt.id === stored)) {
			return stored
		}
	} catch {
		// ignored
	}
	return 'default'
}

function loadSavedCustomBgHex(): string {
	try {
		const stored = localStorage.getItem(CUSTOM_BG_HEX_KEY)
		if (stored && hexToRgb(stored)) {
			return stored
		}
	} catch {
		// ignored
	}
	return '#140c24'
}

const savedAccent = ref<StandardAccent>(loadSavedAccent())
const savedCustomHex = ref<string>(loadSavedCustomHex())
const savedBgTheme = ref<string>(loadSavedBgTheme())
const savedCustomBgHex = ref<string>(loadSavedCustomBgHex())

const previewAccent = ref<StandardAccent | null>(null)
const previewCustomHex = ref<string | null>(null)
const previewBgTheme = ref<string | null>(null)
const previewCustomBgHex = ref<string | null>(null)

const activeAccent = computed<StandardAccent>(() => previewAccent.value ?? savedAccent.value)
const activeCustomHex = computed<string>(() => previewCustomHex.value ?? savedCustomHex.value)
const activeBgTheme = computed<string>(() => previewBgTheme.value ?? savedBgTheme.value)
const activeCustomBgHex = computed<string>(() => previewCustomBgHex.value ?? savedCustomBgHex.value)

function applyThemeToDOM(accentId: StandardAccent, customHex: string, bgThemeId: string, customBgHex: string) {
	if (typeof document === 'undefined') return
	const html = document.documentElement

	// 1. Resolve Background Color
	let bgHex = customBgHex || '#090e17'
	const bgPreset = BG_PRESETS.find((opt) => opt.id === bgThemeId)
	if (bgPreset && bgPreset.id !== 'custom-bg' && bgPreset.hex) {
		bgHex = bgPreset.hex
	}

	const surfaces = generateSurfacePalette(bgHex)

	// Apply Surfaces across entire launcher window
	html.style.setProperty('--surface-1', surfaces.surface1)
	html.style.setProperty('--surface-1-5', surfaces.surface1_5)
	html.style.setProperty('--surface-2', surfaces.surface2)
	html.style.setProperty('--surface-2-5', surfaces.surface2_5)
	html.style.setProperty('--surface-3', surfaces.surface3)
	html.style.setProperty('--surface-4', surfaces.surface4)
	html.style.setProperty('--surface-5', surfaces.surface5)

	html.style.setProperty('--color-bg', surfaces.surface1)
	html.style.setProperty('--color-raised-bg', surfaces.surface3)
	html.style.setProperty('--color-super-raised-bg', surfaces.surface4)
	html.style.setProperty('--color-button-bg', surfaces.surface4)
	html.style.setProperty('--color-scrollbar', surfaces.surface5)
	html.style.setProperty('--color-divider', surfaces.surface4)

	// 2. Resolve Accent Color
	let accentHex = customHex || '#38bdf8'
	const preset = ACCENT_OPTIONS.find((opt) => opt.id === accentId)
	if (preset && preset.id !== 'custom' && preset.color) {
		accentHex = preset.color
	}

	const rgb = hexToRgb(accentHex) || { r: 56, g: 189, b: 248 }
	const { r, g, b } = rgb

	const luminance = (0.299 * r + 0.587 * g + 0.114 * b) / 255
	const contrastText = luminance > 0.6 ? '#0f172a' : '#ffffff'

	// Remove old classes
	for (const opt of ACCENT_OPTIONS) {
		html.classList.remove(`accent-${opt.id}`)
	}
	html.classList.add(`accent-${accentId}`)

	// Apply Accent Variables
	html.style.setProperty('--color-brand', accentHex)
	html.style.setProperty('--color-brand-highlight', `rgba(${r}, ${g}, ${b}, 0.22)`)
	html.style.setProperty('--color-brand-shadow', `rgba(${r}, ${g}, ${b}, 0.7)`)
	html.style.setProperty(
		'--brand-gradient-bg',
		`linear-gradient(0deg, rgba(${Math.round(r * 0.2)}, ${Math.round(g * 0.2)}, ${Math.round(b * 0.2)}, 0.35) 0%, rgba(${r}, ${g}, ${b}, 0.15) 100%)`,
	)
	html.style.setProperty(
		'--brand-gradient-strong-bg',
		`linear-gradient(270deg, rgba(${Math.round(r * 0.05)}, ${Math.round(g * 0.05)}, ${Math.round(b * 0.05)}, 1) 10%, rgba(${Math.round(r * 0.12)}, ${Math.round(g * 0.12)}, ${Math.round(b * 0.12)}, 1) 100%)`,
	)
	html.style.setProperty('--brand-gradient-border', `rgba(${r}, ${g}, ${b}, 0.25)`)
	html.style.setProperty(
		'--brand-gradient-fade-out-color',
		`linear-gradient(to bottom, rgba(${Math.round(r * 0.05)}, ${Math.round(g * 0.05)}, ${Math.round(b * 0.05)}, 0), rgba(${Math.round(r * 0.05)}, ${Math.round(g * 0.05)}, ${Math.round(b * 0.05)}, 1) 80%)`,
	)
	html.style.setProperty(
		'--loading-bar-gradient',
		`linear-gradient(to right, ${accentHex} 0%, rgba(${r}, ${g}, ${b}, 0.5) 100%)`,
	)
	html.style.setProperty('--color-button-bg-selected', `rgba(${r}, ${g}, ${b}, 0.22)`)
	html.style.setProperty('--color-button-text-selected', accentHex)
	html.style.setProperty('--color-accent-contrast', contrastText)
}

// Initial apply
if (typeof document !== 'undefined') {
	applyThemeToDOM(activeAccent.value, activeCustomHex.value, activeBgTheme.value, activeCustomBgHex.value)
}

watch(
	[activeAccent, activeCustomHex, activeBgTheme, activeCustomBgHex],
	([newAccent, newHex, newBg, newBgHex]) => {
		applyThemeToDOM(newAccent, newHex, newBg, newBgHex)
	},
	{ immediate: true },
)

export function useAccentColor() {
	return {
		savedAccent,
		savedCustomHex,
		savedBgTheme,
		savedCustomBgHex,
		previewAccent,
		previewCustomHex,
		previewBgTheme,
		previewCustomBgHex,
		activeAccent,
		activeCustomHex,
		activeBgTheme,
		activeCustomBgHex,
		accentOptions: ACCENT_OPTIONS,
		bgPresets: BG_PRESETS,
		setPreview: (
			accent: StandardAccent | null,
			customHex?: string | null,
			bgTheme?: string | null,
			customBgHex?: string | null,
		) => {
			previewAccent.value = accent
			if (customHex !== undefined && customHex !== null) {
				previewCustomHex.value = customHex
			} else if (accent === null) {
				previewCustomHex.value = null
			}

			previewBgTheme.value = bgTheme ?? null
			if (customBgHex !== undefined && customBgHex !== null) {
				previewCustomBgHex.value = customBgHex
			} else if (bgTheme === null) {
				previewCustomBgHex.value = null
			}
		},
		commit: (accent: StandardAccent, customHex?: string, bgTheme?: string, customBgHex?: string) => {
			savedAccent.value = accent
			if (customHex) savedCustomHex.value = customHex
			if (bgTheme) savedBgTheme.value = bgTheme
			if (customBgHex) savedCustomBgHex.value = customBgHex

			previewAccent.value = null
			previewCustomHex.value = null
			previewBgTheme.value = null
			previewCustomBgHex.value = null

			try {
				localStorage.setItem(ACCENT_KEY, accent)
				if (customHex) localStorage.setItem(CUSTOM_HEX_KEY, customHex)
				if (bgTheme) localStorage.setItem(BG_THEME_KEY, bgTheme)
				if (customBgHex) localStorage.setItem(CUSTOM_BG_HEX_KEY, customBgHex)
			} catch {
				// ignored
			}
			applyThemeToDOM(savedAccent.value, savedCustomHex.value, savedBgTheme.value, savedCustomBgHex.value)
		},
	}
}