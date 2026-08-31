<script setup lang="ts">
import { useAccentColor, type AccentColor } from '@/composables/use-accent-color'

const { currentAccent, accentOptions, setAccent } = useAccentColor()
</script>

<template>
	<div class="mt-8 border-0 border-t border-solid border-divider pt-6 flex flex-col gap-4">
		<div class="flex flex-col gap-1">
			<h2 class="m-0 text-xl font-semibold text-contrast flex items-center gap-2">
				<span>PawsMC Accent Theme</span>
			</h2>
			<p class="m-0 text-secondary text-sm">
				Select your favorite accent color for buttons, sidebars, progress bars, and glowing highlights.
			</p>
		</div>

		<div class="grid grid-cols-2 sm:grid-cols-4 gap-3">
			<button
				v-for="opt in accentOptions"
				:key="opt.id"
				type="button"
				class="group relative flex flex-col items-center justify-center p-3.5 rounded-2xl border-2 transition-all cursor-pointer bg-surface-2 hover:bg-surface-3 text-left focus:outline-none"
				:class="
					currentAccent === opt.id
						? 'border-brand shadow-md shadow-brand/10 bg-surface-3 ring-2 ring-brand/20'
						: 'border-surface-4 hover:border-surface-5 opacity-80 hover:opacity-100'
				"
				@click="setAccent(opt.id)"
			>
				<!-- Color circle swatch with gradient & glow -->
				<div
					class="w-8 h-8 rounded-full mb-2.5 shadow-md flex items-center justify-center transition-transform group-hover:scale-110"
					:style="{ background: opt.previewBg }"
				>
					<div
						v-if="currentAccent === opt.id"
						class="w-2.5 h-2.5 rounded-full bg-white shadow-sm"
					/>
				</div>

				<div class="flex items-center gap-1.5 font-semibold text-sm text-contrast">
					<span>{{ opt.emoji }}</span>
					<span>{{ opt.name }}</span>
				</div>

				<!-- Active pill badge -->
				<span
					v-if="currentAccent === opt.id"
					class="mt-1.5 text-[10px] font-bold uppercase tracking-wider px-2 py-0.5 rounded-full bg-brand/15 text-brand"
				>
					Active
				</span>
			</button>
		</div>
	</div>
</template>
