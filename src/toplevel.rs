// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/// Defines a `fn main()` that will run all benchmarks defined by listed functions `$function` and
/// their associated seeds (if present). Seeds are represented as `Option<[u32; 4]>`. If `None` is
/// given, a random seed will be used. The seeds are used to seed the
/// [`BenchRng`](ctbench/type.BenchRng.html) that's passed to each function.
///
/// ```ignore
/// #[macro_use]
/// extern crate dudect_bencher;
/// extern crate rand;
///
/// use rand::Rng;
/// use dudect_bencher::{BenchRng, Class, CtRunner};
///
/// fn foo(runner: &mut CtRunner, rng: &mut BenchRng) {
///     println!("first u64 is {}", rng.next_u64());
///
///     // Run something so we don't get a panic
///     runner.run_one(Class::Left, || 0);
///     runner.run_one(Class::Right, || 0);
/// }
///
/// fn bar(runner: &mut CtRunner, rng: &mut BenchRng) {
///     println!("first u64 is {}", rng.next_u64());
///
///     // Run something so we don't get a panic
///     runner.run_one(Class::Left, || 0);
///     runner.run_one(Class::Right, || 0);
/// }
///
/// ctbench_main_with_seeds!(
///     (foo, None),
///     (bar, Some([0x6b6c816d, 0x395d3f8e, 0x798e4828, 0xfbb23c0f]))
/// );
/// ```
#[macro_export]
macro_rules! ctbench_main_with_seeds {
    ($(($function:path, $seed:expr)),+) => {
        extern crate clap;
        use clap::App;
        use $crate::ctbench::{run_benches_console, BenchName, BenchMetadata, BenchOpts};
        use std::path::PathBuf;
        fn main() {
            let mut benches = Vec::new();
            $(
                benches.push(BenchMetadata {
                    name: BenchName(stringify!($function)),
                    seed: $seed,
                    benchfn: $function,
                });
            )+
            let matches = App::new("dudect-bencher")
                .arg_from_usage("--filter [BENCH]\
                                 'Only run the benchmarks whose name contains BENCH'")
                .arg_from_usage("--continuous [BENCH]\
                                 'Runs a continuous benchmark on the first bench matching BENCH'")
                .arg_from_usage("--out [FILE]\
                                 'Appends raw benchmarking data in CSV format to FILE'")
                .get_matches();

            let mut test_opts = BenchOpts::default();
            test_opts.filter = matches.value_of("continuous")
                                      .or(matches.value_of("filter"))
                                      .map(|s| s.to_string());
            test_opts.continuous = matches.is_present("continuous");
            test_opts.file_out = matches.value_of("out").map(PathBuf::from);

            run_benches_console(test_opts, benches).unwrap();
        }
    }
}

/// Defines a `fn main()` that will run all benchmarks defined by listed functions `$function`. The
/// [`BenchRng`](ctbench/type.BenchRng.html)s given to each function are randomly seeded. Exmaple
/// usage:
///
/// ```ignore
/// #[macro_use]
/// extern crate dudect_bencher;
/// extern crate rand;
///
/// use dudect_bencher::{BenchRng, Class, CtRunner};
/// use rand::Rng;
///
/// // Return a random vector of length len
/// fn rand_vec(len: usize, rng: &mut BenchRng) -> Vec<u8> {
///     let mut arr = vec![0u8; len];
///     rng.fill_bytes(&mut arr);
///     arr
/// }
///
/// // Benchmark for some random arithmetic operations. This should produce small t-values
/// fn arith(runner: &mut CtRunner, rng: &mut BenchRng) {
///     let mut inputs = Vec::new();
///     let mut classes = Vec::new();
///
///     // Make 100,000 inputs on each run
///     for _ in 0..100_000 {
///         inputs.push(rng.gen::<usize>());
///         // Randomly pick which distribution this example belongs to
///         if rng.gen::<bool>() {
///             classes.push(Class::Left);
///         }
///         else {
///             classes.push(Class::Right);
///         }
///     }
///
///     for (u, class) in inputs.into_iter().zip(classes.into_iter()) {
///         // Time some random arithmetic operations
///         runner.run_one(class, || ((u + 10) / 6) << 5);
///     }
/// }
///
/// // Benchmark for equality of vectors. This does an early return when it finds an inequality,
/// // so it should be very much not constant-time
/// fn vec_eq(runner: &mut CtRunner, rng: &mut BenchRng) {
///     // Make vectors of size 100
///     let vlen = 100;
///     let mut inputs: Vec<(Vec<u8>, Vec<u8>)> = Vec::new();
///     let mut classes = Vec::new();
///
///     // Make 100,000 random pairs of vectors
///     for _ in 0..100_000 {
///         // Flip a coin. If true, make a pair of vectors that are equal to each other and put
///         // it in the Left distribution
///         if rng.gen::<bool>() {
///             let v1 = rand_vec(vlen, rng);
///             let v2 = v1.clone();
///             inputs.push((v1, v2));
///             classes.push(Class::Left);
///         }
///         // Otherwise, make a pair of vectors that differ at the 6th element and put it in the
///         // right distribution
///         else {
///             let v1 = rand_vec(vlen, rng);
///             let mut v2 = v1.clone();
///             v2[5] = 7;
///             inputs.push((v1, v2));
///             classes.push(Class::Right);
///         }
///     }
///
///     for (class, (u, v)) in classes.into_iter().zip(inputs.into_iter()) {
///         // Now time how long it takes to do a vector comparison
///         runner.run_one(class, || u == v);
///     }
/// }
///
/// // Expand the main function to include benches for arith and vec_eq. Use random RNG seeds
/// ctbench_main!(arith, vec_eq);
/// ```
#[macro_export]
macro_rules! ctbench_main {
    ($($function:path),+) => {
        ctbench_main_with_seeds!($(($function, None)),+);
    }
}
