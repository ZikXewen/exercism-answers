#!/usr/bin/env bash

sum=0
for (( i=0; i<${#1}; i++ )); do
    (( sum += ${1:i:1} ** ${#1} ))
done

# if [[ sum -eq $1 ]]; then
#     echo true
# else
#     echo false
# fi

# From community solution
[[ sum -eq $1 ]] && echo true || echo false
