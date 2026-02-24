// Tile rendering Web Worker (ES Module)
// Each worker has its own WASM instance for parallel rendering

let ready = false;
let generate_biome_map;
let search_biome_shard;

async function startup() {
    try {
        // Dynamic import — works reliably under any base path
        const base = new URL('./', import.meta.url).href;
        const mod = await import(base + 'pkg/mc_biome_finder_web.js');
        const wasmUrl = base + 'pkg/mc_biome_finder_web_bg.wasm';
        await mod.default({ module_or_path: wasmUrl });
        generate_biome_map = mod.generate_biome_map;
        search_biome_shard = mod.search_biome_shard;
        ready = true;
        postMessage({ type: 'ready' });
    } catch (e) {
        postMessage({ type: 'error', error: String(e && e.stack || e) });
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