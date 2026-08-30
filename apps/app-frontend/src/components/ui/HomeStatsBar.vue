<script setup lang="ts">
import { ClockIcon, LayersIcon, TimerIcon } from '@modrinth/assets'
import dayjs from 'dayjs'
import { computed } from 'vue'

import type { GameInstance } from '@/helpers/types'

const props = defineProps<{
	instances: GameInstance[]
}>()

function formatDuration(seconds: number): string {
	if (!seconds || seconds <= 0) return '0 min'
	const hours = Math.floor(seconds / 3600)
	const minutes = Math.floor((seconds % 3600) / 60)

	if (hours > 0 && minutes > 0) {
		return `${hours}h ${minutes}m`
	} else if (hours > 0) {
		return `${hours}h`
	} else if (minutes > 0) {
		return `${minutes} min`
	} else {
		return `${Math.floor(seconds)}s`
	}
}

// Total playtime across all instances
const totalPlaytimeSeconds = computed(() => {
	return props.instances.reduce((sum, inst) => {
		const submitted = inst.submitted_time_played || 0
		const recent = inst.recent_time_played || 0
		return sum + submitted + recent
	}, 0)
})

// Playtime today
const todayPlaytimeSeconds = computed(() => {
	const today = dayjs().startOf('day')
	return props.instances.reduce((sum, inst) => {
		if (inst.last_played && dayjs(inst.last_played).isAfter(today)) {
			return sum + (inst.recent_time_played || 0)
		}
		return sum
	}, 0)
})

// Created and installed count
const totalInstancesCount = computed(() => props.instances.length)
const installedInstancesCount = computed(
	() => props.instances.filter((i) => i.install_stage === 'installed').length,
)
</script>

<template>
	<div
		class="grid grid-cols-1 sm:grid-cols-3 gap-3 p-3.5 rounded-2xl bg-surface-2/80 border border-solid border-surface-4 shadow-sm backdrop-blur-sm"
	>
		<!-- Total Playtime -->
		<div class="flex items-center gap-3 p-2.5 rounded-xl bg-surface-1/60 border border-surface-5/50">
			<div class="w-10 h-10 rounded-xl bg-brand/10 text-brand flex items-center justify-center shrink-0">
				<TimerIcon class="w-5 h-5" />
			</div>
			<div class="flex flex-col min-w-0">
				<span class="text-xs text-secondary font-medium uppercase tracking-wider">Total Playtime</span>
				<span class="text-base font-bold text-contrast truncate">
					{{ formatDuration(totalPlaytimeSeconds) }}
				</span>
			</div>
		</div>

		<!-- Playtime Today -->
		<div class="flex items-center gap-3 p-2.5 rounded-xl bg-surface-1/60 border border-surface-5/50">
			<div class="w-10 h-10 rounded-xl bg-brand/10 text-brand flex items-center justify-center shrink-0">
				<ClockIcon class="w-5 h-5" />
			</div>
			<div class="flex flex-col min-w-0">
				<span class="text-xs text-secondary font-medium uppercase tracking-wider">Played Today</span>
				<span class="text-base font-bold text-contrast truncate">
					{{ formatDuration(todayPlaytimeSeconds) }}
				</span>
			</div>
		</div>

		<!-- Instances Count -->
		<div class="flex items-center gap-3 p-2.5 rounded-xl bg-surface-1/60 border border-surface-5/50">
			<div class="w-10 h-10 rounded-xl bg-brand/10 text-brand flex items-center justify-center shrink-0">
				<LayersIcon class="w-5 h-5" />
			</div>
			<div class="flex flex-col min-w-0">
				<span class="text-xs text-secondary font-medium uppercase tracking-wider">Instances</span>
				<div class="flex items-center gap-1.5">
					<span class="text-base font-bold text-contrast">
						{{ installedInstancesCount }}
					</span>
					<span class="text-xs text-secondary">
						installed / {{ totalInstancesCount }} total
					</span>
				</div>
			</div>
		</div>
	</div>
</template>
