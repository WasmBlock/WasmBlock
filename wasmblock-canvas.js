WasmBlock((module) => {
  let contextStorage = []
  return ({
    wasmblock_canvas_get_context: function(targetPtr) {
      let targetName = module.$copyCStr(targetPtr);
      var ctx = document.querySelector(targetName).getContext('2d');
      var idx = contextStorage.length;
      contextStorage.push(ctx);
      return idx;
    },
    wasmblock_canvas_set_fill_style: function(ctx, stylePtr) {
      let style = module.$copyCStr(stylePtr);
      contextStorage[ctx].fillStyle = style;
    },
    wasmblock_canvas_set_fill_style_color: function(ctx, r,g,b,a) {
      contextStorage[ctx].fillStyle = 'rgba(' + [r,g,b,a].join() + ')';
    },
    wasmblock_canvas_fill_rect: function(ctx,x,y,width,height) {
      contextStorage[ctx].fillRect(x,y,width,height);
    }
})})
