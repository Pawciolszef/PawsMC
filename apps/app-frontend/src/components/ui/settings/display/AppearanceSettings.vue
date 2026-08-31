<script setup lang="ts">
import {
	AppearanceSettingsLayout,
	injectAuth,
	injectUserPreferences,
	provideAppearanceSettings,
	useSavable,
} from '@modrinth/ui'
import { computed, inject, onBeforeUnmount, onMounted, ref, watch } from 'vue'

import AccentColorSelector from './AccentColorSelector.vue'
import { hexToRgb, rgbToHex, useAccentColor, type StandardAccent } from '@/composables/use-accent-color'
import { type ColorTheme, isDarkTheme, useTheme } from '@/composables/use-theme.ts'
import { type AppSettings, get, set } from '@/helpers/settings.ts'
import { getOS } from '@/helpers/utils'
import { appSettingsModalContextKey } from '@/providers/app-settings-modal'

const theme = useTheme()
const accent = useAccentColor()
const auth = injectAuth()
const { updatePreferences } = injectUserPreferences()
const settingsModal = inject(appSettingsModalContextKey, null)
const os = await getOS()
const settings = ref(await get())

type AppearanceSettingsState = {
	theme: ColorTheme
	customWindowHex: string
	syncAcrossDevices: boolean
	advancedRendering: boolean
	nativeDecorations: boolean
	accentColor: StandardAccent
	customAccentHex: string
}

function getAppearanceSettingsState(settings: AppSettings): AppearanceSettingsState {
	return {
		theme: settings.theme,
		customWindowHex: accent.savedCustomBgHex.value,
		syncAcrossDevices: settings.sync_theme_across_devices,
		advancedRendering: settings.advanced_rendering,
		nativeDecorations: settings.native_decorations,
		accentColor: accent.savedAccent.value,
		customAccentHex: accent.savedCustomHex.value,
	}
}

const { saved, current, changes, saving, hasChanges, reset, save } = useSavable(
	() => getAppearanceSettingsState(settings.value),
	async (appearanceChanges) => {
		const value = current.value
		if (
			value.syncAcrossDevices &&
			auth.user.value &&
			(appearanceChanges.theme !== undefined || appearanceChanges.syncAcrossDevices !== undefined)
		) {
			await updatePreferences({
				appearance: value.theme === 'system' ? { auto: true } : { auto: false, theme: value.theme },
			})
		}

		const nextSettings: AppSettings = {
			...settings.value,
			theme: value.theme,
			sync_theme_across_devices: value.syncAcrossDevices,
			advanced_rendering: value.advancedRendering,
			native_decorations: value.nativeDecorations,
		}

		await set(nextSettings)
		settings.value = nextSettings
		if (isDarkTheme(value.theme)) {
			theme.preferredDark = value.theme
		}
		theme.preferred = value.theme
		theme.syncAcrossDevices = value.syncAcrossDevices
		theme.advancedRendering = value.advancedRendering

		// Commit all accent and custom window customizations
		accent.commit(value.accentColor, value.customAccentHex, value.theme === 'custom' ? 'custom-bg' : value.theme, value.customWindowHex)
	},
)

const themeOptions = computed(() =>
	theme.options.filter(
		(option) =>
			option !== 'retro' || settings.value.developer_mode || current.value.theme === 'retro',
	),
)

const preferredDarkTheme = computed(() =>
	isDarkTheme(current.value.theme) ? current.value.theme : theme.preferredDark,
)

function setTheme(value: ColorTheme): void {
	current.value.theme = value
}

function setSyncAcrossDevices(enabled: boolean): void {
	current.value.syncAcrossDevices = enabled
}

function setAdvancedRendering(enabled: boolean): void {
	current.value.advancedRendering = enabled
}

function setNativeDecorations(enabled: boolean): void {
	current.value.nativeDecorations = enabled
}

// Window RGB Studio state
const windowRgb = computed(() => hexToRgb(current.value.customWindowHex) || { r: 20, g: 12, b: 36 })
const windowR = ref(windowRgb.value.r)
const windowG = ref(windowRgb.value.g)
const windowB = ref(windowRgb.value.b)
const windowHexInput = ref(current.value.customWindowHex)

watch(
	() => current.value.customWindowHex,
	(newHex) => {
		if (newHex) {
			const rgb = hexToRgb(newHex)
			if (rgb) {
				windowR.value = rgb.r
				windowG.value = rgb.g
				windowB.value = rgb.b
				windowHexInput.value = newHex
			}
		}
	},
	{ immediate: true },
)

