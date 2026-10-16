<script setup lang="ts">
import type { Labrinth } from '@modrinth/api-client'
import { RotateCounterClockwiseIcon, SearchIcon } from '@modrinth/assets'
import { computed, ref, toValue } from 'vue'
import { useRouter } from 'vue-router'

import Admonition from '#ui/components/base/Admonition.vue'
import { Button, IconButton } from '#ui/components/base/buttons'
import Combobox, { type ComboboxOption } from '#ui/components/base/Combobox.vue'
import Input from '#ui/components/base/inputs/Input.vue'
import LoadingIndicator from '#ui/components/base/LoadingIndicator.vue'
import NavTabs from '#ui/components/base/NavTabs.vue'
import Pagination from '#ui/components/base/Pagination.vue'
import ProjectCard from '#ui/components/project/card/ProjectCard.vue'
import ProjectCardList from '#ui/components/project/ProjectCardList.vue'
import SearchFilterControl from '#ui/components/search/SearchFilterControl.vue'
import { defineMessages, useVIntl } from '#ui/composables/i18n'
import { useStickyObserver } from '#ui/composables/sticky-observer'
import { commonMessages, formatProjectTypeSentence } from '#ui/utils/common-messages'
import type { SortType } from '#ui/utils/search'

import SelectedProjectsFloatingBar from './components/SelectedProjectsFloatingBar.vue'
import BrowseInstallHeader from './header.vue'
import { injectBrowseManager } from './providers/browse-manager'
import type { CardAction } from './types'

const ctx = injectBrowseManager()
const { formatMessage } = useVIntl()
const lockedMessages = computed(() => toValue(ctx.lockedFilterMessages))
const stickyInstallHeaderRef = ref<HTMLElement | null>(null)
const { isStuck: isInstallHeaderStuck } = useStickyObserver(
	stickyInstallHeaderRef,
	'BrowseInstallHeader',
)

const sortOptions = computed<ComboboxOption<SortType>[]>(() =>
	ctx.effectiveSortTypes.value.map((st) => ({
		value: st,
		label: st.display,
	})),
)

const maxResultsOptions = computed<ComboboxOption<number>[]>(() =>
	(ctx.maxResultsOptions?.value ?? [5, 10, 15, 20, 50, 100]).map((n) => ({
		value: n,
		label: String(n),
	})),
)

const router = useRouter()

const sourceOptions: ComboboxOption<string>[] = [
	{ value: 'modrinth', label: 'Modrinth' },
	{ value: 'ftb', label: 'Feed The Beast (FTB)' },
]

const currentSourceOption = computed(() => {
	const current = ctx.source?.value ?? 'modrinth'
	return sourceOptions.find((opt) => opt.value === current) ?? sourceOptions[0]
})

const messages = defineMessages({
	searchPlaceholder: {
		id: 'browse.search.placeholder',
		defaultMessage: 'Search {projectType}...',
	},
	viewPrefix: {
		id: 'browse.view-prefix',
		defaultMessage: 'View:',
	},
	filterResults: {
		id: 'browse.filter-results',
		defaultMessage: 'Filter results...',
	},
	offline: {
		id: 'browse.offline',
		defaultMessage: 'You are currently offline. Connect to the internet to browse Modrinth!',
	},
	noResults: {
		id: 'browse.no-results',
		defaultMessage: 'No results found for your query!',
	},
	linkOverridingPreferences: {
		id: 'browse.advanced-filters.link-overriding-preferences',
		defaultMessage: "This link's filters differ from your saved advanced exclusions",
	},
	applySavedPreferences: {
		id: 'browse.advanced-filters.apply-saved-preferences',
		defaultMessage: 'Apply saved preferences',
	},
})

function cardActionType(action: CardAction) {
	if (action.type === 'transparent') return 'quiet'
	if (action.type === 'outlined') return 'outlined'
	return action.color && action.color !== 'standard' ? 'colored' : 'base'
}

function cardActionColor(action: CardAction) {
	const type = cardActionType(action)
	return type === 'colored' || type === 'quiet' ? action.color : undefined
}

function cardActionClass(action: CardAction) {
	if (action.type !== 'outlined' || !action.color || action.color === 'standard') return undefined

	return {
		brand: '!text-brand [&>svg]:!text-brand !shadow-[inset_0_0_0_1px_var(--color-brand)]',
		red: '!text-red [&>svg]:!text-red !shadow-[inset_0_0_0_1px_var(--color-red)]',
		green: '!text-green [&>svg]:!text-green !shadow-[inset_0_0_0_1px_var(--color-green)]',
	}[action.color]
}

function getLoaderFieldValues(
	result: Labrinth.Search.v3.ResultSearchProject,
	field: string,
): string[] {
	return (result.project_loader_fields?.[field] ?? []).filter(
		(value): value is string => typeof value === 'string',
	)
}

