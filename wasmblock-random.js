WasmBlock((module) => ({
  wasmblock_random_get_seed: function() {
    return Math.random();
  }
}))
