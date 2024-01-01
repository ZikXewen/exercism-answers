#!/usr/bin/env bash

# not_found=true
# if (( $1 % 3 == 0 )); then
#     not_found=false
#     echo -n "Pling"
# fi
# if (( $1 % 5 == 0 )); then
#     not_found=false
#     echo -n "Plang"
# fi
# if (( $1 % 7 == 0 )); then
#     not_found=false
#     echo -n "Plong"
# fi
# if $not_found; then
#     echo -n "$1"
# fi

### From Community Solution
(( $1 % 3 )) || result+=Pling
(( $1 % 5 )) || result+=Plang
(( $1 % 7 )) || result+=Plong
echo ${result:-$1}
