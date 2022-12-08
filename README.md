# Experiment with rust Some

ATM this is the code generated by `cargo new --bin` and
there isn't any `Some` code.

## Run:

```
wink@3900x 22-12-08T20:02:24.625Z:~/prgs/rust/myrepos/exper-some (main)
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/exper-some`
6
wink@3900x 22-12-08T20:02:26.573Z:~/prgs/rust/myrepos/exper-some (main)
$ cargo run --release
    Finished release [optimized] target(s) in 0.01s
     Running `target/release/exper-some`
6
wink@3900x 22-12-08T20:02:36.278Z:~/prgs/rust/myrepos/exper-some (main)
```

## Benchmarks:

```
wink@3900x 22-12-08T20:01:00.311Z:~/prgs/rust/myrepos/exper-some (main)
$ taskset -c 0 cargo bench
    Finished bench [optimized] target(s) in 0.02s
     Running unittests src/lib.rs (target/release/deps/exper_some-8988065eb5107eb1)

running 1 test
test tests::it_works ... ignored

test result: ok. 0 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/main.rs (target/release/deps/exper_some-267ade83b9aa0328)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running benches/crit_bench.rs (target/release/deps/bench_crit-5d5219e4cacc1a06)
add                     time:   [1.0929 ns 1.0939 ns 1.0953 ns]
                        change: [-0.1421% -0.0264% +0.1036%] (p = 0.67 > 0.05)
                        No change in performance detected.
Found 16 outliers among 100 measurements (16.00%)
  5 (5.00%) high mild
  11 (11.00%) high severe

     Running benches/iai_bench.rs (target/release/deps/bench_iai-78d0d50bffd19912)
invoke_add
  Instructions:                   0 (No change)
  L1 Accesses:      18446744073709551611 (No change)
  L2 Accesses:                    3 (No change)
  RAM Accesses:                   2 (No change)
  Estimated Cycles:              80 (No change)

wink@3900x 22-12-08T20:01:23.409Z:~/prgs/rust/myrepos/exper-some (main)
```

## Asm code

The assembler code can be found at [/asm](/asm)
and is generated with [`./gen_asm.sh`](/gen_asm.sh).


## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.