# Experiment with rust Option

I'm currious how code using `Option` is different from code without it.
So want to look at the genrated code, how performance might change
and size of the resulting types.

TODO: Summary

## Run:

```
wink@3900x 22-12-10T06:14:00.756Z:~/prgs/rust/myrepos/exper-option (main)
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/exper-option`
size_of::<u8>=1
size_of::<Option<u8>>=2
size_of::<u16>=2
size_of::<Option<u16>>=4
size_of::<u32>=4
size_of::<Option<u32>>=8
size_of::<u64>=8
size_of::<Option<u64>>=16
size_of::<u128>=16
size_of::<Option<u128>>=24
size_of::<usize>=8
size_of::<Option<usize>>=16
discrimanent(&var_option_u8_none)=Discriminant(0)
discrimanent(var_option_u8_some)=Discriminant(1)
size_of::<Option<&u8>>=8
discrimanent(&var_option_ref_u8_none)=Discriminant(0)
discrimanent(&var_option_ref_var_u8_some)=Discriminant(1)
size_of::<UnitStruct>=0
size_of::<&UnitStruct>=8
size_of::<Option<&UnitStruct>>=8
size_of::<StructU8>=1
size_of::<&StructU8>=8
size_of::<Option<&StructU8>>=8
3
-1
-2
3
-1
3
-1
-2
3
-1
3
-1
-2
3
-1
3 Some(3)
-1 Some(255)
-2 None
3 Some(3)
-1 Some(255)
-2 None
3 3
-1 255
wink@3900x 22-12-10T06:14:02.483Z:~/prgs/rust/myrepos/exper-option (main)
$ cargo run --release
    Finished release [optimized] target(s) in 0.01s
     Running `target/release/exper-option`
size_of::<u8>=1
size_of::<Option<u8>>=2
size_of::<u16>=2
size_of::<Option<u16>>=4
size_of::<u32>=4
size_of::<Option<u32>>=8
size_of::<u64>=8
size_of::<Option<u64>>=16
size_of::<u128>=16
size_of::<Option<u128>>=24
size_of::<usize>=8
size_of::<Option<usize>>=16
discrimanent(&var_option_u8_none)=Discriminant(0)
discrimanent(var_option_u8_some)=Discriminant(1)
size_of::<Option<&u8>>=8
discrimanent(&var_option_ref_u8_none)=Discriminant(0)
discrimanent(&var_option_ref_var_u8_some)=Discriminant(1)
size_of::<UnitStruct>=0
size_of::<&UnitStruct>=8
size_of::<Option<&UnitStruct>>=8
size_of::<StructU8>=1
size_of::<&StructU8>=8
size_of::<Option<&StructU8>>=8
3
-1
-2
3
-1
3
-1
-2
3
-1
3
-1
-2
3
-1
3 Some(3)
-1 Some(255)
-2 None
3 Some(3)
-1 Some(255)
-2 None
3 3
-1 255
wink@3900x 22-12-10T06:14:08.427Z:~/prgs/rust/myrepos/exper-option (main)
```

## Benchmarks:

```
wink@fwlaptop 22-12-10T06:17:38.218Z:~/prgs/rust/myrepos/exper-option (main)
$ cargo clean
wink@fwlaptop 22-12-10T06:17:40.753Z:~/prgs/rust/myrepos/exper-option (main)
$ taskset -c 0 cargo bench
   Compiling autocfg v1.1.0
   Compiling proc-macro2 v1.0.47
   ...
   Compiling exper-option v0.1.0 (/home/wink/prgs/rust/myrepos/exper-option)
   Compiling iai v0.1.1
    Finished bench [optimized] target(s) in 57.45s
     Running unittests src/lib.rs (target/release/deps/exper_option-7adf9591ced3cc50)

