#!/usr/bin/env bash

# Enable error options
set -Eeuo pipefail

# Enable debug
#set -x

gen_lib_asm () {
    cargo asm --rust --lib "exper_some::$1" > asm/$1.txt
}

gen_bin_asm () {
    cargo asm --rust --bin exper-some "exper_some::$1" > asm/$1.txt
}

gen_lib_asm "add"
gen_bin_asm "main"
