#!/usr/bin/env bash

# [ $(echo "$1" | tr '[:lower:]' '[:upper:]' | grep -o '[A-Z]' | sort | uniq | wc -l) -eq 26 ] && echo "true" || echo "false"

# From community solution
[ $(echo "$1" | grep -io '[a-z]' | sort -fu | wc -l) -eq 26 ] && echo "true" || echo "false"
