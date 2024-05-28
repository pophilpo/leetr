# Leetr

A CLI tool for setting up local LeetCode projects.

# Installation
Currently, there is no convenient way to install this tool via package managers. You need to build it from the source and copy the binary to your `bin` directory.

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

To use leetr, pass the URL of the LeetCode problem to the tool. Note that the "description" part of the URL is mandatory as it is always present when opening the problem.
```sh
leetr https://leetcode.com/problems/two-sum/description
```
This command generates a Rust project with the following structure:
- A `README.md` file describing the problem.
- A `main.rs` file containing the initial problem code.

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
