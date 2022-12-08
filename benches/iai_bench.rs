// To get "consistent results" use `taskset`, for example
//   taskset -c 0 cargo bench --bench bench-ia1
//
// The CPU in the laptop I've run this tests is:
//   $ inxi -C
//   CPU:
//     Info: quad core model: 11th Gen Intel Core i5-1135G7 bits: 64 type: MT MCP
//       cache: L2: 5 MiB
//     Speed (MHz): avg: 2026 min/max: 400/4200 cores: 1: 2400 2: 2400 3: 2400
//       4: 995 5: 2400 6: 2400 7: 818 8: 2400
//
// I've decided that using `black_box` around the invocation
// of the function being benchmarked gives fairly good numbers.
// The reason is that when trying various combination of `black_box`
// in `bench_check_option_some_ret_val` I think it gives
// "more reasonable" results.
//
// It includes calling overhead as well as the callee code.
// If you don't use `black_box` the `bench_xxx` code sets up
// up the parameters and jumps to invoke the function.  The
// `Instructions` is 9 which is one more the the 8 instructions
// I count in `check_option_val` and the `Estimated Cycles` is
// high at 88 cycles.
//
// But here is another weird thing. The results I pasted into
// each `bench_xx` is when that benchmark was run by itself, the
// other benchmarks were commented out in `iai::main!`. But when
// when I enable all of them we see fairly different numbers,
// especially in teh Estimated Cycles:
//
//  $ taskset -c 0,1 cargo bench --bench bench-iai
//     Compiling exper-option v0.1.0 (/home/wink/prgs/rust/myrepos/exper-option)
//      Finished bench [optimized] target(s) in 0.15s
//       Running benches/iai_bench.rs (target/release/deps/bench_iai-d53779fb5adfcca3)
//  bench_check_option_some_ret_val
//    Instructions:                  14 (No change)
//    L1 Accesses:                   20 (No change)
//    L2 Accesses:                    1 (-50.00000%)
//    RAM Accesses:                   1 (  +inf%)
//    Estimated Cycles:              60 (+100.0000%)
//
//  bench_check_option_val_some_ret_neg1
//    Instructions:                  14 (No change)
//    L1 Accesses:                   20 (+5.263158%)
//    L2 Accesses:                    1 (-50.00000%)
//    RAM Accesses:                   1 (No change)
//    Estimated Cycles:              60 (-6.250000%)
//
//  bench_check_option_val_ret_neg2_none
//    Instructions:                  13 (No change)
//    L1 Accesses:                   17 (-5.555556%)
//    L2 Accesses:                    2 (No change)
//    RAM Accesses:                   2 (+100.0000%)
//    Estimated Cycles:              97 (+53.96825%)
//
//  bench_check_val
//    Instructions:                  10 (No change)
//    L1 Accesses:                   16 (-5.882353%)
//    L2 Accesses:                    1 (No change)
//    RAM Accesses:                   1 (  +inf%)
//    Estimated Cycles:              56 (+154.5455%)
//
//  bench_check_val_ret_neg1
//    Instructions:                  10 (No change)
//    L1 Accesses:                   17 (No change)
//    L2 Accesses:                    0 (-100.0000%)
//    RAM Accesses:                   1 (  +inf%)
//    Estimated Cycles:              52 (+136.3636%)

use criterion::black_box;
use exper_option::{
    check_box_of_val, check_box_of_val_ret_val_box, check_box_of_val_ret_val_option_box,
    check_option_box_of_val, check_option_box_of_val_ret_val_option_box, check_option_ptr_to_val,
    check_option_val, check_ptr_to_val, check_val,
};

