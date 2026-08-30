<script setup lang="ts">
import {
	ArrowUpRightIcon,
	CheckIcon,
	ClipboardCopyIcon,
	ImagesIcon,
	ShirtIcon,
} from '@modrinth/assets'
import { IconButton, useFormatDateTime } from '@modrinth/ui'
import { convertFileSrc } from '@tauri-apps/api/core'
import { readFile } from '@tauri-apps/plugin-fs'
import { onMounted, ref } from 'vue'
import { useRouter } from 'vue-router'

import { useAppEvent } from '@/composables/use-app-event'
import {
	list_all_screenshots,
	open_screenshot,
	type InstanceScreenshot,
	type ScreenshotKey,
} from '@/helpers/instance'

const router = useRouter()
const latestScreenshot = ref<InstanceScreenshot | null>(null)
const loading = ref(true)
const copied = ref(false)
let copyTimeout: number | undefined

const formatTime = useFormatDateTime({ dateStyle: 'short', timeStyle: 'short' })

function getScreenshotDate(s: InstanceScreenshot): Date {
	if (s.modified_at) {
		return new Date(s.modified_at > 1e11 ? s.modified_at : s.modified_at * 1000)
	}
	if (s.created_at) {
		return new Date(s.created_at)
	}
	return new Date()
}

async function fetchLatestScreenshot() {
	try {
		loading.value = true
		const all = await list_all_screenshots()
		if (all && all.length > 0) {
			const sorted = all.slice().sort((a, b) => {
				const timeA = getScreenshotDate(a).getTime()
				const timeB = getScreenshotDate(b).getTime()
				return timeB - timeA
			})
			latestScreenshot.value = sorted[0] ?? null
		} else {
			latestScreenshot.value = null
		}
	} catch (e) {
		console.warn('Failed to load screenshots for Quick Gallery:', e)
		latestScreenshot.value = null
	} finally {
		loading.value = false
	}
}

async function handleCopy() {
	if (!latestScreenshot.value?.path) return
	try {
		const bytes = await readFile(latestScreenshot.value.path)
		const blob = new Blob([bytes], { type: 'image/png' })
		await navigator.clipboard.write([new ClipboardItem({ 'image/png': blob })])
		copied.value = true
		if (copyTimeout) clearTimeout(copyTimeout)
		copyTimeout = window.setTimeout(() => {
			copied.value = false
		}, 2000)
	} catch (e) {
		console.error('Failed to copy screenshot to clipboard:', e)
	}
}

async function handleOpen() {
	if (!latestScreenshot.value) return
	const key: ScreenshotKey = {
		instance_id: latestScreenshot.value.instance_id,
		file_name: latestScreenshot.value.file_name,
	}
	try {
		await open_screenshot(key)
	} catch (e) {
		console.error('Failed to open screenshot:', e)
	}
}

function goToGallery() {
	void router.push('/screenshots')
}

onMounted(() => {
	void fetchLatestScreenshot()
})

useAppEvent('instance', fetchLatestScreenshot)
</script>

<template>
	<div class="p-4 border-0 border-b-[1px] border-[--brand-gradient-border] border-solid flex flex-col gap-3">
		<div class="flex items-center justify-between">
			<div class="flex items-center gap-1.5 text-primary font-medium text-base">
				<ImagesIcon class="w-4 h-4 text-brand" />
				<span>Quick Gallery</span>
			</div>
			<button
				type="button"
				class="text-xs text-brand hover:underline bg-transparent border-0 cursor-pointer flex items-center gap-1 p-0"
				@click="goToGallery"
			>
				View all
				<ArrowUpRightIcon class="w-3 h-3" />
			</button>
		</div>

		<div
			v-if="latestScreenshot"
			class="group relative rounded-xl overflow-hidden border border-solid border-surface-5 bg-surface-2 transition-all hover:border-brand/50 shadow-sm"
		>
			<div class="aspect-video w-full bg-black/40 overflow-hidden relative cursor-pointer" @click="handleOpen">
				<img
					:src="latestScreenshot.url || convertFileSrc(latestScreenshot.path)"
					:alt="latestScreenshot.file_name"
					class="w-full h-full object-cover transition-transform duration-300 group-hover:scale-105"
					loading="lazy"
				/>
				<div
					class="absolute inset-0 bg-gradient-to-t from-black/80 via-black/20 to-transparent opacity-0 group-hover:opacity-100 transition-opacity flex items-end p-2 justify-between"
				>
					<span class="text-[11px] text-white/90 truncate max-w-[140px] font-medium">
						{{ latestScreenshot.file_name }}
					</span>
					<div class="flex items-center gap-1">
						<IconButton
							type="colored"
							color="brand"
							size="sm"
							:label="copied ? 'Copied!' : 'Copy to clipboard'"
							@click.stop="handleCopy"
						>
							<CheckIcon v-if="copied" class="w-3.5 h-3.5 text-contrast" />
							<ClipboardCopyIcon v-else class="w-3.5 h-3.5" />
						</IconButton>
					</div>
				</div>
			</div>
			<div class="px-2.5 py-1.5 flex items-center justify-between text-xs text-secondary bg-surface-1">
				<span class="truncate">{{ latestScreenshot.instance_name || latestScreenshot.instance_id }}</span>
				<span class="shrink-0 text-[10px] opacity-75">
					{{ formatTime(getScreenshotDate(latestScreenshot)) }}
				</span>
			</div>
		</div>

		<div
			v-else-if="!loading"
			class="rounded-xl border border-dashed border-surface-4 p-3.5 flex flex-col items-center justify-center text-center gap-2 bg-surface-1/50"
		>
			<div class="w-8 h-8 rounded-full bg-brand/10 text-brand flex items-center justify-center">
				<ImagesIcon class="w-4 h-4" />
			</div>
			<div class="flex flex-col gap-0.5">
				<span class="text-xs font-medium text-primary">No screenshots yet</span>
				<span class="text-[11px] text-secondary">Press <kbd class="px-1 py-0.5 rounded bg-surface-3 text-[10px] font-mono">F2</kbd> in Minecraft to capture!</span>
			</div>
		</div>

		<!-- Quick Shortcuts Grid -->
		<div class="grid grid-cols-2 gap-2 pt-1">
			<button
				type="button"
				class="flex items-center gap-2 p-2 rounded-xl bg-surface-2 hover:bg-surface-3 border border-surface-4 text-xs font-medium text-primary hover:text-contrast transition-colors cursor-pointer text-left"
				@click="goToGallery"
			>
				<ImagesIcon class="w-4 h-4 text-brand shrink-0" />
				<span class="truncate">Gallery</span>
			</button>
			<button
				type="button"
				class="flex items-center gap-2 p-2 rounded-xl bg-surface-2 hover:bg-surface-3 border border-surface-4 text-xs font-medium text-primary hover:text-contrast transition-colors cursor-pointer text-left"
				@click="() => router.push('/skins')"
			>
				<ShirtIcon class="w-4 h-4 text-brand shrink-0" />
				<span class="truncate">Skins</span>
			</button>
		</div>
	</div>
</template>
