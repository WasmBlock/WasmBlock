var WasmBlock = {
  extensions:{}
};

(function(){

function fetchAndInstantiate(url, importObject) {
  return fetch(url).then(response =>
    response.arrayBuffer()
  ).then(bytes =>
    WebAssembly.instantiate(bytes, importObject)
  ).then(results =>
    results.instance
  );
}

function copyCStr(module, ptr) {
  let orig_ptr = ptr;
  const collectCString = function* () {
    let memory = new Uint8Array(module.memory.buffer);
    while (memory[ptr] !== 0) {
      if (memory[ptr] === undefined) { throw new Error("Tried to read undef mem") }
      yield memory[ptr]
      ptr += 1
    }
  }

  const buffer_as_u8 = new Uint8Array(collectCString())
  const utf8Decoder = new TextDecoder("UTF-8");
  const buffer_as_utf8 = utf8Decoder.decode(buffer_as_u8);
  module.dealloc_str(orig_ptr);
  return buffer_as_utf8
}

function getStr(module, ptr, len) {
  const getData = function* (ptr, len) {
    let memory = new Uint8Array(module.memory.buffer);
    for (let index = 0; index < len; index++) {
      if (memory[ptr] === undefined) { throw new Error(`Tried to read undef mem at ${ptr}`) }
      yield memory[ptr + index]
    }
  }

  const buffer_as_u8 = new Uint8Array(getData(ptr/8, len/8));
  const utf8Decoder = new TextDecoder("UTF-8");
  const buffer_as_utf8 = utf8Decoder.decode(buffer_as_u8);
  return buffer_as_utf8;
}

function newString(module, str) {
  const utf8Encoder = new TextEncoder("UTF-8");
  let string_buffer = utf8Encoder.encode(str)
  let len = string_buffer.length
  let ptr = module.alloc(len+1)

  let memory = new Uint8Array(module.memory.buffer);
  for (i = 0; i < len; i++) {
    memory[ptr+i] = string_buffer[i]
  }

  memory[ptr+len] = 0;

  return ptr
}

"use strict";
class WasmModule extends HTMLElement {
   constructor() {
       super();
   }
   connectedCallback(){
     var entry = this.getAttribute("entry");
     if(!entry){
       console.error("You must provide an entry method for your web assembly module")
       return;
     }
     var src = this.getAttribute("src");
     if(!src){
       console.error("You must provide an wasm src path for your web assembly module")
       return;
     }

     let Module = {}

     // Initializing the memory with 20 pages (20 * 64KiB = 1.25 MiB)
     const memory = new WebAssembly.Memory({initial: 20});
     const imports = {
       env: {
         memory:memory
       }
     };

     for(var i = 0 ; i < this.attributes.length; i++){
       var attr = this.attributes[i];
       if(attr.name != "entry" && attr.name != "src" && attr.value === ""){
         imports.env = Object.assign(imports.env,WasmBlock.extensions[attr.name](Module));
       }
     }

     // On instantiation we pass the imports object
     fetchAndInstantiate("./helloworld.wasm", imports)
       .then(mod => {
         Module.$copyCStr   = copyCStr;
         Module.memory      = mod.exports.memory;
         Module.dealloc     = mod.exports.dealloc;
         Module.dealloc_str = mod.exports.dealloc_str;

         mod.exports[entry]();
       });

   }
}

customElements.define("wasm-module", WasmModule);


})()
