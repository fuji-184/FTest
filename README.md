# Rust Macro Test & Bench

This project provides a set of declarative macros designed to simplify the declaration of unit tests and benchmarks in Rust. By using these macros, you can group tests and benchmarks into modules with shared scope, reducing boilerplate and improving readability.

---

## Features

* **Scoped Test Modules**: Group tests within a specific module name easily.
* **Shared Context**: Define constants, imports, and functions once within the macro block to be shared across multiple test cases.
* **Integrated Benchmarking**: Supports the nightly `extern crate test` for benchmarking with optional setup blocks.
* **Flexible Syntax**: Use a clean, curly-brace-based syntax for defining test and bench logic.

---

## Usage

Add the library to Cargo.toml using Github URL

```toml
ftest = { git = "https://github.com/fuji-184/FTest.git" }

# for benchmark support in nightly rust
ftest = { git = "https://github.com/fuji-184/FTest.git", features = ["bench"] }
```

### 1. Unit Testing with `test!`

The `test!` macro creates a test module and automatically wraps blocks labeled with your custom name into `#[test]` functions.

```rust
test!(my_test_module_name, {
    use library_name_if_using_lib;

    const OFFSET: i32 = 10;

    fn helper() -> i32 {
        5
    }

    test_case_one {
        let result = OFFSET + helper();
        assert_eq!(result, 15);
    }

    test_case_two {
        assert!(OFFSET > 0);
    }

    // to test async with tokio
    test_something.tokio {

    }

    // to skip running certain test
    test_something.skip {

    }

    // to skip running certain async/tokio test
    test_something.tokio.skip {

    }
});

```

### 2. Benchmarking with `bench!`

The `bench!` macro simplifies the creation of benchmarks using the unstable `Bencher` API. It supports two modes:

* **Standard Bench**: Executes the block inside the `b.iter` loop.
* **Bench with Setup**: Allows you to perform one time setup (outside the loop) while keeping the variables available for the benchmarked code.

**Note**: Benchmarking requires the `#[feature(test)]` attribute and the `bench` feature to be enabled.

```rust
bench!(my_benchmarks, {
    use library_name_if_using_lib;

    const BASE: i32 = 100;

    // Standard benchmark
    simple_addition {
        10 + 20 + BASE
    }

    // Benchmark with setup logic
    complex_setup {
        let data = vec![1, 2, 3, 4, 5];
    } -> {
        data.iter().sum::<i32>() + BASE
    }
});

```

---

## Implementation Details

### The Macros

* **`test!`**: Wraps the content in a `#[cfg(test)] mod`.
* **`parse!`**: A recursive macro that identifies blocks and converts them into `#[test]` functions while leaving standard items (like `fn` or `const`) untouched.
* **`bench!`**: Similar to `test!`, but specifically targets the benchmarking feature.
* **`parse_bench!`**: Handles the transformation of blocks into `#[bench]` functions, utilizing `test::black_box` to prevent compiler optimizations from removing the code under test.

### Requirements

To use the benchmarking features, you must use a Nightly Rust compiler and include the following at the top of your crate root:

```rust
#![feature(test)]

```


---
