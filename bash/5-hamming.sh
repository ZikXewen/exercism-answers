#!/usr/bin/env bash

error () {
    echo $1
    exit 1
}

(( $# == 2 )) || error "Usage: hamming.sh <string1> <string2>"
(( ${#1} == ${#2} )) || error "strands must be of equal length"

for (( i=0; i<${#1}; i++ )); do
    if [[ "${1:i:1}" != "${2:i:1}" ]]; then
        (( count++ ))
    fi
done
echo ${count-0}