running 12 tests
test tests::check_size_of_u8 ... ignored
test tests::none_check_option_ptr_to_val ... ignored
test tests::none_check_option_val ... ignored
test tests::range_check_box_of_val ... ignored
test tests::range_check_box_of_val_ret_val_box ... ignored
test tests::range_check_box_of_val_ret_val_option_box ... ignored
test tests::range_check_option_box_of_val_ret_val_option_box ... ignored
test tests::range_check_option_of_val ... ignored
test tests::range_check_option_ptr_to_val ... ignored
test tests::range_check_option_val ... ignored
test tests::range_check_ptr_to_val ... ignored
test tests::range_check_val ... ignored

test result: ok. 0 passed; 0 failed; 12 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/main.rs (target/release/deps/exper_option-b5717ac31043ddf7)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running benches/crit_bench.rs (target/release/deps/bench_crit-6ca0450e0d69b1ca)
bench_crit_check_option_val_ret_val
                        time:   [1.1946 ns 1.1951 ns 1.1956 ns]
Found 12 outliers among 100 measurements (12.00%)
  7 (7.00%) high mild
  5 (5.00%) high severe

bench_crit_check_option_val_ret_neg1
                        time:   [1.1947 ns 1.1956 ns 1.1968 ns]
Found 16 outliers among 100 measurements (16.00%)
  5 (5.00%) high mild
  11 (11.00%) high severe

bench_crit_check_option_val_ret_neg2
                        time:   [990.42 ps 994.06 ps 997.61 ps]
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high mild

bench_crit_check_val_ret_val
                        time:   [955.69 ps 956.02 ps 956.39 ps]
Found 12 outliers among 100 measurements (12.00%)
  7 (7.00%) high mild
  5 (5.00%) high severe

bench_crit_check_val_ret_neg1
                        time:   [956.15 ps 956.75 ps 957.43 ps]
Found 12 outliers among 100 measurements (12.00%)
  8 (8.00%) high mild
  4 (4.00%) high severe

bench_crit_check_option_ptr_to_val_ret_val
                        time:   [955.93 ps 957.45 ps 959.95 ps]
Found 10 outliers among 100 measurements (10.00%)
  6 (6.00%) high mild
  4 (4.00%) high severe

bench_crit_check_option_ptr_to_val_ret_neg1
                        time:   [1.1946 ns 1.1952 ns 1.1958 ns]
Found 13 outliers among 100 measurements (13.00%)
  5 (5.00%) high mild
  8 (8.00%) high severe

bench_crit_check_option_ptr_to_val_ret_neg2
                        time:   [1.1947 ns 1.1954 ns 1.1962 ns]
Found 15 outliers among 100 measurements (15.00%)
  6 (6.00%) high mild
  9 (9.00%) high severe

bench_crit_check_ptr_to_val_ret_val
                        time:   [956.28 ps 958.54 ps 962.40 ps]
Found 18 outliers among 100 measurements (18.00%)
  7 (7.00%) high mild
  11 (11.00%) high severe

bench_crit_check_ptr_to_val_ret_neg1
                        time:   [955.74 ps 956.27 ps 956.88 ps]
Found 18 outliers among 100 measurements (18.00%)
  5 (5.00%) high mild
  13 (13.00%) high severe

bench_crit_check_option_box_of_val_ret_val
                        time:   [8.8067 ns 8.8213 ns 8.8367 ns]
Found 7 outliers among 100 measurements (7.00%)
  2 (2.00%) low severe
  2 (2.00%) low mild
  2 (2.00%) high mild
  1 (1.00%) high severe

bench_crit_check_option_box_of_val_ret_neg1
                        time:   [8.5085 ns 8.5204 ns 8.5323 ns]
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) low severe
  1 (1.00%) high mild

bench_crit_check_option_box_of_val_ret_neg2
                        time:   [1.1993 ns 1.2051 ns 1.2127 ns]
Found 16 outliers among 100 measurements (16.00%)
  5 (5.00%) high mild
  11 (11.00%) high severe

bench_crit_check_box_of_val_ret_val
                        time:   [8.5478 ns 8.5592 ns 8.5711 ns]
