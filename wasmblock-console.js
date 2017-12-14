WasmBlock((Module) => ({
  console_log: function(strPtr) {
    let result = Module.$copyCStr(strPtr);
    console.log(result);
  }
}))