#[allow(unused)]
fn bench_iai_check_option_val_ret_val() {
    //  $ cargo asm --rust --bench bench-iai "bench_iai::iai_wrappers::bench_check_option_val_ret_val"
    //     Compiling exper-option v0.1.0 (/home/wink/prgs/rust/myrepos/exper-option)
    //      Finished release [optimized] target(s) in 0.24s
    //  .section .text.bench_iai::iai_wrappers::bench_check_option_some_ret_val,"ax",@progbits
    //  	.p2align	4, 0x90
    //  	.type	bench_iai::iai_wrappers::bench_check_option_some_ret_val,@function
    //  bench_iai::iai_wrappers::bench_check_option_some_ret_val:
    //
    //  		// /home/wink/.cargo/registry/src/github.com-1ecc6299db9ec823/iai-0.1.1/src/macros.rs : 41
    //  		pub fn $func_name() {
    //  	.cfi_startproc
    //  		// /home/wink/prgs/rust/myrepos/exper-option/benches/iai_bench.rs : 8
    //  		check_option_val(Some(21));
    //  	mov edi, 1
    //  	mov esi, 21
    //  	jmp qword ptr [rip + exper_option::check_option_val@GOTPCREL]
    //
    //  	.size	bench_iai::iai_wrappers::bench_check_option_some_ret_val, .Lfunc_end5-bench_iai::iai_wrappers::bench_check_option_some_ret_val

    //  $ taskset -c 0,1 cargo bench --bench bench-iai
    //     Compiling exper-option v0.1.0 (/home/wink/prgs/rust/myrepos/exper-option)
    //      Finished bench [optimized] target(s) in 0.14s
    //       Running benches/iai_bench.rs (target/release/deps/bench_iai-d53779fb5adfcca3)
    //  bench_check_option_some_ret_val
    //    Instructions:                   9 (-47.05882%)
    //    L1 Accesses:                    8 (-68.00000%)
    //    L2 Accesses:                    2 (No change)
    //    RAM Accesses:                   2 (  +inf%)
    //    Estimated Cycles:              88 (+151.4286%)
    //
    //  wink@fwlaptop 22-12-09T17:56:55.310Z:~/prgs/rust/myrepos/exper-option (main)
    //check_option_val(Some(21));

    //  $ cargo asm --rust --bench bench-iai "bench_iai::iai_wrappers::bench_check_option_some_ret_val"
    //     Compiling exper-option v0.1.0 (/home/wink/prgs/rust/myrepos/exper-option)
    //      Finished release [optimized] target(s) in 0.24s
    //  .section .text.bench_iai::iai_wrappers::bench_check_option_some_ret_val,"ax",@progbits
    //  	.p2align	4, 0x90
    //  	.type	bench_iai::iai_wrappers::bench_check_option_some_ret_val,@function
    //  bench_iai::iai_wrappers::bench_check_option_some_ret_val:
    //
    //  		// /home/wink/.cargo/registry/src/github.com-1ecc6299db9ec823/iai-0.1.1/src/macros.rs : 41
    //  		pub fn $func_name() {
    //  	.cfi_startproc
    //  	push rax
    //  	.cfi_def_cfa_offset 16
    //
    //  		// /home/wink/prgs/rust/myrepos/exper-option/benches/iai_bench.rs : 73
    //  		black_box(check_option_val(Some(21)));
    //  	mov edi, 1
    //  	mov esi, 21
    //  	call qword ptr [rip + exper_option::check_option_val@GOTPCREL]
    //  	mov qword ptr [rsp], rax
    //
    //  		// /rustc/897e37553bba8b42751c67658967889d11ecd120/library/core/src/ptr/mod.rs : 1474
    //  		intrinsics::volatile_load(src)
    //  	mov rax, qword ptr [rsp]
    //
    //  		// /home/wink/.cargo/registry/src/github.com-1ecc6299db9ec823/iai-0.1.1/src/macros.rs : 43
    //  		}
    //  	pop rax
    //  	.cfi_def_cfa_offset 8
    //  	ret
    //
    //  	.size	bench_iai::iai_wrappers::bench_check_option_some_ret_val, .Lfunc_end5-bench_iai::iai_wrappers::bench_check_option_some_ret_val

    //  $ taskset -c 0,1 cargo bench --bench bench-iai
    //     Compiling exper-option v0.1.0 (/home/wink/prgs/rust/myrepos/exper-option)
    //      Finished bench [optimized] target(s) in 0.15s
    //       Running benches/iai_bench.rs (target/release/deps/bench_iai-d53779fb5adfcca3)
    //  bench_check_option_some_ret_val
    //    Instructions:                  14 (No change)
    //    L1 Accesses:                   20 (No change)
    //    L2 Accesses:                    2 (No change)
    //    RAM Accesses:                   0 (No change)
    //    Estimated Cycles:              30 (No change)

    black_box(check_option_val(Some(21)));

    // Here is the assmebler code for `check_option_val`.
    // I count 8 instructions including the `ret`.
    //
    //  $ cargo asm --rust --lib check_option_val
    //      Finished release [optimized] target(s) in 0.01s
    //  .section .text.exper_option::check_option_val,"ax",@progbits
    //  	.globl	exper_option::check_option_val
    //  	.p2align	4, 0x90
    //  	.type	exper_option::check_option_val,@function
    //  exper_option::check_option_val:
    //
    //  		// /home/wink/prgs/rust/myrepos/exper-option/src/lib.rs : 1
    //  		pub fn check_option_val(param: Option<u8>) -> isize {
    //  	.cfi_startproc
    //  		// /home/wink/prgs/rust/myrepos/exper-option/src/lib.rs : 2
    //  		if let Some(val) = param {
    //  	test sil, sil
    //  	movzx eax, sil
    //  	mov rcx, -1
    //  	cmovns rcx, rax
    //  	test edi, edi
    //  	mov rax, -2
    //  	cmovne rax, rcx
    //
    //  		// /home/wink/prgs/rust/myrepos/exper-option/src/lib.rs : 11
    //  		}
    //  	ret
    //
    //  	.size	exper_option::check_option_val, .Lfunc_end0-exper_option::check_option_val
}

