WasmBlock((module) => ({
  wasmblock_dom_create_element: function(targetPtr,elPtr, idPtr) {
    let targetName = module.$copyCStr(targetPtr);
    let elementType = module.$copyCStr(elPtr);
    let id = module.$copyCStr(idPtr);
    var el = document.createElement(elementType);
    el.setAttribute('id',id);
    document.querySelector(targetName).append(el);
  },
  wasmblock_dom_set_attribute: function(targetPtr,attrPtr,valPtr) {
    let targetName = module.$copyCStr(targetPtr);
    let attrName = module.$copyCStr(attrPtr);
    let val = module.$copyCStr(valPtr);
    document.querySelector(targetName).setAttribute(attrName,val);
  },
  wasmblock_dom_set_inner_html: function(targetPtr,htmlPtr) {
    let targetName = module.$copyCStr(targetPtr);
    let html = module.$copyCStr(htmlPtr);
    document.querySelector(targetName).innerHTML = html;
  },
  wasmblock_dom_add_event_listener: function(targetPtr,eventPtr,callbackPtr) {
    let targetName = module.$copyCStr(targetPtr);
    let eventName = module.$copyCStr(eventPtr);
    let callbackName = module.$copyCStr(callbackPtr);
    document.querySelector(targetName).addEventListener(eventName,function(e){
      module[callbackName](module.$newString(e.target.id));
    });
  }
}))
