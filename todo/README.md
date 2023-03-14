# Todo App

This actor implements the [Todo backend spec](https://github.com/TodoBackend/todo-backend-js-spec/blob/master/js/specs.js).

This actor makes use of the HTTP server (`wasmcloud:httpserver`) capability, the key-value store capability (`wasmcloud:keyvalue`) and the logging capability (`wasmcloud:logging`). As usual, it is worth noting that this actor does _not_ know where its HTTP server comes from, nor does it know which key-value implementation the host runtime has provided.

This is a decently modified version of the [open source wasmCloud example](https://github.com/wasmCloud/examples/tree/main/actor/todo) contributed by the wonderful folks at [Red Badger](https://red-badger.com/).

## Running

Ensure you've changed directory into this folder for these commands.

```
cosmo up
cosmo launch
```

Optionally, if you have [wash](https://wasmcloud.com/docs/installation) installed, you can run the following commands to start Redis and launch local capability providers for the Todo app.

```
redis-server &
wash ctl start provider wasmcloud.azurecr.io/httpserver:0.17.0
wash ctl start provider wasmcloud.azurecr.io/kvredis:0.19.0

ACTOR_ID=$(wash claims inspect build/todo_s.wasm -o json | jq -r '.module')
wash ctl link put $ACTOR_ID VAG3QITQQ2ODAOWB5TTQSDJ53XK3SHBEIFNK4AYJ5RKAX2UNSCAPHA5M wasmcloud:httpserver address=0.0.0.0:8080
wash ctl link put $ACTOR_ID VAZVC4RX54J2NVCMCW7BPCAHGGG5XZXDBXFUMDUXGESTMQEJLC3YVZWB wasmcloud:keyvalue URL=redis://127.0.0.1:6379
```
