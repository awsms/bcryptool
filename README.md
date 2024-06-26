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
bcryptool "my_password"
```

### Verifying a string against a hash

To verify if a string matches a provided bcrypt hash, use the --compare flag:

```sh
bcryptool --compare "$2y$12$NUnwvTu7dA2qMtP9/.EMvu5S8F52vDrAWX8VeiKszxM4wKYaZ.mE." "my_password"
```
