WasmBlock((module) => ({
  console_log: function(strPtr) {
    let result = module.$copyCStr(strPtr);
    console.log(result);
  }
}))
