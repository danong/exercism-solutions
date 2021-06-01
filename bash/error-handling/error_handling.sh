#!/usr/bin/env bash

main () {
# throw an error if number of parameters is not 1
    if [[ $# -ne 1 ]]; then
        echo "Usage: error_handling.sh <person>"
        exit 1
    fi

    echo "Hello, $1"
}

main "$@"
