#!/usr/bin/env bash

# replace if with AND short circuit evaluation
# also, use lower case for variables
(( $1 % 3 == 0 )) && output="Pling"
# string concatenation
(( $1 % 5 == 0 )) && output+="Plang"
# OR only executes 2nd statement if 1st fails 
(( $1 % 7 )) || output+="Plong"

# print $output. if $output is unset/null, print 1st input param
echo "${output:-$1}"
