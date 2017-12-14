WasmBlock((module) => ({
  wasmblock_console_log: function(strPtr) {
    let result = module.$copyCStr(strPtr);
    console.log(result);
  },
  wasmblock_console_error: function(strPtr) {
    let result = module.$copyCStr(strPtr);
    console.error(result);
  },
  wasmblock_console_info: function(strPtr) {
    let result = module.$copyCStr(strPtr);
    console.info(result);
  },
  wasmblock_console_debug: function(strPtr) {
    let result = module.$copyCStr(strPtr);
    console.debug(result);
  },
  wasmblock_console_clear: function(strPtr) {
    console.clear();
  },
  wasmblock_console_time: function() {
    console.time();
  },
  wasmblock_console_time_end: function() {
    console.timeEnd();
  }
}))
