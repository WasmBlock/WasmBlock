WebBlocks.extensions.console = function(Module){
  return {
    console_log: function(outptr) {
      let result = Module.$copyCStr(Module, outptr);
      console.log(result);
    }
  }
}
