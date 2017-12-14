WebBlocks.extensions.console = function(Module){
  return {
    console_log: function(outptr) {
      let result = copyCStr(Module, outptr);
      Module.dealloc(outptr);
      console.log(result);
    }
  }
}