#[allow(unused)]
fn bench_iai_check_option_val_ret_neg1() {
    //  $ taskset -c 0,1 cargo bench --bench bench-iai
    //      Finished bench [optimized] target(s) in 0.02s
    //       Running benches/iai_bench.rs (target/release/deps/bench_iai-d53779fb5adfcca3)
    //  bench_check_option_val_some_ret_neg1
    //    Instructions:                  14 (No change)
    //    L1 Accesses:                   20 (No change)
    //    L2 Accesses:                    2 (No change)
    //    RAM Accesses:                   0 (No change)
    //    Estimated Cycles:              30 (No change)
    black_box(check_option_val(Some(242)));
}

#[allow(unused)]
fn bench_iai_check_option_val_ret_neg2() {
    //  $ taskset -c 0,1 cargo bench --bench bench-iai
    //      Finished bench [optimized] target(s) in 0.02s
    //       Running benches/iai_bench.rs (target/release/deps/bench_iai-d53779fb5adfcca3)
    //  bench_check_option_val_ret_neg2_none
    //    Instructions:                  13 (No change)
    //    L1 Accesses:                   19 (No change)
    //    L2 Accesses:                    2 (No change)
    //    RAM Accesses:                   0 (No change)
    //    Estimated Cycles:              29 (No change)
    black_box(check_option_val(None));
}

#[allow(unused)]
fn bench_iai_check_val_ret_val() {
    //  $ taskset -c 0,1 cargo bench --bench bench-iai
    //      Finished bench [optimized] target(s) in 0.03s
    //       Running benches/iai_bench.rs (target/release/deps/bench_iai-d53779fb5adfcca3)
    //  bench_check_val
    //    Instructions:                  10 (No change)
    //    L1 Accesses:                   17 (No change)
    //    L2 Accesses:                    1 (No change)
    //    RAM Accesses:                   0 (No change)
    //    Estimated Cycles:              22 (No change)
    black_box(check_val(21));
}

#[allow(unused)]
fn bench_iai_check_val_ret_neg1() {
    //  $ taskset -c 0,1 cargo bench --bench bench-iai
    //      Finished bench [optimized] target(s) in 0.02s
    //       Running benches/iai_bench.rs (target/release/deps/bench_iai-d53779fb5adfcca3)
    //  bench_check_val_ret_neg1
    //    Instructions:                  10 (No change)
    //    L1 Accesses:                   17 (No change)
    //    L2 Accesses:                    1 (No change)
    //    RAM Accesses:                   0 (No change)
    //    Estimated Cycles:              22 (No change)
    black_box(check_val(242));
}

#[allow(unused)]
fn bench_iai_check_option_ptr_to_val_ret_val() {
    let val = 21;
    black_box(check_option_ptr_to_val(Some(&val)));
}

#[allow(unused)]
fn bench_iai_check_option_ptr_to_val_ret_neg1() {
    let val = 242;
    black_box(check_option_ptr_to_val(Some(&val)));
}

#[allow(unused)]
fn bench_iai_check_option_ptr_to_val_ret_neg2() {
    black_box(check_option_ptr_to_val(None));
}

