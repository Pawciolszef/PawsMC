import 'floating-vue/dist/style.css'
import 'overlayscrollbars/overlayscrollbars.css'

// Trackers disabled for PawsMC

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

app.mount('#app')
