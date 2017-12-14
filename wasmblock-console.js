WasmBlock((module) => ({
  console_log: function(strPtr) {
    let result = module.$copyCStr(strPtr);
    console.log(result);
  },
  console_error: function(strPtr) {
    let result = module.$copyCStr(strPtr);
    console.error(result);
  },
  console_info: function(strPtr) {
    let result = module.$copyCStr(strPtr);
    console.info(result);
  },
  console_debug: function(strPtr) {
    let result = module.$copyCStr(strPtr);
    console.debug(result);
  },
  console_clear: function(strPtr) {
    console.clear();
  },
  console_time: function() {
    console.time();
  },
  console_time_end: function() {
    console.timeEnd();
  }
}))
