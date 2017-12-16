WasmBlock((module) => ({
  rasm_random_get_seed: function() {
    return Math.random();
  }
}))
