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
import { useAccentColor, type StandardAccent } from '@/composables/use-accent-color'
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
	syncAcrossDevices: boolean
	advancedRendering: boolean
	nativeDecorations: boolean
	accentColor: StandardAccent
	customAccentHex: string
	bgTheme: string
	customBgHex: string
}

function getAppearanceSettingsState(settings: AppSettings): AppearanceSettingsState {
	return {
		theme: settings.theme,
		syncAcrossDevices: settings.sync_theme_across_devices,
		advancedRendering: settings.advanced_rendering,
		nativeDecorations: settings.native_decorations,
		accentColor: accent.savedAccent.value,
		customAccentHex: accent.savedCustomHex.value,
		bgTheme: accent.savedBgTheme.value,
		customBgHex: accent.savedCustomBgHex.value,
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

		// Commit all accent and background customizations
		accent.commit(value.accentColor, value.customAccentHex, value.bgTheme, value.customBgHex)
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

watch(
	[() => current.value.theme, () => saved.value.theme],
	([selectedTheme, savedTheme]) => {
		theme.preview = selectedTheme === savedTheme ? null : selectedTheme
	},
	{ immediate: true },
)

watch(
	[
		() => current.value.accentColor,
		() => current.value.customAccentHex,
		() => current.value.bgTheme,
		() => current.value.customBgHex,
		() => saved.value.accentColor,
		() => saved.value.customAccentHex,
		() => saved.value.bgTheme,
		() => saved.value.customBgHex,
	],
	([currAccent, currHex, currBg, currBgHex, savedAccent, savedHex, savedBg, savedBgHex]) => {
		const isAccentChanged = currAccent !== savedAccent || (currAccent === 'custom' && currHex !== savedHex)
		const isBgChanged = currBg !== savedBg || (currBg === 'custom-bg' && currBgHex !== savedBgHex)
		if (isAccentChanged || isBgChanged) {
			accent.setPreview(currAccent, currHex, currBg, currBgHex)
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
		<AccentColorSelector
			v-model="current.accentColor"
			v-model:custom-hex="current.customAccentHex"
			v-model:bg-theme="current.bgTheme"
			v-model:custom-bg-hex="current.customBgHex"
		/>
	</div>
</template>