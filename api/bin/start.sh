#!/bin/bash

if [ "${MANUAL}" = "" ]; then
  diesel database setup
  cargo run
else
  tail -f < /dev/null
fi
