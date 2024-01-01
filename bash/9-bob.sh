#!/usr/bin/env bash

var=$(echo "$1" | tr -d '[:space:]')

if [[ -z $var ]]; then
    echo "Fine. Be that way!"
elif [[ ${var^^} == $var ]] && [[ ${var,,} != $var ]]; then
    if [[ ${var: -1} == \? ]]; then
        echo "Calm down, I know what I'm doing!"
    else
        echo "Whoa, chill out!"
    fi
elif [[ ${var: -1} == \? ]]; then
    echo "Sure."
else
    echo "Whatever."
fi
