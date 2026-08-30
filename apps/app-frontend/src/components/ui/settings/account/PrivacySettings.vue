<script setup lang="ts">
import { ShieldCheckIcon } from '@modrinth/assets'
import {
	defineMessages,
	Toggle,
	useVIntl,
} from '@modrinth/ui'
import { ref, watch } from 'vue'

import { get, set } from '@/helpers/settings.ts'

const { formatMessage } = useVIntl()
const settings = ref(await get())

const messages = defineMessages({
	privacyTitle: {
		id: 'app.settings.privacy.guarantee.title',
		defaultMessage: 'PawsMC Privacy Guarantee',
	},
	privacyDescription: {
		id: 'app.settings.privacy.guarantee.description',
		defaultMessage:
			'PawsMC is 100% ad-free, telemetry-free, and tracker-free. No analytical or tracking data is ever collected or transmitted to third-party servers.',
	},
	discordRichPresenceTitle: {
		id: 'app.settings.privacy.discord-rich-presence.title',
		defaultMessage: 'Discord Rich Presence',
	},
	discordRichPresenceDescription: {
		id: 'app.settings.privacy.discord-rich-presence.description',
		defaultMessage:
			'Show PawsMC as your current activity on Discord. This does not affect Rich Presence added to instances by mods. Requires an app restart.',
	},
})

watch(
	settings,
	async () => {
		await set(settings.value)
	},
	{ deep: true },
)
</script>

<template>
	<div class="flex flex-col gap-6">
		<div class="p-4 rounded-2xl bg-surface-2 border border-solid border-surface-4 flex items-start gap-3.5">
			<div class="w-10 h-10 rounded-xl bg-brand/10 text-brand flex items-center justify-center shrink-0 mt-0.5">
				<ShieldCheckIcon class="w-5 h-5" />
			</div>
			<div class="flex flex-col gap-1">
				<h2 class="m-0 text-base font-semibold text-contrast">
					{{ formatMessage(messages.privacyTitle) }}
				</h2>
				<p class="m-0 text-sm text-secondary leading-relaxed">
					{{ formatMessage(messages.privacyDescription) }}
				</p>
			</div>
		</div>

		<div class="flex items-center justify-between gap-4 p-4 rounded-2xl bg-surface-1 border border-solid border-surface-4">
			<div>
				<h2 class="m-0 text-base font-semibold text-contrast">
					{{ formatMessage(messages.discordRichPresenceTitle) }}
				</h2>
				<p class="m-0 mt-1 text-sm text-secondary">
					{{ formatMessage(messages.discordRichPresenceDescription) }}
				</p>
			</div>
			<Toggle id="disable-discord-rpc" v-model="settings.discord_rpc" />
		</div>
	</div>
</template>
