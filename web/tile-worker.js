// Tile rendering Web Worker (ES Module)
// Each worker has its own WASM instance for parallel rendering

import init, { generate_biome_map, search_biome } from './pkg/mc_biome_finder_web.js';

let ready = false;

async function startup() {
    try {
        await init();
        ready = true;
        postMessage({ type: 'ready' });
    } catch (e) {
        postMessage({ type: 'error', error: e.message });
    }
}

self.onmessage = function(e) {
    const msg = e.data;

    if (msg.type === 'render-tile') {
        if (!ready) { postMessage({ type: 'tile-error', id: msg.id, error: 'not ready' }); return; }
        try {
            const rgba = generate_biome_map(
                BigInt(msg.seed), msg.version, false,
                msg.cx, msg.cz, msg.size, msg.size, msg.scale
            );
            // Copy to transferable ArrayBuffer
            const buf = new Uint8Array(rgba).buffer;
            postMessage({ type: 'tile-done', id: msg.id, rgba: buf, size: msg.size }, [buf]);
        } catch (e) {
            postMessage({ type: 'tile-error', id: msg.id, error: e.message });
        }
        return;
    }

    if (msg.type === 'search') {
        if (!ready) { postMessage({ type: 'search-error', error: 'not ready' }); return; }
        try {
            const t0 = performance.now();
            const results = search_biome(
                BigInt(msg.seed), msg.version, false,
                msg.biomeId, msg.windowSize,
                msg.originX, msg.originZ,
                msg.radius, msg.count
            );
            const elapsed = ((performance.now() - t0) / 1000).toFixed(2);
            postMessage({ type: 'search-done', results: results || [], elapsed });
        } catch (e) {
            postMessage({ type: 'search-error', error: e.message });
        }
        return;
    }
};

startup();