<script setup lang="ts">
import {
	ACCENT_OPTIONS,
	type AccentColor,
	type AccentOption,
} from '@/composables/use-accent-color'

const props = defineProps<{
	modelValue: AccentColor
}>()

const emit = defineEmits<{
	'update:modelValue': [accent: AccentColor]
}>()

function selectAccent(opt: AccentOption) {
	emit('update:modelValue', opt.id)
}
</script>

<template>
	<div class="mt-8 border-0 border-t border-solid border-divider pt-6 flex flex-col gap-4">
		<!-- Section Header -->
		<div class="flex flex-col gap-1">
			<h2 class="m-0 text-xl font-semibold text-contrast flex items-center gap-2">
				<span>PawsMC Accent Theme</span>
			</h2>
			<p class="m-0 text-secondary text-sm">
				Select your favorite accent color for buttons, sidebars, progress bars, and glowing highlights.
			</p>
		</div>

		<!-- Palette Grid -->
		<div class="grid grid-cols-2 sm:grid-cols-4 gap-3">
			<button
				v-for="opt in ACCENT_OPTIONS"
				:key="opt.id"
				type="button"
				class="group relative flex flex-col items-center justify-center p-3.5 rounded-2xl border-2 transition-all cursor-pointer bg-surface-2 hover:bg-surface-3 text-left focus:outline-none"
				:class="
					modelValue === opt.id
						? 'border-brand shadow-md shadow-brand/10 bg-surface-3 ring-2 ring-brand/20'
						: 'border-surface-4 hover:border-surface-5 opacity-80 hover:opacity-100'
				"
				@click="selectAccent(opt)"
			>
				<!-- Color circle swatch with gradient & glow -->
				<div
					class="w-8 h-8 rounded-full mb-2.5 shadow-md flex items-center justify-center transition-transform group-hover:scale-110"
					:style="{ background: opt.previewBg }"
				>
					<div
						v-if="modelValue === opt.id"
						class="w-2.5 h-2.5 rounded-full bg-white shadow-sm"
					/>
				</div>

				<div class="font-semibold text-sm text-contrast">
					{{ opt.name }}
				</div>

				<!-- Active pill badge -->
				<span
					v-if="modelValue === opt.id"
					class="mt-1.5 text-[10px] font-bold uppercase tracking-wider px-2 py-0.5 rounded-full bg-brand/15 text-brand"
				>
					Selected
				</span>
			</button>
		</div>
	</div>
</template>