<template>
	<div class="inline-flex items-center gap-2 select-none" :class="{ animate }">
		<img
			src="/icon.png"
			alt="PawsMC"
			class="h-7 w-7 object-contain drop-shadow-[0_2px_8px_rgba(56,189,248,0.35)] pointer-events-none shrink-0"
		/>
		<div class="flex items-baseline font-extrabold tracking-tight text-lg leading-none shrink-0">
			<span class="text-contrast">Paws</span>
			<span class="text-brand font-black ml-0.5">MC</span>
		</div>
	</div>
</template>

<script setup>
defineProps({
	animate: {
		type: Boolean,
		default: false,
	},
})
</script>

<style lang="scss" scoped>
.animate {
	img {
		animation: pulse-glow 2s ease-in-out infinite alternate;
	}
	@keyframes pulse-glow {
		from {
			filter: drop-shadow(0 0 4px rgba(56, 189, 248, 0.3));
		}
		to {
			filter: drop-shadow(0 0 12px rgba(56, 189, 248, 0.7));
		}
	}
}
</style>
