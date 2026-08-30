<script setup lang="ts">
import {
	PlayIcon,
	SparklesIcon,
	SpinnerIcon,
	StopCircleIcon,
} from '@modrinth/assets'
import { Avatar, Button, useFormatDateTime } from '@modrinth/ui'
import dayjs from 'dayjs'
import { computed, onMounted, ref } from 'vue'
import { useRouter } from 'vue-router'

import { useAppEvent } from '@/composables/use-app-event'
import {
	getInstanceIconUrl,
	kill,
	list,
	run,
	type GameInstance,
} from '@/helpers/instance'
import { get_all, type ProcessMetadata } from '@/helpers/process'

const router = useRouter()
const instances = ref<GameInstance[]>([])
const runningProcesses = ref<ProcessMetadata[]>([])
const loading = ref(false)
const fetching = ref(true)

const formatTime = useFormatDateTime({ dateStyle: 'short', timeStyle: 'short' })

const mostRecentInstance = computed<GameInstance | null>(() => {
	if (!instances.value || instances.value.length === 0) return null
	const sorted = instances.value.slice().sort((a, b) => {
		const timeA = dayjs(a.last_played ?? a.created).valueOf()
		const timeB = dayjs(b.last_played ?? b.created).valueOf()
		return timeB - timeA
	})
	return sorted[0] ?? null
})

const isPlaying = computed(() => {
	if (!mostRecentInstance.value) return false
	return runningProcesses.value.some((p) => p.instance_id === mostRecentInstance.value?.id)
})

async function refreshData() {
	try {
		fetching.value = true
		const [allInstances, processes] = await Promise.all([
			list().catch(() => []),
			get_all().catch(() => []),
		])
		instances.value = allInstances
		runningProcesses.value = processes
	} catch (e) {
		console.warn('Failed to fetch data for QuickLaunchCard:', e)
	} finally {
		fetching.value = false
	}
}

async function handleAction(e: Event) {
	e.stopPropagation()
	if (!mostRecentInstance.value) return

	if (isPlaying.value) {
		try {
			await kill(mostRecentInstance.value.id)
		} catch (err) {
			console.error('Failed to stop instance:', err)
		}
	} else {
		try {
			loading.value = true
			await run(mostRecentInstance.value.id)
		} catch (err) {
			console.error('Failed to start instance:', err)
		} finally {
			loading.value = false
		}
	}
	await refreshData()
}

function openInstance() {
	if (!mostRecentInstance.value) return
	void router.push(`/instance/${encodeURIComponent(mostRecentInstance.value.id)}`)
}

onMounted(() => {
	void refreshData()
})

useAppEvent('instance', refreshData)
useAppEvent('process', refreshData)
</script>

<template>
	<div
		v-if="mostRecentInstance"
		class="p-4 border-0 border-b-[1px] border-[--brand-gradient-border] border-solid flex flex-col gap-2.5"
	>
		<div class="flex items-center justify-between">
			<div class="flex items-center gap-1.5 text-primary font-medium text-base">
				<PlayIcon class="w-4 h-4 text-brand" />
				<span>Quick Play</span>
			</div>
			<span
				v-if="mostRecentInstance.last_played"
				class="text-[11px] text-secondary"
			>
				{{ formatTime(new Date(mostRecentInstance.last_played)) }}
			</span>
			<span v-else class="text-[11px] text-secondary">
				Ready to play
			</span>
		</div>

		<div
			class="group relative flex items-center justify-between p-2.5 rounded-2xl bg-surface-2 hover:bg-surface-3 border border-solid border-surface-4 transition-all hover:border-brand/40 shadow-sm cursor-pointer"
			@click="openInstance"
		>
			<div class="flex items-center gap-2.5 min-w-0 pr-2">
				<Avatar
					:src="getInstanceIconUrl(mostRecentInstance.icon_path)"
					:tint-by="mostRecentInstance.id"
					size="40px"
					class="!rounded-xl shrink-0"
					no-shadow
					pad-transparent-corners
				/>
				<div class="flex flex-col min-w-0">
					<span class="text-sm font-semibold text-contrast truncate">
						{{ mostRecentInstance.name }}
					</span>
					<span class="text-xs text-secondary truncate flex items-center gap-1 capitalize">
						{{ mostRecentInstance.loader }} {{ mostRecentInstance.game_version }}
					</span>
				</div>
			</div>

			<div class="shrink-0" @click.stop>
				<Button
					v-if="isPlaying"
					type="colored"
					color="red"
					size="sm"
					class="!px-3 !py-1.5 font-medium"
					@click="handleAction"
				>
					<StopCircleIcon class="w-4 h-4" />
					Stop
				</Button>
				<Button
					v-else
					type="colored"
					color="brand"
					size="sm"
					:disabled="loading || mostRecentInstance.quarantined || mostRecentInstance.install_stage !== 'installed'"
					class="!px-3.5 !py-1.5 font-medium shadow-sm"
					@click="handleAction"
				>
					<SpinnerIcon v-if="loading" class="w-4 h-4 animate-spin" />
					<PlayIcon v-else class="w-4 h-4" />
					Play
				</Button>
			</div>
		</div>
	</div>
</template>
