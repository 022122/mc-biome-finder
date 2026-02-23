// Tile rendering Web Worker (ES Module)
// Each worker has its own WASM instance for parallel rendering

import init, { generate_biome_map, search_biome_shard } from './pkg/mc_biome_finder_web.js';

let ready = false;

async function startup() {
    try {
        // Resolve WASM URL relative to this worker script
        const wasmUrl = new URL('./pkg/mc_biome_finder_web_bg.wasm', import.meta.url);
        await init({ module_or_path: wasmUrl });
        ready = true;
        postMessage({ type: 'ready' });
    } catch (e) {
        postMessage({ type: 'error', error: String(e) });
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
            const buf = new Uint8Array(rgba).buffer;
            postMessage({ type: 'tile-done', id: msg.id, rgba: buf, size: msg.size }, [buf]);
        } catch (e) {
            postMessage({ type: 'tile-error', id: msg.id, error: e.message });
        }
        return;
    }

    if (msg.type === 'search-shard') {
        if (!ready) { postMessage({ type: 'shard-error', shardId: msg.shardId, error: 'not ready' }); return; }
        try {
            const results = search_biome_shard(
                BigInt(msg.seed), msg.version, false,
                msg.biomeId, msg.windowSize,
                msg.originX, msg.originZ,
                msg.radius, msg.bzStart, msg.bzEnd
            );
            postMessage({ type: 'shard-done', shardId: msg.shardId, results: results || [] });
        } catch (e) {
            postMessage({ type: 'shard-error', shardId: msg.shardId, error: e.message });
        }
        return;
    }
};

startup();