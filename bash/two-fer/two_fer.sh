#!/usr/bin/env bash

main () {
    echo "One for $1, one for me."
}

# if first positional parameter is unset/null, 'you' is substituted
main "${1:-you}"
