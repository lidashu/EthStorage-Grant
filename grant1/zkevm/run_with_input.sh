#!/bin/bash

# Must use zkGo to build
# GOOS=wasip1 GOARCH=wasm go build -gcflags=all=-d=softfloat -o fib_with_input.wasm fib_zkgo.go 
GOOS=wasip1 GOARCH=wasm /Users/lidashu/code/zkgo/go-zkGo/bin/go  build -gcflags=all=-d=softfloat -o fib_zkgo.wasm fib_zkgo.go 

# Require node > 20
echo `date '+%s.%n'`
node ../zkWasm-emulator/wasi/wasi_exec_node.js fib_zkgo.wasm input.dat
echo `date '+%s.%n'`

/Users/lidashu/code/ethstorage/zkWasm/target/release/delphinus-cli -k 22 --wasm fib_zkgo.wasm  --function zkmain dry-run
echo `date '+%s.%n'`