#[allow(unused)]
fn bench_iai_check_ptr_to_val_ret_val() {
    let val = 21;
    black_box(check_ptr_to_val(&val));
}

#[allow(unused)]
fn bench_iai_check_ptr_to_val_ret_neg1() {
    let val = 242;
    black_box(check_ptr_to_val(&val));
}

#[allow(unused)]
fn bench_iai_check_option_box_of_val_ret_val() {
    let val = Box::new(21);
    black_box(check_option_box_of_val(Some(val)));
}

#[allow(unused)]
fn bench_iai_check_option_box_of_val_ret_neg1() {
    let val = Box::new(242);
    black_box(check_option_box_of_val(Some(val)));
}

#[allow(unused)]
fn bench_iai_check_option_box_of_val_ret_neg2() {
    black_box(check_option_box_of_val(None));
}

#[allow(unused)]
fn bench_iai_check_box_of_val_ret_val() {
    let val = Box::new(21);
    black_box(check_box_of_val(val));
}

#[allow(unused)]
fn bench_iai_check_box_of_val_ret_neg1() {
    let val = Box::new(242);
    black_box(check_box_of_val(val));
}

#[allow(unused)]
fn bench_iai_check_option_box_of_val_ret_val_option_box_val() {
    let val = Box::new(21);
    black_box(check_option_box_of_val_ret_val_option_box(Some(val)));
}

#[allow(unused)]
fn bench_iai_check_option_box_of_val_ret_val_option_box_neg1() {
    let val = Box::new(242);
    black_box(check_option_box_of_val_ret_val_option_box(Some(val)));
}

#[allow(unused)]
fn bench_iai_check_option_box_of_val_ret_val_option_box_neg2() {
    black_box(check_option_box_of_val_ret_val_option_box(None));
}

#[allow(unused)]
fn bench_iai_check_box_of_val_ret_val_option_box_val() {
    let val = Box::new(21);
    black_box(check_box_of_val_ret_val_option_box(val));
}

#[allow(unused)]
fn bench_iai_check_box_of_val_ret_val_option_box_neg1() {
    let val = Box::new(242);
    black_box(check_box_of_val_ret_val_option_box(val));
}

#[allow(unused)]
fn bench_iai_check_box_of_val_ret_val_option_box_neg2() {
    let val = Box::new(128);
    black_box(check_box_of_val_ret_val_option_box(val));
}

#[allow(unused)]
fn bench_iai_check_box_of_val_ret_val_box_val() {
    let val = Box::new(21);
    black_box(check_box_of_val_ret_val_box(val));
}

#[allow(unused)]
fn bench_iai_check_box_of_val_ret_val_box_neg1() {
    let val = Box::new(242);
    black_box(check_box_of_val_ret_val_box(val));
}

iai::main!(
    bench_iai_check_option_val_ret_val,
    bench_iai_check_option_val_ret_neg1,
    bench_iai_check_option_val_ret_neg2,
    bench_iai_check_val_ret_val,
    bench_iai_check_val_ret_neg1,
    bench_iai_check_option_ptr_to_val_ret_val,
    bench_iai_check_option_ptr_to_val_ret_neg1,
    bench_iai_check_option_ptr_to_val_ret_neg2,
    bench_iai_check_ptr_to_val_ret_val,
    bench_iai_check_ptr_to_val_ret_neg1,
    bench_iai_check_option_box_of_val_ret_val,
    bench_iai_check_option_box_of_val_ret_neg1,
    bench_iai_check_option_box_of_val_ret_neg2,
    bench_iai_check_box_of_val_ret_val,
    bench_iai_check_box_of_val_ret_neg1,
    bench_iai_check_option_box_of_val_ret_val_option_box_val,
    bench_iai_check_option_box_of_val_ret_val_option_box_neg1,
    bench_iai_check_option_box_of_val_ret_val_option_box_neg2,
    bench_iai_check_box_of_val_ret_val_option_box_val,
    bench_iai_check_box_of_val_ret_val_option_box_neg1,
    bench_iai_check_box_of_val_ret_val_option_box_neg2,
    bench_iai_check_box_of_val_ret_val_box_val,
    bench_iai_check_box_of_val_ret_val_box_neg1,
);
