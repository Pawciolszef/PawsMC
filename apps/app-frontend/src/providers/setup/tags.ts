import type { AbstractWebNotificationManager } from '@modrinth/ui'
import { provideTags } from '@modrinth/ui'
import { ref } from 'vue'

import { get_game_versions, get_loaders } from '@/helpers/tags'

export function setupTagsProvider(_notificationManager: AbstractWebNotificationManager) {
	const gameVersions = ref([])
	const loaders = ref([])

	async function loadTags() {
		try {
			const [v, l] = await Promise.all([get_game_versions(), get_loaders()])
			gameVersions.value = v
			loaders.value = l
		} catch {
			setTimeout(loadTags, 400)
		}
	}

	void loadTags()

	provideTags({ gameVersions, loaders })
}
