import { ref, watch } from 'vue'

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

const currentAccent = ref<AccentColor>(loadSavedAccent())

function applyAccentClass(accent: AccentColor) {
	if (typeof document === 'undefined') return
	const html = document.documentElement
	html.classList.remove('accent-blue', 'accent-pink', 'accent-purple', 'accent-mint')
	html.classList.add(`accent-${accent}`)
}

// Initial apply
if (typeof document !== 'undefined') {
	applyAccentClass(currentAccent.value)
}

watch(
	currentAccent,
	(val) => {
		try {
			localStorage.setItem(ACCENT_KEY, val)
		} catch {
			// ignored
		}
		applyAccentClass(val)
	},
	{ immediate: true },
)

export function useAccentColor() {
	return {
		currentAccent,
		accentOptions: ACCENT_OPTIONS,
		setAccent: (accent: AccentColor) => {
			currentAccent.value = accent
		},
	}
}
