<template>
	<svg
		xmlns="http://www.w3.org/2000/svg"
		viewBox="0 0 1000 280"
		fill="none"
		:class="{ animate }"
	>
		<defs>
			<linearGradient id="paws-logo-grad" x1="0%" y1="0%" x2="100%" y2="100%">
				<stop offset="0%" stop-color="#BAE6FD" />
				<stop offset="50%" stop-color="#38BDF8" />
				<stop offset="100%" stop-color="#0284C7" />
			</linearGradient>
			<filter id="paws-glow-filter" x="-20%" y="-20%" width="140%" height="140%">
				<feGaussianBlur stdDeviation="6" result="blur" />
				<feComposite in="SourceGraphic" in2="blur" operator="over" />
			</filter>
		</defs>

		<!-- Paw Emblem -->
		<g class="paw-icon" filter="url(#paws-glow-filter)">
			<!-- Main Pad -->
			<path
				d="M130 135 C95 135 80 165 85 190 C90 212 108 225 130 225 C152 225 170 212 175 190 C180 165 165 135 130 135 Z"
				fill="url(#paws-logo-grad)"
			/>
			<!-- Toes -->
			<ellipse cx="75" cy="100" rx="20" ry="28" transform="rotate(-20 75 100)" fill="url(#paws-logo-grad)" />
			<ellipse cx="112" cy="80" rx="21" ry="30" transform="rotate(-6 112 80)" fill="url(#paws-logo-grad)" />
			<ellipse cx="150" cy="80" rx="21" ry="30" transform="rotate(6 150 80)" fill="url(#paws-logo-grad)" />
			<ellipse cx="187" cy="100" rx="20" ry="28" transform="rotate(20 187 100)" fill="url(#paws-logo-grad)" />
		</g>

		<!-- PawsMC Typography -->
		<g class="brand-text">
			<!-- "Paws" text in clean modern white -->
			<text
				x="250"
				y="190"
				font-family="system-ui, -apple-system, 'Outfit', 'Inter', sans-serif"
				font-size="140"
				font-weight="800"
				letter-spacing="-2"
				fill="currentColor"
			>
				Paws
			</text>
			<!-- "MC" text with cyan/purple gradient -->
			<text
				x="640"
				y="190"
				font-family="system-ui, -apple-system, 'Outfit', 'Inter', sans-serif"
				font-size="140"
				font-weight="900"
				letter-spacing="-1"
				fill="url(#paws-logo-grad)"
			>
				MC
			</text>
		</g>
	</svg>
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
	.ring {
		transform-origin: center;
		transform-box: fill-box;
		animation-fill-mode: forwards;
		transition: transform 2s ease-in-out;
		&--large {
			animation: spin 1s ease-in-out infinite forwards;
		}
		&--small {
			animation: spin 2s ease-in-out infinite reverse;
		}
	}
	@keyframes spin {
		from {
			transform: rotate(0deg);
		}
		to {
			transform: rotate(360deg);
		}
	}
}
</style>