function getProjectCardTags(result: Labrinth.Search.v3.ResultSearchProject, displayOnly: boolean) {
	const tags = new Set(displayOnly ? result.display_categories : result.categories)

	for (const loader of result.loaders) {
		if (loader !== 'mrpack') {
			tags.add(loader)
		}
	}

	if (result.loaders.includes('mrpack')) {
		for (const loader of getLoaderFieldValues(result, 'mrpack_loaders')) {
			tags.add(loader)
		}
	}

	return Array.from(tags)
}
</script>

<template>
	<template v-if="ctx.installContext?.value && ctx.variant !== 'web'">
		<div
			ref="stickyInstallHeaderRef"
			class="sticky top-0 z-20 -mx-6 -mt-6 rounded-tl-[--radius-xl] border-0 border-b border-solid bg-surface-1 px-6 py-4 border-surface-5"
			:class="[isInstallHeaderStuck ? 'border-t' : '']"
		>
			<BrowseInstallHeader />
		</div>
	</template>
	<SelectedProjectsFloatingBar v-if="ctx.installContext?.value && ctx.variant !== 'web'" />

	<div class="flex flex-wrap items-center justify-between gap-4">
		<NavTabs
			v-if="ctx.showProjectTypeTabs.value"
			:links="ctx.selectableProjectTypes.value"
			:replace="ctx.variant === 'app'"
		/>

		<!-- Source Switcher (Modrinth / FTB) -->
		<div v-if="ctx.source" class="flex items-center gap-2">
			<span class="text-xs font-semibold text-secondary uppercase tracking-wider">Source:</span>
			<div class="inline-flex rounded-full bg-surface-2 p-1 border border-solid border-surface-4">
				<button
					type="button"
					v-for="opt in sourceOptions"
					:key="opt.value"
					@click="ctx.setSource?.(opt.value)"
					:class="[
						'px-3.5 py-1.5 rounded-full text-xs font-bold transition-all cursor-pointer border-0',
						(ctx.source?.value ?? 'modrinth') === opt.value
							? 'bg-brand text-brand-inverted shadow-sm'
							: 'bg-transparent text-secondary hover:text-primary hover:bg-surface-3'
					]"
				>
					{{ opt.label }}
				</button>
			</div>
		</div>
	</div>

	<Input
		v-model="ctx.query.value"
		:icon="SearchIcon"
		type="text"
		autocomplete="off"
		:placeholder="
			formatMessage(messages.searchPlaceholder, {
				projectType: formatProjectTypeSentence(formatMessage, ctx.projectType.value, 2),
			})
		"
		clearable
		wrapper-class="w-full"
		size="large"
		@clear="ctx.clearSearch()"
	/>

	<Admonition
		v-if="ctx.linkOverridesAdvancedPrefs.value"
		type="info"
		:header="formatMessage(messages.linkOverridingPreferences)"
		inline-actions
		center-content
	>
		<template #actions>
			<Button type="colored" color="blue" @click="ctx.applySavedAdvancedPrefs()">
				<RotateCounterClockwiseIcon />
				{{ formatMessage(messages.applySavedPreferences) }}
			</Button>
		</template>
	</Admonition>

	<div class="flex flex-wrap items-center gap-2">
		<Combobox
			:model-value="ctx.effectiveCurrentSortType.value"
			:options="sortOptions"
			trigger-type="base"
			:class="
				ctx.variant === 'web'
					? '!w-[16rem] min-w-max max-w-full flex-grow md:flex-grow-0'
					: '!w-[16rem] min-w-max max-w-full'
			"
			@update:model-value="(val: SortType) => (ctx.effectiveCurrentSortType.value = val)"
		>
			<template #prefix>
				<span class="font-semibold text-primary">{{
					formatMessage(commonMessages.sortByLabel)
				}}</span>
			</template>
		</Combobox>

		<Combobox
			:model-value="ctx.maxResults.value"
			:options="maxResultsOptions"
			trigger-type="base"
			:class="
				ctx.variant === 'web'
					? '!w-[9rem] min-w-max max-w-full flex-grow md:flex-grow-0'
					: '!w-[9rem] min-w-max max-w-full'
			"
			:placeholder="formatMessage(commonMessages.viewLabel)"
			@update:model-value="(val: number) => (ctx.maxResults.value = val)"
		>
			<template #prefix>
				<span class="font-semibold text-primary">{{ formatMessage(messages.viewPrefix) }}</span>
			</template>
		</Combobox>
		<Combobox
			v-if="ctx.source"
			:model-value="ctx.source?.value ?? 'modrinth'"
			:options="sourceOptions"
			trigger-type="base"
			:class="
				ctx.variant === 'web'
					? '!w-[13rem] min-w-max max-w-full flex-grow md:flex-grow-0'
					: '!w-[13rem] min-w-max max-w-full'
			"
			placeholder="Source"
			@update:model-value="(val: string) => ctx.setSource?.(val)"
		>
			<template #prefix>
				<span class="font-semibold text-primary">Source:</span>
			</template>
		</Combobox>

		<div v-if="ctx.filtersMenuOpen && !ctx.filtersMenuOpen.value" class="lg:hidden">
			<Button @click="ctx.filtersMenuOpen.value = true">
				{{ formatMessage(messages.filterResults) }}
			</Button>
		</div>

		<IconButton
			v-if="ctx.cycleDisplayMode"
			label="Change display mode"
			@click="ctx.cycleDisplayMode!()"
		>
			<slot name="display-mode-icon" />
		</IconButton>

		<Pagination
			:page="ctx.currentPage.value"
			:count="ctx.pageCount.value"
			:class="ctx.variant === 'web' ? 'mx-auto sm:ml-auto sm:mr-0' : 'ml-auto'"
			@switch-page="ctx.setPage"
		/>
	</div>

	<SearchFilterControl v-if="(!ctx.source?.value || ctx.source.value === 'modrinth') && ctx.isServerType.value"
		v-model:selected-filters="ctx.serverCurrentFilters.value"
		:filters="ctx.serverFilterTypes.value"
		:project-type="ctx.projectType.value"
		:provided-filters="[]"
		:overridden-provided-filter-types="[]"
	/>
	<SearchFilterControl v-else-if="!ctx.source?.value || ctx.source.value === 'modrinth'"
		v-model:selected-filters="ctx.currentFilters.value"
		:filters="
			ctx.filters.value.filter(
				(f) => f.display !== 'none' && !(ctx.hiddenFilterTypes?.value ?? []).includes(f.id),
			)
		"
		:project-type="ctx.projectType.value"
		:provided-filters="ctx.providedFilters?.value ?? []"
		:overridden-provided-filter-types="ctx.overriddenProvidedFilterTypes.value"
		:provided-message="lockedMessages?.providedBy"
	/>

	<div class="search mt-1 [overflow-anchor:none]">
		<section v-if="ctx.loading.value" class="offline">
			<component :is="ctx.loadingComponent ?? LoadingIndicator" />
		</section>
		<section v-else-if="ctx.offline?.value && ctx.totalHits.value === 0" class="offline">
			{{ formatMessage(messages.offline) }}
		</section>
		<section
			v-else-if="
				ctx.isServerType.value
					? ctx.serverHits.value.length === 0
					: ctx.projectHits.value.length === 0
			"
			class="offline"
		>
			<p>{{ formatMessage(messages.noResults) }}</p>
		</section>

		<div
			v-if="ctx.source?.value === 'ftb' && ctx.projectType.value !== 'modpack'"
			class="my-8 flex flex-col items-center justify-center rounded-2xl bg-surface-2 p-8 text-center border border-surface-5"
		>
			<div class="mb-4 flex h-16 w-16 items-center justify-center rounded-full bg-brand/10 text-brand text-3xl">
				📦
			</div>
			<h3 class="text-xl font-bold text-primary mb-2">
				FTB does not support adding {{ formatProjectTypeSentence(formatMessage, ctx.projectType.value, 2) }}
			</h3>
			<p class="max-w-md text-secondary text-sm mb-6">
				Feed The Beast exclusively curates and distributes complete Modpacks. To browse standalone mods, shaders, resource packs, or data packs, switch the source to Modrinth.
			</p>
			<div class="flex flex-wrap gap-3 justify-center">
				<Button type="colored" color="brand" @click="ctx.setSource?.('modrinth')">
					Switch to Modrinth
				</Button>
				<Button type="base" @click="router.push('/browse/modpack')">
					Browse FTB Modpacks
				</Button>
			</div>
		</div>

		<ProjectCardList v-else :layout="ctx.effectiveLayout.value">
			<template v-if="ctx.isServerType.value">
				<ProjectCard
					v-for="result in ctx.serverHits.value"
					:key="`server-card-${result.project_id}`"
					:title="result.name"
					:icon-url="result.icon_url || undefined"
					:summary="result.summary"
					:tags="result.categories"
					:link="ctx.getServerProjectLink(result)"
					:server-online-players="result.minecraft_java_server?.ping?.data?.players_online ?? 0"
					:server-region="result.minecraft_server?.region"
					:server-recent-plays="result.minecraft_java_server?.verified_plays_2w ?? 0"
					:server-modpack-content="ctx.getServerModpackContent?.(result)"
					:server-ping="ctx.serverPings?.value?.[result.project_id]"
					:server-status-online="!!result.minecraft_java_server?.ping?.data"
					:hide-online-players-label="ctx.variant === 'app'"
					:hide-recent-plays-label="ctx.variant === 'app'"
					:layout="ctx.effectiveLayout.value"
					:max-tags="2"
					is-server-project
					exclude-loaders
					:color="result.color ?? undefined"
					:banner="result.featured_gallery ?? undefined"
					@contextmenu.prevent.stop="(event: MouseEvent) => ctx.onContextMenu?.(event, result)"
					@mouseenter="ctx.onServerProjectHover?.(result)"
					@mouseleave="ctx.onProjectHoverEnd?.()"
				>
					<template v-if="ctx.getCardActions?.(result, ctx.projectType.value)?.length" #actions>
						<div class="flex gap-2">
							<template
								v-for="action in ctx.getCardActions(result, ctx.projectType.value)"
								:key="action.key"
							>
								<IconButton
									v-if="action.circular"
									v-tooltip="action.tooltip"
									:type="cardActionType(action)"
									:color="cardActionColor(action)"
									:class="cardActionClass(action)"
									:label="action.label || action.tooltip || action.key"
									:disabled="action.disabled"
									@click.stop="action.onClick"
								>
									<component :is="action.icon" :class="action.iconClass" />
								</IconButton>
								<Button
									v-else
									v-tooltip="action.tooltip"
									:type="cardActionType(action)"
									:color="cardActionColor(action)"
									:class="cardActionClass(action)"
									:disabled="action.disabled"
									@click.stop="action.onClick"
								>
									<component :is="action.icon" :class="action.iconClass" />
									{{ action.label }}
								</Button>
							</template>
						</div>
					</template>
				</ProjectCard>
			</template>
			<template v-else>
				<ProjectCard
					v-for="result in ctx.projectHits.value"
					:key="result.project_id"
					:link="ctx.getProjectLink(result)"
					:title="result.name"
					:icon-url="result.icon_url ?? undefined"
					:author="{
						name: result.organization == null ? result.author : result.organization,
						link:
							result.organization_id == null
								? `/user/${encodeURIComponent(result.author_id ?? result.author)}`
								: ctx.variant === 'web'
									? `/organization/${result.organization_id}`
									: `https://modrinth.com/organization/${result.organization_id}`,
					}"
					:date-updated="result.date_modified"
					:date-published="result.date_created"
					:displayed-date="
						ctx.effectiveCurrentSortType.value.name === 'newest' ? 'published' : 'updated'
					"
					:downloads="result.downloads"
					:summary="result.summary"
					:tags="getProjectCardTags(result, true)"
					:all-tags="getProjectCardTags(result, false)"
					:deprioritized-tags="ctx.deprioritizedTags.value"
					:exclude-loaders="ctx.excludeLoaders.value"
					:followers="result.follows"
					:banner="result.featured_gallery ?? undefined"
					:color="result.color ?? undefined"
					:environment="
						['mod', 'modpack'].includes(ctx.projectType.value)
							? result.project_loader_fields?.environment?.[0]
							: undefined
					"
					:layout="ctx.effectiveLayout.value"
					@contextmenu.prevent.stop="(event: MouseEvent) => ctx.onContextMenu?.(event, result)"
					@mouseenter="ctx.onProjectHover?.(result)"
					@mouseleave="ctx.onProjectHoverEnd?.()"
				>
					<template v-if="ctx.getCardActions?.(result, ctx.projectType.value)?.length" #actions>
						<div class="flex gap-2">
							<template
								v-for="action in ctx.getCardActions(result, ctx.projectType.value)"
								:key="action.key"
							>
								<IconButton
									v-if="action.circular"
									v-tooltip="action.tooltip"
									:type="cardActionType(action)"
									:color="cardActionColor(action)"
									:class="cardActionClass(action)"
									:label="action.label || action.tooltip || action.key"
									:disabled="action.disabled"
									@click.stop="action.onClick"
								>
									<component :is="action.icon" :class="action.iconClass" />
								</IconButton>
								<Button
									v-else
									v-tooltip="action.tooltip"
									:type="cardActionType(action)"
									:color="cardActionColor(action)"
									:class="cardActionClass(action)"
									:disabled="action.disabled"
									@click.stop="action.onClick"
								>
									<component :is="action.icon" :class="action.iconClass" />
									{{ action.label }}
								</Button>
							</template>
						</div>
					</template>
				</ProjectCard>
			</template>
		</ProjectCardList>

		<div :class="ctx.variant === 'web' ? 'pagination-after mt-3' : 'flex justify-end mt-3'">
			<Pagination
				:page="ctx.currentPage.value"
				:count="ctx.pageCount.value"
				:class="ctx.variant === 'web' ? 'justify-end' : 'pagination-after'"
				@switch-page="ctx.setPage"
			/>
		</div>
	</div>

	<slot name="after" />
</template>
