# Todo App

This actor implements the [Todo backend spec](https://github.com/TodoBackend/todo-backend-js-spec/blob/master/js/specs.js).

This actor makes use of the HTTP server (`wasmcloud:httpserver`) capability, the key-value store capability (`wasmcloud:keyvalue`) and the logging capability (`wasmcloud:logging`). As usual, it is worth noting that this actor does _not_ know where its HTTP server comes from, nor does it know which key-value implementation the host runtime has provided.

This is a decently modified version of the [open source wasmCloud example](https://github.com/wasmCloud/examples/tree/main/actor/todo) contributed by the wonderful folks at [Red Badger](https://red-badger.com/).

## Running
```
wash build
wash up -d
# launch TODO actor from file (TODO: maybe we can really use the file thing?)
wash ctl start provider wasmcloud.azurecr.io/httpserver:0.17.0
wash ctl start provider wasmcloud.azurecr.io/kvredis:0.19.0
# TODO: correct actor ID
wash ctl link put MCFMFDWFHGKELOXPCNCDXKK5OFLHBVEWRAOXR5JSQUD2TOFRE3DFPM7E VAG3QITQQ2ODAOWB5TTQSDJ53XK3SHBEIFNK4AYJ5RKAX2UNSCAPHA5M wasmcloud:httpserver address=0.0.0.0:8080
wash ctl link put MCFMFDWFHGKELOXPCNCDXKK5OFLHBVEWRAOXR5JSQUD2TOFRE3DFPM7E VAZVC4RX54J2NVCMCW7BPCAHGGG5XZXDBXFUMDUXGESTMQEJLC3YVZWB wasmcloud:keyvalue URL=redis://127.0.0.1:6379
```