function updateWindowFromRgb() {
	const hex = rgbToHex(windowR.value, windowG.value, windowB.value)
	windowHexInput.value = hex
	current.value.customWindowHex = hex
}

function updateWindowFromHex(raw: string) {
	let val = raw.trim()
	if (!val.startsWith('#')) val = '#' + val
	windowHexInput.value = val
	const rgb = hexToRgb(val)
	if (rgb) {
		windowR.value = rgb.r
		windowG.value = rgb.g
		windowB.value = rgb.b
		current.value.customWindowHex = val
	}
}

const quickWindowShades = [
	{ name: 'Cyber Plum', hex: '#0d0716' },
	{ name: 'Blood Ruby', hex: '#140508' },
	{ name: 'Deep Forest', hex: '#04120a' },
	{ name: 'Deep Slate', hex: '#0f172a' },
	{ name: 'Midnight', hex: '#090e17' },
	{ name: 'Graphite', hex: '#18181b' },
]

watch(
	[() => current.value.theme, () => saved.value.theme],
	([selectedTheme, savedTheme]) => {
		theme.preview = selectedTheme === savedTheme ? null : selectedTheme
	},
	{ immediate: true },
)

watch(
	[
		() => current.value.theme,
		() => current.value.customWindowHex,
		() => current.value.accentColor,
		() => current.value.customAccentHex,
		() => saved.value.theme,
		() => saved.value.customWindowHex,
		() => saved.value.accentColor,
		() => saved.value.customAccentHex,
	],
	([currTheme, currWinHex, currAccent, currAccHex, savedTheme, savedWinHex, savedAccent, savedAccHex]) => {
		const isThemeChanged = currTheme !== savedTheme || (currTheme === 'custom' && currWinHex !== savedWinHex)
		const isAccentChanged = currAccent !== savedAccent || (currAccent === 'custom' && currAccHex !== savedAccHex)

		if (isThemeChanged || isAccentChanged) {
			accent.setPreview(
				currAccent,
				currAccHex,
				currTheme === 'custom' ? 'custom-bg' : currTheme,
				currWinHex,
			)
		} else {
			accent.setPreview(null, null, null, null)
		}
	},
	{ immediate: true },
)

async function saveAppearanceSettings(): Promise<void> {
	try {
		await save()
	} catch {
		return
	}
}

onMounted(() => {
	settingsModal?.registerUnsavedChangesController({
		hasChanges: () => hasChanges.value,
		getOriginal: () => saved.value,
		getModified: () => changes.value,
		isSaving: () => saving.value,
		reset: () => {
			reset()
			accent.setPreview(null, null, null, null)
		},
		save: saveAppearanceSettings,
	})
})

onBeforeUnmount(() => {
	theme.preview = null
	accent.setPreview(null, null, null, null)
	settingsModal?.registerUnsavedChangesController(null)
})

provideAppearanceSettings({
	deferPersistence: true,
	theme: {
		current: computed(() => current.value.theme),
		options: themeOptions,
		system: computed(() => (theme.native === 'light' ? 'light' : preferredDarkTheme.value)),
		preferredDark: preferredDarkTheme,
		set: setTheme,
		syncAcrossDevices: {
			value: computed(() => current.value.syncAcrossDevices),
			set: setSyncAcrossDevices,
		},
		syncDisabled: computed(() => !auth.user.value),
	},
	advancedRendering: {
		value: computed(() => current.value.advancedRendering),
		set: setAdvancedRendering,
	},
	nativeDecorations:
		os !== 'MacOS'
			? {
					value: computed(() => current.value.nativeDecorations),
					set: setNativeDecorations,
				}
			: undefined,
	updatePreferences,
})
</script>

