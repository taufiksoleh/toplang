# Testing Guide

This document describes how to test the TopLang compiler and example programs.

## Local Testing

### Run All Example Tests

Use the provided test script to run all example files:

```bash
./test_examples.sh
```

This script will:
- Build the compiler in release mode
- Run all `.top` files in the `examples/` directory
- Skip interactive examples that require user input
- Report pass/fail status with colored output
- Exit with non-zero code if any test fails

### Run Individual Tests

To test a specific example:

```bash
cargo build --release
./target/release/topc examples/hello.top
```

### Run Unit Tests

To run Rust unit tests:

```bash
cargo test
```

## Test Organization

### Example Files

- `examples/*.top` - Example programs demonstrating language features
- `examples/test_*.top` - Specific feature test files with assertions

Test files include:
- `test_comparison_operators.top` - Tests `not equals`, `>=`, `<=` operators
- `test_modulo.top` - Tests modulo operator with various cases
- `test_loop_control.top` - Tests `break` and `continue` statements
- `test_string_operations.top` - Tests string functions (`length`, `uppercase`, `substring`)

### Interactive Examples

These examples require user input and are skipped by the test runner:
- `input.top` - Demonstrates user input with the `ask` statement
- `arrays_with_input.top` - Array manipulation with user input

To test these manually:

```bash
./target/release/topc examples/input.top
# Follow the prompts
```

## Continuous Integration

### GitHub Actions

The project uses GitHub Actions for automated testing. The CI pipeline runs on:
- Every push to `main` or `master` branches
- Every pull request to `main` or `master` branches

### CI Jobs

1. **Test Job**
   - Checks code formatting (`cargo fmt`)
   - Runs clippy lints (`cargo clippy`)
   - Builds the project
   - Runs unit tests
   - Runs all example tests using `test_examples.sh`

2. **Coverage Job**
   - Generates code coverage using `cargo-tarpaulin`
   - Uploads coverage reports to Codecov

### Viewing CI Results

Check the Actions tab in the GitHub repository to see CI results for each commit and pull request.

## Writing New Tests

### Adding Example Files

1. Create a new `.top` file in the `examples/` directory
2. Add a comment at the top describing what the example demonstrates
3. Include a `main()` function as the entry point
4. The test runner will automatically pick it up

Example:

```toplang
# Test: Feature Name

function main() {
    # Your test code here
    var result is 42
    print result
}
```

### Adding Test Assertions

For test files, use conditional logic and print statements to indicate pass/fail:

```toplang
function main() {
    print "Testing feature X"

    var result is some_operation()

    if result equals expected_value {
        print "PASS: feature X works correctly"
    } else {
        print "FAIL: feature X returned wrong value"
    }
}
```

### Interactive Tests

If your test requires user input:
1. Name it clearly (e.g., `something_with_input.top`)
2. Update `test_examples.sh` to skip it:
   ```bash
   if [[ "$filename" == "input.top" ]] || [[ "$filename" == "your_new_file.top" ]]; then
   ```

## Code Quality Checks

### Formatting

Check code formatting:

```bash
cargo fmt --check
```

Auto-fix formatting:

```bash
cargo fmt
```

### Linting

Run clippy to check for common issues:

```bash
cargo clippy
```

Fix clippy warnings:

```bash
cargo clippy --fix
```

### Security Audit

Check for known security vulnerabilities in dependencies:

```bash
cargo audit
```

## Debugging Failed Tests

If a test fails:

1. **Check the error output** - The test runner shows the first few lines of error output
2. **Run the test manually** to see full output:
   ```bash
   ./target/release/topc examples/failing_test.top
   ```
3. **Use verbose mode** for more details:
   ```bash
   ./target/release/topc -v examples/failing_test.top
   ```
4. **Check tokens and AST**:
   ```bash
   ./target/release/topc -t examples/failing_test.top  # Show tokens
   ./target/release/topc -a examples/failing_test.top  # Show AST
   ```

## Performance Testing

To test performance:

```bash
# Build with optimizations
cargo build --release

# Time execution
time ./target/release/topc examples/fibonacci.top

# Profile with Rust tools
cargo build --release
perf record ./target/release/topc examples/fibonacci.top
perf report
```

## Contributing

When contributing new features:

1. Add example files demonstrating the feature
2. Add test files with assertions
3. Ensure all tests pass locally: `./test_examples.sh`
4. Ensure code is formatted: `cargo fmt`
5. Ensure no clippy warnings: `cargo clippy`
6. Submit a pull request - CI will run automatically

## Resources

- [Rust Testing Documentation](https://doc.rust-lang.org/book/ch11-00-testing.html)
- [GitHub Actions Documentation](https://docs.github.com/en/actions)
- [Codecov Documentation](https://docs.codecov.com/)
