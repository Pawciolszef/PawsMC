// Telemetry and trackers completely disabled for PawsMC

export type AnalyticsEvent = string

export const initAnalytics = () => {}
export const debugAnalytics = () => {}
export const optOutAnalytics = () => {}
export const optInAnalytics = () => {}

export const trackEvent = (
	_eventName: string,
	..._args: any[]
) => {}