Found 4 outliers among 100 measurements (4.00%)
  1 (1.00%) low severe
  1 (1.00%) low mild
  1 (1.00%) high mild
  1 (1.00%) high severe

bench_crit_check_box_of_val_ret_neg1
                        time:   [8.8401 ns 8.8547 ns 8.8701 ns]
Found 4 outliers among 100 measurements (4.00%)
  1 (1.00%) low severe
  2 (2.00%) low mild
  1 (1.00%) high severe

bench_crit_check_option_box_of_val_ret_val_option_box_val
                        time:   [8.3828 ns 8.3913 ns 8.4005 ns]
Found 9 outliers among 100 measurements (9.00%)
  1 (1.00%) low severe
  1 (1.00%) low mild
  4 (4.00%) high mild
  3 (3.00%) high severe

bench_crit_check_option_box_of_val_ret_val_option_box_neg1
                        time:   [8.3732 ns 8.3964 ns 8.4272 ns]
Found 8 outliers among 100 measurements (8.00%)
  1 (1.00%) low severe
  3 (3.00%) high mild
  4 (4.00%) high severe

bench_crit_check_option_box_of_val_ret_val_option_box_neg2
                        time:   [1.1949 ns 1.1969 ns 1.2008 ns]
Found 9 outliers among 100 measurements (9.00%)
  5 (5.00%) high mild
  4 (4.00%) high severe

bench_crit_check_box_of_val_ret_val_option_box_val
                        time:   [8.8329 ns 8.8402 ns 8.8482 ns]
Found 6 outliers among 100 measurements (6.00%)
  2 (2.00%) low severe
  2 (2.00%) low mild
  2 (2.00%) high mild

bench_crit_check_box_of_val_ret_val_option_box_neg1
                        time:   [8.5261 ns 8.5355 ns 8.5447 ns]
Found 4 outliers among 100 measurements (4.00%)
  3 (3.00%) low mild
  1 (1.00%) high severe

bench_crit_check_box_of_val_ret_val_option_box_neg2
                        time:   [8.6144 ns 8.6249 ns 8.6361 ns]
Found 6 outliers among 100 measurements (6.00%)
  2 (2.00%) low severe
  3 (3.00%) high mild
  1 (1.00%) high severe

bench_crit_check_box_of_val_ret_val_box_val
                        time:   [8.3491 ns 8.3578 ns 8.3667 ns]
Found 4 outliers among 100 measurements (4.00%)
  1 (1.00%) low severe
  3 (3.00%) low mild

bench_crit_check_box_of_val_ret_val_box_neg1
                        time:   [8.5299 ns 8.5370 ns 8.5442 ns]
Found 5 outliers among 100 measurements (5.00%)
  2 (2.00%) low severe
  1 (1.00%) low mild
  2 (2.00%) high mild

     Running benches/iai_bench.rs (target/release/deps/bench_iai-d53779fb5adfcca3)
bench_iai_check_option_val_ret_val
  Instructions:                  14
  L1 Accesses:                   20
  L2 Accesses:                    2
  RAM Accesses:                   0
  Estimated Cycles:              30

bench_iai_check_option_val_ret_neg1
  Instructions:                  14
  L1 Accesses:                   19
  L2 Accesses:                    3
  RAM Accesses:                   0
  Estimated Cycles:              34

bench_iai_check_option_val_ret_neg2
  Instructions:                  13
  L1 Accesses:                   17
  L2 Accesses:                    3
  RAM Accesses:                   1
  Estimated Cycles:              67

bench_iai_check_val_ret_val
  Instructions:                  10
  L1 Accesses:                   13
  L2 Accesses:                    4
  RAM Accesses:                   1
  Estimated Cycles:              68

bench_iai_check_val_ret_neg1
  Instructions:                  10
  L1 Accesses:                   12
  L2 Accesses:                    4
  RAM Accesses:                   2
  Estimated Cycles:             102

