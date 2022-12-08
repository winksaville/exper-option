#!/usr/bin/env bash

# Enable error options
set -Eeuo pipefail

# Enable debug
#set -x

# Use `cargo asm --rust --bin exper-option` to see list
gen_bin_asm() {
    cargo asm --rust --bin exper-option "exper_option::$1" > asm/$1.txt
}

# Use `cargo asm --rust --lib` to see list
gen_lib_asm() {
    cargo asm --rust --lib "exper_option::$1" > asm/$1.txt
}

# Use `cargo asm --rust --bench bench-iai` to see list
gen_bench_iai_asm() {
    cargo asm --rust --bench bench-iai "bench_iai::iai_wrappers::$1" > asm/$1.txt
}

# Use `cargo asm --rust --bench bench-crit` to see list
gen_bench_crit_asm() {
    cargo asm --rust --bench bench-crit "bench_crit::$1" > asm/bench_crit_$1.txt
}

gen_bin_asm "main"

gen_lib_asm "check_option_val"
gen_lib_asm "check_val"
gen_lib_asm "check_option_ptr_to_val"
gen_lib_asm "check_ptr_to_val"
gen_lib_asm "check_option_box_of_val"
gen_lib_asm "check_box_of_val"
gen_lib_asm "check_option_box_of_val_ret_val_option_box"
gen_lib_asm "check_box_of_val_ret_val_option_box"
gen_lib_asm "check_box_of_val_ret_val_box"

gen_bench_iai_asm "bench_iai_check_option_val_ret_val"
gen_bench_iai_asm "bench_iai_check_option_val_ret_neg1"
gen_bench_iai_asm "bench_iai_check_option_val_ret_neg2"
gen_bench_iai_asm "bench_iai_check_val_ret_val"
gen_bench_iai_asm "bench_iai_check_val_ret_neg1"
gen_bench_iai_asm "bench_iai_check_option_ptr_to_val_ret_val"
gen_bench_iai_asm "bench_iai_check_option_ptr_to_val_ret_neg1"
gen_bench_iai_asm "bench_iai_check_option_ptr_to_val_ret_neg2"
gen_bench_iai_asm "bench_iai_check_ptr_to_val_ret_val"
gen_bench_iai_asm "bench_iai_check_ptr_to_val_ret_neg1"
gen_bench_iai_asm "bench_iai_check_option_box_of_val_ret_val"
gen_bench_iai_asm "bench_iai_check_option_box_of_val_ret_neg1"
gen_bench_iai_asm "bench_iai_check_option_box_of_val_ret_neg2"
gen_bench_iai_asm "bench_iai_check_box_of_val_ret_val"
gen_bench_iai_asm "bench_iai_check_box_of_val_ret_neg1"
gen_bench_iai_asm "bench_iai_check_option_box_of_val_ret_val_option_box_val"
gen_bench_iai_asm "bench_iai_check_option_box_of_val_ret_val_option_box_neg1"
gen_bench_iai_asm "bench_iai_check_option_box_of_val_ret_val_option_box_neg2"
gen_bench_iai_asm "bench_iai_check_box_of_val_ret_val_option_box_val"
gen_bench_iai_asm "bench_iai_check_box_of_val_ret_val_option_box_neg1"
gen_bench_iai_asm "bench_iai_check_box_of_val_ret_val_option_box_neg2"
gen_bench_iai_asm "bench_iai_check_box_of_val_ret_val_box_val"
gen_bench_iai_asm "bench_iai_check_box_of_val_ret_val_box_neg1"

# These generate some asm files, but I don't recognize any
# bench_crit_check_xxx code in the output :(
#gen_bench_crit_asm "main"
#gen_bench_crit_asm "benches"
