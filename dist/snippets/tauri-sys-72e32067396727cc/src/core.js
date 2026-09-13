// tauri/tooling/api/src/core.ts
function transformCallback(callback, once = false) {
	return window.__TAURI_INTERNALS__.transformCallback(callback, once)
}
async function invoke(cmd, args = {}) {
	// NB: `options` ignored as not used here.
	return window.__TAURI_INTERNALS__.invoke(cmd, args)
}
function convertFileSrc(filePath, protocol = 'asset') {
	return window.__TAURI_INTERNALS__.convertFileSrc(filePath, protocol)
}
function isTauri() {
	return 'isTauri' in window && !!window.isTauri
}
export {
	invoke,
	convertFileSrc,
	transformCallback,
	isTauri,
}
