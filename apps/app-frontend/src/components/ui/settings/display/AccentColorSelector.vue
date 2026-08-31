<script setup lang="ts">
import { computed, ref, watch } from 'vue'
import {
	ACCENT_OPTIONS,
	BG_PRESETS,
	hexToRgb,
	rgbToHex,
	type AccentOption,
	type BgOption,
	type StandardAccent,
} from '@/composables/use-accent-color'

const props = defineProps<{
	modelValue: StandardAccent
	customHex?: string
	bgTheme?: string
	customBgHex?: string
}>()

const emit = defineEmits<{
	'update:modelValue': [accent: StandardAccent]
	'update:customHex': [hex: string]
	'update:bgTheme': [theme: string]
	'update:customBgHex': [hex: string]
}>()

// Accent state
const currentCustomHex = computed(() => props.customHex || '#ec4899')
const accentRgb = computed(() => hexToRgb(currentCustomHex.value) || { r: 236, g: 72, b: 153 })

const accentR = ref(accentRgb.value.r)
const accentG = ref(accentRgb.value.g)
const accentB = ref(accentRgb.value.b)
const accentHexInput = ref(currentCustomHex.value)

watch(
	() => props.customHex,
	(newHex) => {
		if (newHex) {
			const rgb = hexToRgb(newHex)
			if (rgb) {
				accentR.value = rgb.r
				accentG.value = rgb.g
				accentB.value = rgb.b
				accentHexInput.value = newHex
			}
		}
	},
	{ immediate: true },
)

function updateAccentFromRgb() {
	const hex = rgbToHex(accentR.value, accentG.value, accentB.value)
	accentHexInput.value = hex
	emit('update:customHex', hex)
	if (props.modelValue !== 'custom') {
		emit('update:modelValue', 'custom')
	}
}

function updateAccentFromHex(raw: string) {
	let val = raw.trim()
	if (!val.startsWith('#')) val = '#' + val
	accentHexInput.value = val
	const rgb = hexToRgb(val)
	if (rgb) {
		accentR.value = rgb.r
		accentG.value = rgb.g
		accentB.value = rgb.b
		emit('update:customHex', val)
		if (props.modelValue !== 'custom') {
			emit('update:modelValue', 'custom')
		}
	}
}

function selectAccent(opt: AccentOption) {
	emit('update:modelValue', opt.id)
	if (opt.id === 'custom') {
		emit('update:customHex', accentHexInput.value || '#ec4899')
	}
}

// Background Window Theme state
const currentBgTheme = computed(() => props.bgTheme || 'default')
const currentCustomBgHex = computed(() => props.customBgHex || '#140c24')
const bgRgb = computed(() => hexToRgb(currentCustomBgHex.value) || { r: 20, g: 12, b: 36 })

const bgR = ref(bgRgb.value.r)
const bgG = ref(bgRgb.value.g)
const bgB = ref(bgRgb.value.b)
const bgHexInput = ref(currentCustomBgHex.value)

watch(
	() => props.customBgHex,
	(newHex) => {
		if (newHex) {
			const rgb = hexToRgb(newHex)
			if (rgb) {
				bgR.value = rgb.r
				bgG.value = rgb.g
				bgB.value = rgb.b
				bgHexInput.value = newHex
			}
		}
	},
	{ immediate: true },
)

function updateBgFromRgb() {
	const hex = rgbToHex(bgR.value, bgG.value, bgB.value)
	bgHexInput.value = hex
	emit('update:customBgHex', hex)
	if (currentBgTheme.value !== 'custom-bg') {
		emit('update:bgTheme', 'custom-bg')
	}
}

function updateBgFromHex(raw: string) {
	let val = raw.trim()
	if (!val.startsWith('#')) val = '#' + val
	bgHexInput.value = val
	const rgb = hexToRgb(val)
	if (rgb) {
		bgR.value = rgb.r
		bgG.value = rgb.g
		bgB.value = rgb.b
		emit('update:customBgHex', val)
		if (currentBgTheme.value !== 'custom-bg') {
			emit('update:bgTheme', 'custom-bg')
		}
	}
}

