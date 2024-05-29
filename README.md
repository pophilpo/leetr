# Leetr

A CLI tool for setting up local LeetCode projects.

# Installation
Using cargo
```sh
cargo install leetr
```

From source
```sh
# Clone the repository
git clone https://github.com/pophilpo/leetr

# Navigate into the project directory
cd leetr

# Build the project in release mode
cargo build --release

# Copy the binary to your bin directory (adjust the path if necessary)
sudo cp target/release/leetr /usr/local/bin
```

# Usage

To use leetr, pass the URL of the LeetCode problem and the language of your choice (python/rust) to the tool. Note that the "description" part of the URL is mandatory as it is always present when opening the problem.

## Arguments
  - d, --dir <directory>  Create a directory with custom name for the problem, otherwise uses problem name
  - l, --lang <language>  Programming language used to setup the project [default: rust]
  - h, --help             Print help

This command generates a Python 3 project with the following structure:
- A `two_sum/README.md` file describing the problem.
- A `two_sum/main.py` file containing the initial problem code
```sh
leetr https://leetcode.com/problems/two-sum/description -l python
```

This command generates a Rust project with custom name and the following structure:
- A `my_project/README.md` file describing the problem.
- A `my_project/main.rs` file containing the initial problem code
```sh
leetr https://leetcode.com/problems/two-sum/description -l rust -d my_project
```

# Supported languages

- [x] Rust
- [x] Python 3
- [ ] Python
- [ ] C++
- [ ] Java
- [ ] C
- [ ] C#
- [ ] JavaScript
- [ ] TypeScript
- [ ] PHP
- [ ] Swift
- [ ] Kotlin
- [ ] Dart
- [ ] Go
- [ ] Ruby
- [ ] Scala
- [ ] Racket
- [ ] Erlang
- [ ] Elixir
