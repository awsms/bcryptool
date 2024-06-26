# bcryptool

`bcryptool` is a command-line utility written in Rust for hashing passwords with bcrypt and verifying strings against bcrypt hashes.

## Features

- Hash a string using bcrypt.
- Verify if a given string matches a provided bcrypt hash.

## Installation

1. **Clone the repository**:
    ```sh
    git clone https://github.com/awsms/bcryptool.git
    cd bcryptool
    ```

2. **Build the project**:
    ```sh
    cargo build --release
    ```

## Usage

### Hashing a string

To hash a string, simply provide the string as an argument:

```sh
./target/release/bcryptool "my_password"
