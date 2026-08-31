import { computed, ref, watch } from 'vue'

export type AccentColor = 'blue' | 'pink' | 'purple' | 'mint'

export interface AccentOption {
	id: AccentColor
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
		emoji: '🩵',
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
		emoji: '💜',
		color: '#c084fc',
		previewBg: 'linear-gradient(135deg, #c084fc 0%, #9333ea 100%)',
		border: '#c084fc',
	},
	{
		id: 'mint',
		name: 'Mint Green',
		emoji: '🌿',
		color: '#2dd4bf',
		previewBg: 'linear-gradient(135deg, #2dd4bf 0%, #0d9488 100%)',
		border: '#2dd4bf',
	},
]

const ACCENT_KEY = 'pawsmc-accent-color'

function loadSavedAccent(): AccentColor {
	try {
		const stored = localStorage.getItem(ACCENT_KEY)
		if (stored && ['blue', 'pink', 'purple', 'mint'].includes(stored)) {
			return stored as AccentColor
		}
	} catch {
		// ignored
	}
	return 'blue'
}

const savedAccent = ref<AccentColor>(loadSavedAccent())
const previewAccent = ref<AccentColor | null>(null)

const activeAccent = computed<AccentColor>(() => previewAccent.value ?? savedAccent.value)

function applyAccentClass(accent: AccentColor) {
	if (typeof document === 'undefined') return
	const html = document.documentElement

	// Remove any inline surface variables that were previously set
	html.style.removeProperty('--surface-1')
	html.style.removeProperty('--surface-1-5')
	html.style.removeProperty('--surface-2')
	html.style.removeProperty('--surface-2-5')
	html.style.removeProperty('--surface-3')
	html.style.removeProperty('--surface-4')
	html.style.removeProperty('--surface-5')
	html.style.removeProperty('--color-bg')
	html.style.removeProperty('--color-raised-bg')
	html.style.removeProperty('--color-super-raised-bg')
	html.style.removeProperty('--color-button-bg')
	html.style.removeProperty('--color-scrollbar')
	html.style.removeProperty('--color-divider')
	html.style.removeProperty('--color-brand')
	html.style.removeProperty('--color-brand-highlight')
	html.style.removeProperty('--color-brand-shadow')
	html.style.removeProperty('--brand-gradient-bg')
	html.style.removeProperty('--brand-gradient-strong-bg')
	html.style.removeProperty('--brand-gradient-border')
	html.style.removeProperty('--brand-gradient-fade-out-color')
	html.style.removeProperty('--loading-bar-gradient')
	html.style.removeProperty('--color-button-bg-selected')
	html.style.removeProperty('--color-button-text-selected')
	html.style.removeProperty('--color-accent-contrast')

	html.classList.remove('accent-blue', 'accent-pink', 'accent-purple', 'accent-mint', 'accent-orange', 'accent-yellow', 'accent-red', 'accent-green', 'accent-custom')
	html.classList.add(`accent-${accent}`)
}

// Initial apply
if (typeof document !== 'undefined') {
	applyAccentClass(activeAccent.value)
}

watch(
	activeAccent,
	(val) => {
		applyAccentClass(val)
	},
	{ immediate: true },
)

export function useAccentColor() {
	return {
		savedAccent,
		previewAccent,
		activeAccent,
		accentOptions: ACCENT_OPTIONS,
		setPreview: (accent: AccentColor | null) => {
			previewAccent.value = accent
		},
		commit: (accent: AccentColor) => {
			savedAccent.value = accent
			previewAccent.value = null
			try {
				localStorage.setItem(ACCENT_KEY, accent)
			} catch {
				// ignored
			}
			applyAccentClass(savedAccent.value)
		},
	}
}