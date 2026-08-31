import { computed, reactive, ref, watch } from 'vue'

export type StandardAccent = 'blue' | 'pink' | 'purple' | 'mint' | 'orange' | 'yellow' | 'red' | 'green' | 'custom'

export interface AccentOption {
	id: StandardAccent
	name: string
	emoji: string
	color: string
	previewBg: string
	border: string
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

const ACCENT_KEY = 'pawsmc-accent-color'
const CUSTOM_HEX_KEY = 'pawsmc-custom-accent-hex'

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

const savedAccent = ref<StandardAccent>(loadSavedAccent())
const savedCustomHex = ref<string>(loadSavedCustomHex())
const previewAccent = ref<StandardAccent | null>(null)
const previewCustomHex = ref<string | null>(null)

const activeAccent = computed<StandardAccent>(() => previewAccent.value ?? savedAccent.value)
const activeCustomHex = computed<string>(() => previewCustomHex.value ?? savedCustomHex.value)

function applyAccentToDOM(accentId: StandardAccent, customHex: string) {
	if (typeof document === 'undefined') return
	const html = document.documentElement

	// Remove all old classes
	for (const opt of ACCENT_OPTIONS) {
		html.classList.remove(`accent-${opt.id}`)
	}
	html.classList.add(`accent-${accentId}`)

	let colorHex = customHex || '#38bdf8'
	const preset = ACCENT_OPTIONS.find((opt) => opt.id === accentId)
	if (preset && preset.id !== 'custom' && preset.color) {
		colorHex = preset.color
	}

	const rgb = hexToRgb(colorHex) || { r: 56, g: 189, b: 248 }
	const { r, g, b } = rgb

	// Compute luminance for contrast
	const luminance = (0.299 * r + 0.587 * g + 0.114 * b) / 255
	const contrastText = luminance > 0.6 ? '#0f172a' : '#ffffff'

	// Set dynamic CSS variables on documentElement so that 100% of UI elements pick up the color
	html.style.setProperty('--color-brand', colorHex)
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
		`linear-gradient(to right, ${colorHex} 0%, rgba(${r}, ${g}, ${b}, 0.5) 100%)`,
	)
	html.style.setProperty('--color-button-bg-selected', `rgba(${r}, ${g}, ${b}, 0.22)`)
	html.style.setProperty('--color-button-text-selected', colorHex)
	html.style.setProperty('--color-accent-contrast', contrastText)
}

// Initial apply
if (typeof document !== 'undefined') {
	applyAccentToDOM(activeAccent.value, activeCustomHex.value)
}

watch(
	[activeAccent, activeCustomHex],
	([newAccent, newHex]) => {
		applyAccentToDOM(newAccent, newHex)
	},
	{ immediate: true },
)

export function useAccentColor() {
	return {
		savedAccent,
		savedCustomHex,
		previewAccent,
		previewCustomHex,
		activeAccent,
		activeCustomHex,
		accentOptions: ACCENT_OPTIONS,
		setPreview: (accent: StandardAccent | null, customHex?: string | null) => {
			previewAccent.value = accent
			if (customHex !== undefined && customHex !== null) {
				previewCustomHex.value = customHex
			} else {
				previewCustomHex.value = null
			}
		},
		commit: (accent: StandardAccent, customHex?: string) => {
			savedAccent.value = accent
			if (customHex) {
				savedCustomHex.value = customHex
			}
			previewAccent.value = null
			previewCustomHex.value = null
			try {
				localStorage.setItem(ACCENT_KEY, accent)
				if (customHex) {
					localStorage.setItem(CUSTOM_HEX_KEY, customHex)
				}
			} catch {
				// ignored
			}
			applyAccentToDOM(savedAccent.value, savedCustomHex.value)
		},
	}
}