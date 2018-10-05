const mod = (() => {
  let wasm;

  let cachedDecoder = new TextDecoder('utf-8');

  let cachegetUint8Memory = null;
  function getUint8Memory() {
      if (cachegetUint8Memory === null || cachegetUint8Memory.buffer !== wasm.memory.buffer) {
          cachegetUint8Memory = new Uint8Array(wasm.memory.buffer);
      }
      return cachegetUint8Memory;
  }

  function getStringFromWasm(ptr, len) {
      return cachedDecoder.decode(getUint8Memory().subarray(ptr, ptr + len));
  }

  function __wbg_alert_8c454b1ebc6068d7(arg0, arg1) {
      let varg0 = getStringFromWasm(arg0, arg1);
      alert(varg0);
  }
  /**
  * @returns {void}
  */
  function greet() {
      return wasm.greet();
  }

  let cachedGlobalArgumentPtr = null;
  function globalArgumentPtr() {
      if (cachedGlobalArgumentPtr === null) {
          cachedGlobalArgumentPtr = wasm.__wbindgen_global_argument_ptr();
      }
      return cachedGlobalArgumentPtr;
  }

  let cachegetUint32Memory = null;
  function getUint32Memory() {
      if (cachegetUint32Memory === null || cachegetUint32Memory.buffer !== wasm.memory.buffer) {
          cachegetUint32Memory = new Uint32Array(wasm.memory.buffer);
      }
      return cachegetUint32Memory;
  }
  /**
  * @param {number} arg0
  * @param {number} arg1
  * @returns {string}
  */
  function get_phrase_text(arg0, arg1) {
      const retptr = globalArgumentPtr();
      wasm.get_phrase_text(retptr, arg0, arg1);
      const mem = getUint32Memory();
      const rustptr = mem[retptr / 4];
      const rustlen = mem[retptr / 4 + 1];

      const realRet = getStringFromWasm(rustptr, rustlen).slice();
      wasm.__wbindgen_free(rustptr, rustlen * 1);
      return realRet;

  }

  const __wbg_now_4410283ed4cdb45a_target = Date.now.bind(Date) || function() {
      throw new Error(`wasm-bindgen: Date.now.bind(Date) does not exist`);
  };

  function __wbg_now_4410283ed4cdb45a() {
      return __wbg_now_4410283ed4cdb45a_target();
  }

  const importObject = {
      './bob_ross_lorem_ipsum_rust': {
          __wbg_alert_8c454b1ebc6068d7,
          __wbg_now_4410283ed4cdb45a,
      }
  };

  const inst = new WebAssembly.Instance(bobross, importObject);
  wasm = inst.exports;

  return {
      greet,
      get_phrase_text,
  };
})();

addEventListener('fetch', event => {
event.respondWith(handleRequest(event.request))
})

/**
* Fetch and log a request
* @param {Request} request
*/
async function handleRequest(request) {
console.log('Got request', request)

let phrases = mod.get_phrase_text(100, 10);

const response = new Response(phrases, {status: 200});
return response
}

