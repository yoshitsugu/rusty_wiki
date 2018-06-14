#!/bin/bash

if [ "${MANUAL}" = "" ]; then
  cargo web start --target=wasm32-unknown-unknown --host 0.0.0.0
else
  tail -f < /dev/null
fi