<template>
	<div>
		<AppearanceSettingsLayout />

		<!-- Custom Window Theme Studio (Directly extends the Color Theme section above) -->
		<div
			v-if="current.theme === 'custom'"
			class="mt-4 p-5 rounded-2xl border-2 border-brand/40 bg-surface-2/80 backdrop-blur-md flex flex-col gap-4 shadow-xl shadow-brand/5 animate-fade-in"
		>
			<div class="flex items-center justify-between flex-wrap gap-2">
				<div class="flex items-center gap-2">
					<span class="text-lg">🎛️</span>
					<span class="font-semibold text-contrast text-base">Custom Launcher Window Color</span>
				</div>
				<span class="text-xs text-secondary font-mono bg-surface-4/60 px-2.5 py-1 rounded-lg">
					RGB({{ windowR }}, {{ windowG }}, {{ windowB }})
				</span>
			</div>

			<div class="grid grid-cols-1 md:grid-cols-3 gap-4 items-center">
				<!-- Window Color Box Preview -->
				<div
					class="h-20 rounded-xl p-3 flex flex-col justify-between shadow-inner relative overflow-hidden transition-colors border border-white/10"
					:style="{ backgroundColor: current.customWindowHex }"
				>
					<div class="flex justify-between items-center text-xs font-bold px-1 text-white drop-shadow-[0_1px_2px_rgba(0,0,0,0.8)]">
						<span>WINDOW TONE</span>
						<span>{{ windowHexInput.toUpperCase() }}</span>
					</div>
					<span class="px-2 py-0.5 rounded bg-black/50 text-white text-[10px] font-medium w-fit backdrop-blur-sm">
						Window Surfaces & Sidebar
					</span>
				</div>

				<!-- Picker & Hex -->
				<div class="md:col-span-2 flex flex-col gap-3">
					<div class="flex items-center gap-3">
						<input
							type="color"
							:value="current.customWindowHex"
							class="w-11 h-11 rounded-xl cursor-pointer border-2 border-surface-4 bg-transparent p-0.5"
							@input="(e) => updateWindowFromHex((e.target as HTMLInputElement).value)"
						/>
						<div class="flex-1 flex flex-col gap-1">
							<label class="text-xs font-medium text-secondary">Window Background HEX</label>
							<input
								type="text"
								maxlength="7"
								:value="windowHexInput"
								class="w-full bg-surface-3 text-contrast font-mono text-sm px-3 py-2 rounded-xl border border-surface-4 focus:border-brand focus:outline-none focus:ring-1 focus:ring-brand"
								placeholder="#0d0716"
								@input="(e) => updateWindowFromHex((e.target as HTMLInputElement).value)"
							/>
						</div>
					</div>

					<div class="flex items-center gap-1.5 flex-wrap">
						<span class="text-[11px] text-secondary font-medium mr-1">Tones:</span>
						<button
							v-for="qp in quickWindowShades"
							:key="qp.hex"
							type="button"
							class="w-6 h-6 rounded-full border border-surface-4 hover:scale-125 transition-transform cursor-pointer shadow-sm"
							:style="{ backgroundColor: qp.hex }"
							:title="qp.name"
							@click="updateWindowFromHex(qp.hex)"
						/>
					</div>
				</div>
			</div>

			<!-- RGB Range Sliders -->
			<div class="grid grid-cols-1 sm:grid-cols-3 gap-3 pt-2 border-t border-surface-4/40">
				<div class="flex flex-col gap-1 bg-surface-3/50 p-2.5 rounded-xl border border-surface-4/30">
					<div class="flex justify-between items-center text-xs font-semibold">
						<span class="text-red-400">Red (R)</span>
						<span class="font-mono text-contrast">{{ windowR }}</span>
					</div>
					<input
						type="range"
						min="0"
						max="255"
						v-model.number="windowR"
						class="w-full accent-red-500 cursor-pointer h-1.5 bg-surface-4 rounded-lg"
						@input="updateWindowFromRgb"
					/>
				</div>

				<div class="flex flex-col gap-1 bg-surface-3/50 p-2.5 rounded-xl border border-surface-4/30">
					<div class="flex justify-between items-center text-xs font-semibold">
						<span class="text-green-400">Green (G)</span>
						<span class="font-mono text-contrast">{{ windowG }}</span>
					</div>
					<input
						type="range"
						min="0"
						max="255"
						v-model.number="windowG"
						class="w-full accent-green-500 cursor-pointer h-1.5 bg-surface-4 rounded-lg"
						@input="updateWindowFromRgb"
					/>
				</div>

				<div class="flex flex-col gap-1 bg-surface-3/50 p-2.5 rounded-xl border border-surface-4/30">
					<div class="flex justify-between items-center text-xs font-semibold">
						<span class="text-blue-400">Blue (B)</span>
						<span class="font-mono text-contrast">{{ windowB }}</span>
					</div>
					<input
						type="range"
						min="0"
						max="255"
						v-model.number="windowB"
						class="w-full accent-blue-500 cursor-pointer h-1.5 bg-surface-4 rounded-lg"
						@input="updateWindowFromRgb"
					/>
				</div>
			</div>
		</div>

		<!-- Accent Theme Selector -->
		<AccentColorSelector
			v-model="current.accentColor"
			v-model:custom-hex="current.customAccentHex"
		/>
	</div>
</template>

<style scoped>
@keyframes fadeIn {
	from {
		opacity: 0;
		transform: translateY(-6px);
	}
	to {
		opacity: 1;
		transform: translateY(0);
	}
}

.animate-fade-in {
	animation: fadeIn 0.2s ease-out forwards;
}
</style>