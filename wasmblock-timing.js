WasmBlock((module) => ({
  request_animation_frame: function(strPtr) {
    let funcName = module.$copyCStr(strPtr);
    window.requestAnimationFrame(function(){
      module[funcName]();
    });
  },
  set_timeout: function(strPtr,milliseconds) {
    let funcName = module.$copyCStr(strPtr);
    window.setTimeout(function(){
      module[funcName]();
    },milliseconds);
  }
}))
