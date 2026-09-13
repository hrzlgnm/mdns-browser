// tauri/tooling/api/src/core.ts
function transformCallback(callback, once = false) {
	return window.__TAURI_INTERNALS__.transformCallback(callback, once)
}
async function invoke(cmd, args = {}) {
	// NB: `options` ignored as not used here.
	return window.__TAURI_INTERNALS__.invoke(cmd, args)
}

// tauri/tooling/api/src/event.ts
async function _unlisten(event, eventId) {
	await invoke('plugin:event|unlisten', {
		event,
		eventId
	})
}
async function emit(event, payload) {
	await invoke('plugin:event|emit', {
		event,
		payload
	})
}
async function emitTo(
	target,
	event,
	payload
) {
	await invoke('plugin:event|emit_to', {
		target,
		event,
		payload
	})
}
async function listen(event, handler, options) {
	const target =
		typeof options?.target === 'string'
			? { kind: 'AnyLabel', label: options.target }
			: options?.target ?? { kind: 'Any' }
	return invoke('plugin:event|listen', {
		event,
		target,
		handler: transformCallback(handler)
	}).then((eventId) => {
		return async () => _unlisten(event, eventId)
	})
}
async function once(event, handler, options) {
	return listen(
		event,
		(eventData) => {
			handler(eventData)
			_unlisten(event, eventData.id).catch(() => { })
		},
		options
	)
}

export {
	emit,
	emitTo,
	listen,
	once,
};
