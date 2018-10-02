curl -sv -X PUT "https://api.cloudflare.com/client/v4/accounts/b88135f6fd85583eb4dda76d022c1148/workers/scripts/bobross" -H "X-Auth-Email:spack@cloudflare.com" \
-H "X-Auth-Key:xxx" \
-F 'metadata=@worker/bobross-meta.json;type=application/json' \
-F 'wasmprogram=@pkg/bob_ross_lorem_ipsum_rust_bg.wasm;application/wasm' \
-F 'script=@worker/worker.js;type=application/javsacript'
