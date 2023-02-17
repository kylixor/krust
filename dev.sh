#!/bin/bash

# WARNING!! use this script while running without docker.
export FRONTEND_PORT=8080
export BACKEND_PORT=8086

children=()

_term() {
    echo "Caught SIGTERM"
    for child in "${children[@]}"; do
        kill -TERM "$child" 2>/dev/null
    done 
}

_int() {
    echo "Caught SIGINT"
    for child in "${children[@]}"; do
        kill -TERM "$child" 2>/dev/null
    done 
}

trap _term SIGTERM
trap _int SIGINT

pushd frontend;
cargo watch -x "run" &
FRONTEND_PROC=$!
children+=($FRONTEND_PROC)
popd;

pushd backend;
trunk serve &
BACKEND_PROCESS=$!
children+=($BACKEND_PROCESS)
popd;

wait $BACKEND_PROCESS
echo "Done running frontend and backend, bye"