function selectBgPreset(opt: BgOption) {
	emit('update:bgTheme', opt.id)
	if (opt.id === 'custom-bg') {
		emit('update:customBgHex', bgHexInput.value || '#140c24')
	}
}

const quickCustomAccents = [
	{ name: 'Neon Coral', hex: '#ff5e7e' },
	{ name: 'Electric Cyan', hex: '#00f0ff' },
	{ name: 'Lavender', hex: '#d8b4fe' },
	{ name: 'Lime Zest', hex: '#84cc16' },
	{ name: 'Peach Glow', hex: '#fca5a5' },
	{ name: 'Deep Indigo', hex: '#6366f1' },
	{ name: 'Amber Gold', hex: '#f59e0b' },
	{ name: 'Hot Magenta', hex: '#f43f5e' },
]

const quickBgPresets = [
	{ name: 'Midnight', hex: '#090e17' },
	{ name: 'Pure OLED', hex: '#000000' },
	{ name: 'Dark Velvet', hex: '#160822' },
	{ name: 'Blood Ruby', hex: '#18070b' },
	{ name: 'Deep Forest', hex: '#06160e' },
	{ name: 'Graphite', hex: '#18181b' },
]
</script>

<template>
	<div class="mt-8 border-0 border-t border-solid border-divider pt-6 flex flex-col gap-8">
		
		<!-- ================================================================= -->
		<!-- 1. WINDOW BACKGROUND THEME (Całe okno launchera / Tło)            -->
		<!-- ================================================================= -->
		<div class="flex flex-col gap-4">
			<div class="flex flex-col gap-1">
				<h2 class="m-0 text-xl font-semibold text-contrast flex items-center gap-2">
					<span>🖼️ Launcher Window & Background Theme</span>
				</h2>
				<p class="m-0 text-secondary text-sm">
					Select the base color for the entire launcher window (sidebar, top bar, background & cards) or choose a custom RGB tint.
				</p>
			</div>

			<!-- Window BG Preset Grid -->
			<div class="grid grid-cols-2 sm:grid-cols-4 gap-3">
				<button
					v-for="opt in BG_PRESETS"
					:key="opt.id"
					type="button"
					class="group relative flex flex-col items-center justify-center p-3.5 rounded-2xl border-2 transition-all cursor-pointer bg-surface-2 hover:bg-surface-3 text-left focus:outline-none"
					:class="
						currentBgTheme === opt.id
							? 'border-brand shadow-lg shadow-brand/15 bg-surface-3 ring-2 ring-brand/30 scale-[1.02]'
							: 'border-surface-4 hover:border-surface-5 opacity-80 hover:opacity-100 hover:scale-[1.01]'
					"
					@click="selectBgPreset(opt)"
				>
					<div
						class="w-9 h-9 rounded-xl mb-2.5 shadow-md flex items-center justify-center transition-transform group-hover:scale-110 relative border border-white/10"
						:style="{
							background: opt.id === 'custom-bg' ? currentCustomBgHex : opt.previewBg,
							boxShadow: currentBgTheme === opt.id ? `0 0 12px ${opt.id === 'custom-bg' ? currentCustomBgHex : opt.hex}66` : undefined,
						}"
					>
						<div
							v-if="currentBgTheme === opt.id"
							class="w-2.5 h-2.5 rounded-full bg-white shadow-sm ring-2 ring-black/20"
						/>
					</div>

					<div class="flex items-center gap-1.5 font-semibold text-sm text-contrast">
						<span>{{ opt.emoji }}</span>
						<span class="truncate max-w-[110px]">{{ opt.name }}</span>
					</div>

					<span
						v-if="currentBgTheme === opt.id"
						class="mt-1.5 text-[10px] font-bold uppercase tracking-wider px-2 py-0.5 rounded-full bg-brand/15 text-brand"
					>
						Selected
					</span>
				</button>
			</div>

			<!-- Custom Window BG Studio -->
			<div
				v-if="currentBgTheme === 'custom-bg'"
				class="p-5 rounded-2xl border-2 border-brand/40 bg-surface-2/80 backdrop-blur-md flex flex-col gap-4 shadow-xl shadow-brand/5 animate-fade-in"
			>
				<div class="flex items-center justify-between flex-wrap gap-2">
					<div class="flex items-center gap-2">
						<span class="text-lg">🎛️</span>
						<span class="font-semibold text-contrast text-base">Custom Window Background Studio</span>
					</div>
					<span class="text-xs text-secondary font-mono bg-surface-4/60 px-2.5 py-1 rounded-lg">
						RGB({{ bgR }}, {{ bgG }}, {{ bgB }})
					</span>
				</div>

				<div class="grid grid-cols-1 md:grid-cols-3 gap-4 items-center">
					<!-- Preview Box -->
					<div
						class="h-20 rounded-xl p-3 flex flex-col justify-between shadow-inner relative overflow-hidden transition-colors border border-white/10"
						:style="{ backgroundColor: currentCustomBgHex }"
					>
						<div class="flex justify-between items-center text-xs font-bold px-1 text-white drop-shadow-[0_1px_2px_rgba(0,0,0,0.8)]">
							<span>WINDOW BG</span>
							<span>{{ bgHexInput.toUpperCase() }}</span>
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
								:value="currentCustomBgHex"
								class="w-11 h-11 rounded-xl cursor-pointer border-2 border-surface-4 bg-transparent p-0.5"
								@input="(e) => updateBgFromHex((e.target as HTMLInputElement).value)"
							/>
							<div class="flex-1 flex flex-col gap-1">
								<label class="text-xs font-medium text-secondary">Window Background HEX</label>
								<input
									type="text"
									maxlength="7"
									:value="bgHexInput"
									class="w-full bg-surface-3 text-contrast font-mono text-sm px-3 py-2 rounded-xl border border-surface-4 focus:border-brand focus:outline-none focus:ring-1 focus:ring-brand"
									placeholder="#140c24"
									@input="(e) => updateBgFromHex((e.target as HTMLInputElement).value)"
								/>
							</div>
						</div>

						<div class="flex items-center gap-1.5 flex-wrap">
							<span class="text-[11px] text-secondary font-medium mr-1">Shades:</span>
							<button
								v-for="qp in quickBgPresets"
								:key="qp.hex"
								type="button"
								class="w-6 h-6 rounded-full border border-surface-4 hover:scale-125 transition-transform cursor-pointer shadow-sm"
								:style="{ backgroundColor: qp.hex }"
								:title="qp.name"
								@click="updateBgFromHex(qp.hex)"
							/>
						</div>
					</div>
				</div>

				<!-- RGB Range Sliders -->
				<div class="grid grid-cols-1 sm:grid-cols-3 gap-3 pt-2 border-t border-surface-4/40">
					<div class="flex flex-col gap-1 bg-surface-3/50 p-2.5 rounded-xl border border-surface-4/30">
						<div class="flex justify-between items-center text-xs font-semibold">
							<span class="text-red-400">Red (R)</span>
							<span class="font-mono text-contrast">{{ bgR }}</span>
						</div>
						<input
							type="range"
							min="0"
							max="255"
							v-model.number="bgR"
							class="w-full accent-red-500 cursor-pointer h-1.5 bg-surface-4 rounded-lg"
							@input="updateBgFromRgb"
						/>
					</div>

					<div class="flex flex-col gap-1 bg-surface-3/50 p-2.5 rounded-xl border border-surface-4/30">
						<div class="flex justify-between items-center text-xs font-semibold">
							<span class="text-green-400">Green (G)</span>
							<span class="font-mono text-contrast">{{ bgG }}</span>
						</div>
						<input
							type="range"
							min="0"
							max="255"
							v-model.number="bgG"
							class="w-full accent-green-500 cursor-pointer h-1.5 bg-surface-4 rounded-lg"
							@input="updateBgFromRgb"
						/>
					</div>

					<div class="flex flex-col gap-1 bg-surface-3/50 p-2.5 rounded-xl border border-surface-4/30">
						<div class="flex justify-between items-center text-xs font-semibold">
							<span class="text-blue-400">Blue (B)</span>
							<span class="font-mono text-contrast">{{ bgB }}</span>
						</div>
						<input
							type="range"
							min="0"
							max="255"
							v-model.number="bgB"
							class="w-full accent-blue-500 cursor-pointer h-1.5 bg-surface-4 rounded-lg"
							@input="updateBgFromRgb"
						/>
					</div>
				</div>
			</div>
		</div>


		<!-- ================================================================= -->
		<!-- 2. ACCENT & HIGHLIGHT THEME (Kolor akcentów, przycisków i glow)   -->
		<!-- ================================================================= -->
		<div class="flex flex-col gap-4">
			<div class="flex flex-col gap-1">
				<h2 class="m-0 text-xl font-semibold text-contrast flex items-center gap-2">
					<span>✨ Accent & Highlight Colors</span>
				</h2>
				<p class="m-0 text-secondary text-sm">
					Select the accent highlight color for buttons, active tabs, progress bars, glowing borders and badges.
				</p>
			</div>

			<!-- Accent Grid -->
			<div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-3 lg:grid-cols-5 gap-3">
				<button
					v-for="opt in ACCENT_OPTIONS"
					:key="opt.id"
					type="button"
					class="group relative flex flex-col items-center justify-center p-3.5 rounded-2xl border-2 transition-all cursor-pointer bg-surface-2 hover:bg-surface-3 text-left focus:outline-none"
					:class="
						modelValue === opt.id
							? 'border-brand shadow-lg shadow-brand/15 bg-surface-3 ring-2 ring-brand/30 scale-[1.02]'
							: 'border-surface-4 hover:border-surface-5 opacity-80 hover:opacity-100 hover:scale-[1.01]'
					"
					@click="selectAccent(opt)"
				>
					<div
						class="w-9 h-9 rounded-full mb-2.5 shadow-md flex items-center justify-center transition-transform group-hover:scale-110 relative"
						:style="{
							background: opt.id === 'custom' ? currentCustomHex : opt.previewBg,
							boxShadow: modelValue === opt.id ? `0 0 12px ${opt.id === 'custom' ? currentCustomHex : opt.color}66` : undefined,
						}"
					>
						<div
							v-if="modelValue === opt.id"
							class="w-2.5 h-2.5 rounded-full bg-white shadow-sm ring-2 ring-black/20"
						/>
					</div>

					<div class="flex items-center gap-1.5 font-semibold text-sm text-contrast">
						<span>{{ opt.emoji }}</span>
						<span class="truncate max-w-[100px]">{{ opt.name }}</span>
					</div>

					<span
						v-if="modelValue === opt.id"
						class="mt-1.5 text-[10px] font-bold uppercase tracking-wider px-2 py-0.5 rounded-full bg-brand/15 text-brand"
					>
						Selected
					</span>
				</button>
			</div>

			<!-- Custom Accent Studio -->
			<div
				v-if="modelValue === 'custom'"
				class="p-5 rounded-2xl border-2 border-brand/40 bg-surface-2/80 backdrop-blur-md flex flex-col gap-4 shadow-xl shadow-brand/5 animate-fade-in"
			>
				<div class="flex items-center justify-between flex-wrap gap-2">
					<div class="flex items-center gap-2">
						<span class="text-lg">🎛️</span>
						<span class="font-semibold text-contrast text-base">Custom Accent Studio</span>
					</div>
					<span class="text-xs text-secondary font-mono bg-surface-4/60 px-2.5 py-1 rounded-lg">
						RGB({{ accentR }}, {{ accentG }}, {{ accentB }})
					</span>
				</div>

				<div class="grid grid-cols-1 md:grid-cols-3 gap-4 items-center">
					<!-- Preview Box -->
					<div
						class="h-20 rounded-xl p-3 flex flex-col justify-between shadow-inner relative overflow-hidden transition-colors border border-white/10"
						:style="{ backgroundColor: currentCustomHex }"
					>
						<div class="flex justify-between items-center text-xs font-bold px-1 text-white drop-shadow-[0_1px_2px_rgba(0,0,0,0.8)]">
							<span>ACCENT</span>
							<span>{{ accentHexInput.toUpperCase() }}</span>
						</div>
						<div class="flex gap-1.5">
							<span class="px-2 py-0.5 rounded bg-black/40 text-white text-[10px] font-medium backdrop-blur-sm">
								Glow & Buttons
							</span>
							<span class="px-2 py-0.5 rounded bg-white/40 text-black text-[10px] font-semibold backdrop-blur-sm">
								Active
							</span>
						</div>
					</div>

					<!-- Picker & Hex -->
					<div class="md:col-span-2 flex flex-col gap-3">
						<div class="flex items-center gap-3">
							<input
								type="color"
								:value="currentCustomHex"
								class="w-11 h-11 rounded-xl cursor-pointer border-2 border-surface-4 bg-transparent p-0.5"
								@input="(e) => updateAccentFromHex((e.target as HTMLInputElement).value)"
							/>
							<div class="flex-1 flex flex-col gap-1">
								<label class="text-xs font-medium text-secondary">Accent HEX Code</label>
								<input
									type="text"
									maxlength="7"
									:value="accentHexInput"
									class="w-full bg-surface-3 text-contrast font-mono text-sm px-3 py-2 rounded-xl border border-surface-4 focus:border-brand focus:outline-none focus:ring-1 focus:ring-brand"
									placeholder="#ec4899"
									@input="(e) => updateAccentFromHex((e.target as HTMLInputElement).value)"
								/>
							</div>
						</div>

						<div class="flex items-center gap-1.5 flex-wrap">
							<span class="text-[11px] text-secondary font-medium mr-1">Quick:</span>
							<button
								v-for="qp in quickCustomAccents"
								:key="qp.hex"
								type="button"
								class="w-6 h-6 rounded-full border border-surface-4 hover:scale-125 transition-transform cursor-pointer shadow-sm"
								:style="{ backgroundColor: qp.hex }"
								:title="qp.name"
								@click="updateAccentFromHex(qp.hex)"
							/>
						</div>
					</div>
				</div>

				<!-- RGB Sliders -->
				<div class="grid grid-cols-1 sm:grid-cols-3 gap-3 pt-2 border-t border-surface-4/40">
					<div class="flex flex-col gap-1 bg-surface-3/50 p-2.5 rounded-xl border border-surface-4/30">
						<div class="flex justify-between items-center text-xs font-semibold">
							<span class="text-red-400">Red (R)</span>
							<span class="font-mono text-contrast">{{ accentR }}</span>
						</div>
						<input
							type="range"
							min="0"
							max="255"
							v-model.number="accentR"
							class="w-full accent-red-500 cursor-pointer h-1.5 bg-surface-4 rounded-lg"
							@input="updateAccentFromRgb"
						/>
					</div>

					<div class="flex flex-col gap-1 bg-surface-3/50 p-2.5 rounded-xl border border-surface-4/30">
						<div class="flex justify-between items-center text-xs font-semibold">
							<span class="text-green-400">Green (G)</span>
							<span class="font-mono text-contrast">{{ accentG }}</span>
						</div>
						<input
							type="range"
							min="0"
							max="255"
							v-model.number="accentG"
							class="w-full accent-green-500 cursor-pointer h-1.5 bg-surface-4 rounded-lg"
							@input="updateAccentFromRgb"
						/>
					</div>

					<div class="flex flex-col gap-1 bg-surface-3/50 p-2.5 rounded-xl border border-surface-4/30">
						<div class="flex justify-between items-center text-xs font-semibold">
							<span class="text-blue-400">Blue (B)</span>
							<span class="font-mono text-contrast">{{ accentB }}</span>
						</div>
						<input
							type="range"
							min="0"
							max="255"
							v-model.number="accentB"
							class="w-full accent-blue-500 cursor-pointer h-1.5 bg-surface-4 rounded-lg"
							@input="updateAccentFromRgb"
						/>
					</div>
				</div>
			</div>
		</div>

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