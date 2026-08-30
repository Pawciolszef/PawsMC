<template>
	<Transition name="splash-fade" @after-leave="onAfterLeave">
		<div v-if="!doneLoading" class="splash-screen" :class="`${theme.active}-mode`">
			<div class="app-logo-wrapper" data-tauri-drag-region>
				<svg
					class="app-logo"
					viewBox="0 0 1000 280"
					fill="none"
					color="var(--color-contrast)"
				>
					<defs>
						<linearGradient id="splash-paws-grad" x1="0%" y1="0%" x2="100%" y2="100%">
							<stop offset="0%" stop-color="#C084FC" />
							<stop offset="50%" stop-color="#7C3AED" />
							<stop offset="100%" stop-color="#06B6D4" />
						</linearGradient>
						<filter id="splash-paws-glow" x="-20%" y="-20%" width="140%" height="140%">
							<feGaussianBlur stdDeviation="8" result="blur" />
							<feComposite in="SourceGraphic" in2="blur" operator="over" />
						</filter>
					</defs>

					<!-- Paw Emblem -->
					<g class="paw-icon" filter="url(#splash-paws-glow)">
						<path
							d="M130 135 C95 135 80 165 85 190 C90 212 108 225 130 225 C152 225 170 212 175 190 C180 165 165 135 130 135 Z"
							fill="url(#splash-paws-grad)"
						/>
						<ellipse cx="75" cy="100" rx="20" ry="28" transform="rotate(-20 75 100)" fill="url(#splash-paws-grad)" />
						<ellipse cx="112" cy="80" rx="21" ry="30" transform="rotate(-6 112 80)" fill="url(#splash-paws-grad)" />
						<ellipse cx="150" cy="80" rx="21" ry="30" transform="rotate(6 150 80)" fill="url(#splash-paws-grad)" />
						<ellipse cx="187" cy="100" rx="20" ry="28" transform="rotate(20 187 100)" fill="url(#splash-paws-grad)" />
					</g>

					<!-- PawsMC Typography -->
					<g class="brand-text">
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
						<text
							x="640"
							y="190"
							font-family="system-ui, -apple-system, 'Outfit', 'Inter', sans-serif"
							font-size="140"
							font-weight="900"
							letter-spacing="-1"
							fill="url(#splash-paws-grad)"
						>
							MC
						</text>
					</g>
				</svg>
				<ProgressBar class="loading-bar" :progress="Math.min(loadingProgress, 100)" />
				<span v-if="message">{{ message }}</span>
			</div>
			<div class="gradient-bg" data-tauri-drag-region></div>
			<div class="cube-bg"></div>
			<div class="base-bg"></div>
		</div>
	</Transition>
</template>

<script setup>
import { injectLoadingState } from '@modrinth/ui'
import { ref, watch } from 'vue'

import ProgressBar from '@/components/ui/ProgressBar.vue'
import { useAppEvent } from '@/composables/use-app-event'
import { useTheme } from '@/composables/use-theme.ts'

const theme = useTheme()

const doneLoading = ref(false)
const loadingProgress = ref(0)
const message = ref()

const MIN_DISPLAY_MS = 500
const mountedAt = Date.now()

const loading = injectLoadingState()

function onAfterLeave() {
	loading.setEnabled(true)
}

watch(
	[loading.barEnabled, loading.pending],
	([barEnabled, pending]) => {
		if (barEnabled) {
			return
		}

		if (pending) {
			loadingProgress.value = 0
			fakeLoadingIncrease()
			return
		}

		const elapsed = Date.now() - mountedAt
		const delay = Math.max(0, MIN_DISPLAY_MS - elapsed)

		setTimeout(() => {
			if (loading.pending.value) {
				return
			}
			doneLoading.value = true
		}, delay)
	},
	{ immediate: true },
)

function fakeLoadingIncrease() {
	if (loadingProgress.value < 95) {
		setTimeout(() => {
			loadingProgress.value += 2
			fakeLoadingIncrease()
		}, 5)
	}
}

useAppEvent('loading', (e) => {
	if (e.event.type === 'directory_move') {
		loadingProgress.value = 100 * (e.fraction ?? 1)
		message.value = 'Updating app directory...'
	}
})
</script>

<style scoped lang="scss">
.splash-screen {
	position: fixed;
	inset: 0;
	z-index: 10000;

	--splash-cube-image: url('@/assets/loading/cube.png');

	&.light-mode {
		--splash-cube-image: url('@/assets/loading/cube-light.webp');
	}
}

.splash-fade-leave-active {
	transition: opacity 0.3s ease-in-out;
}

.splash-fade-leave-to {
	opacity: 0;
}

.app-logo-wrapper {
	position: absolute;
	height: 100vh;
	width: 100%;

	display: flex;
	flex-direction: column;
	justify-content: center;
	align-items: center;

	gap: 1rem;
	color: var(--color-contrast);

	z-index: 9998;
}

.app-logo {
	height: 2.25rem;
	width: fit-content;
}

.loading-bar {
	max-width: 20rem;
}

.gradient-bg {
	position: absolute;
	height: 100vh;
	width: 100vw;
	background:
		linear-gradient(180deg, var(--splash-tint-top) 0%, var(--splash-tint-bottom) 97.29%),
		linear-gradient(0deg, var(--splash-overlay), var(--splash-overlay));
	z-index: 9997;
}

.cube-bg {
	position: absolute;

	left: 50%;
	top: 50%;
	transform: translate(-50%, -50%);

	width: 180vw;
	height: 180vh;
	background-color: var(--color-bg);

	z-index: 9996;

	&::after {
		content: '';
		position: absolute;
		inset: 0;
		background: var(--splash-cube-image) center no-repeat;
		background-size: contain;
		opacity: var(--splash-cube-opacity);
		mix-blend-mode: var(--splash-cube-blend);
	}
}

.base-bg {
	position: absolute;
	top: 0;
	left: 0;
	width: 100%;
	height: 100%;
	background: var(--color-bg);
	z-index: 9995;
}
</style>