bench_iai_check_option_ptr_to_val_ret_val
  Instructions:                  13
  L1 Accesses:                   19
  L2 Accesses:                    3
  RAM Accesses:                   0
  Estimated Cycles:              34

bench_iai_check_option_ptr_to_val_ret_neg1
  Instructions:                  13
  L1 Accesses:                   19
  L2 Accesses:                    3
  RAM Accesses:                   0
  Estimated Cycles:              34

bench_iai_check_option_ptr_to_val_ret_neg2
  Instructions:                   9
  L1 Accesses:                   13
  L2 Accesses:                    3
  RAM Accesses:                   1
  Estimated Cycles:              63

bench_iai_check_ptr_to_val_ret_val
  Instructions:                  11
  L1 Accesses:                   17
  L2 Accesses:                    2
  RAM Accesses:                   1
  Estimated Cycles:              62

bench_iai_check_ptr_to_val_ret_neg1
  Instructions:                  11
  L1 Accesses:                   17
  L2 Accesses:                    2
  RAM Accesses:                   1
  Estimated Cycles:              62

bench_iai_check_option_box_of_val_ret_val
  Instructions:                 181
  L1 Accesses:                  262
  L2 Accesses:                    2
  RAM Accesses:                   1
  Estimated Cycles:             307

bench_iai_check_option_box_of_val_ret_neg1
  Instructions:                 181
  L1 Accesses:                  261
  L2 Accesses:                    3
  RAM Accesses:                   1
  Estimated Cycles:             311

bench_iai_check_option_box_of_val_ret_neg2
  Instructions:                  29
  L1 Accesses:                   38
  L2 Accesses:                    3
  RAM Accesses:                   2
  Estimated Cycles:             123

bench_iai_check_box_of_val_ret_val
  Instructions:                 179
  L1 Accesses:                  259
  L2 Accesses:                    4
  RAM Accesses:                   0
  Estimated Cycles:             279

bench_iai_check_box_of_val_ret_neg1
  Instructions:                 179
  L1 Accesses:                  258
  L2 Accesses:                    5
  RAM Accesses:                   0
  Estimated Cycles:             283

bench_iai_check_option_box_of_val_ret_val_option_box_val
  Instructions:                 182
  L1 Accesses:                  258
  L2 Accesses:                    4
  RAM Accesses:                   1
  Estimated Cycles:             313

bench_iai_check_option_box_of_val_ret_val_option_box_neg1
  Instructions:                 182
  L1 Accesses:                  259
  L2 Accesses:                    3
  RAM Accesses:                   1
  Estimated Cycles:             309

bench_iai_check_option_box_of_val_ret_val_option_box_neg2
  Instructions:                  31
  L1 Accesses:                   40
  L2 Accesses:                    3
  RAM Accesses:                   1
  Estimated Cycles:              90

bench_iai_check_box_of_val_ret_val_option_box_val
  Instructions:                 184
  L1 Accesses:                  260
  L2 Accesses:                    4
  RAM Accesses:                   3
  Estimated Cycles:             385

bench_iai_check_box_of_val_ret_val_option_box_neg1
  Instructions:                 184
  L1 Accesses:                  260
  L2 Accesses:                    5
  RAM Accesses:                   2
  Estimated Cycles:             355

bench_iai_check_box_of_val_ret_val_option_box_neg2
  Instructions:                 184
  L1 Accesses:                  263
  L2 Accesses:                    4
  RAM Accesses:                   2
  Estimated Cycles:             353

bench_iai_check_box_of_val_ret_val_box_val
  Instructions:                 178
  L1 Accesses:                  255
  L2 Accesses:                    2
  RAM Accesses:                   2
  Estimated Cycles:             335

bench_iai_check_box_of_val_ret_val_box_neg1
  Instructions:                 178
  L1 Accesses:                  255
  L2 Accesses:                    2
  RAM Accesses:                   2
  Estimated Cycles:             335

wink@fwlaptop 22-12-10T06:22:42.469Z:~/prgs/rust/myrepos/exper-option (main)
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
