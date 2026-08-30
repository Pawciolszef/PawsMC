import 'floating-vue/dist/style.css'
import 'overlayscrollbars/overlayscrollbars.css'

import { invoke } from '@tauri-apps/api/core'
import { VueScanPlugin } from '@taijased/vue-render-tracker'
import { VueQueryPlugin } from '@tanstack/vue-query'
import FloatingVue from 'floating-vue'
import { createApp } from 'vue'

import App from '@/App.vue'
import '@/composables/use-accent-color'
import { overlayScrollbarsDirective } from '@/directives/overlayScrollbars'
import i18nPlugin from '@/plugins/i18n'
import i18nDebugPlugin from '@/plugins/i18n-debug'
import router from '@/routes'

window.addEventListener('error', (e) => {
	console.error('[Global Error]', e)
	invoke('show_window').catch(() => {})
})
window.addEventListener('unhandledrejection', (e) => {
	console.error('[Unhandled Rejection]', e)
})

const vueScan = new VueScanPlugin({
	enabled: false,
	showOverlay: true,
	log: false,
	playSound: false,
})

const app = createApp(App)

app.config.errorHandler = (err, instance, info) => {
	console.error('[Vue Error]', err, info)
	invoke('show_window').catch(() => {})
}

app.use(VueQueryPlugin)
app.use(vueScan)
app.use(router)
app.use(FloatingVue, {
	themes: {
		'ribbit-popout': {
			$extend: 'dropdown',
			placement: 'bottom-end',
			instantMove: true,
			distance: 8,
		},
		'dismissable-prompt': {
			$extend: 'dropdown',
			placement: 'bottom-start',
		},
	},
})
app.use(i18nPlugin)
app.use(i18nDebugPlugin)
app.directive('overlay-scrollbars', overlayScrollbarsDirective)

try {
	app.mount('#app')
} catch (err) {
	console.error('[Mount Error]', err)
	invoke('show_window').catch(() => {})
}
