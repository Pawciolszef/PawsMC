// Advertisements completely disabled for PawsMC

export async function init_ads_window(_overrideShown = false) {
	return Promise.resolve()
}

export async function take_ads_window_hold() {
	return Promise.resolve()
}

export async function release_ads_window_hold() {
	return Promise.resolve()
}

export async function hide_ads_window(_reset) {
	return Promise.resolve()
}

export async function should_show_ads_consent_popup() {
	return Promise.resolve(false)
}

export async function perform_ads_consent_action(_action) {
	return Promise.resolve()
}

export async function open_ads_consent_preferences() {
	return Promise.resolve()
}

export async function record_ads_click() {
	return Promise.resolve()
}

export async function open_ads_link(_path, _origin) {
	return Promise.resolve()
}
