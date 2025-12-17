# `ev`

![Crates.io Version](https://img.shields.io/crates/v/ev-manager) ![GitHub branch check runs](https://img.shields.io/github/check-runs/kostya-zero/ev/main)

Control your environment.

`ev` provides a user-friendly command-line interface to manage environment variables in `.env` files and generate new `.env` files from `.env.example` files.

> [!NOTE]
> This project is in beta. Some changes in newer version may not be backward compatible with previous versions and may require actions from user for an update.

## Installation

You can install `ev` with [Cargo](https://doc.rust-lang.org/cargo/) using the following commands:

```shell
# Compile and install ev.
cargo install ev-manager

# Install precompiled binaries (requires cargo-binstall).
cargo binstall ev-manager
```

You can also install `ev` from [GitHub Releases](https://github.com/kostya-zero/ev/releases).

## Usage

If you run `ev` without any arguments, it will display the list of all keys in `.env` with their values.

```shell
$ ev
DATABASE_URL: postgres://example:example@localhost:5432/exampledb
JWT_SECRET: very_secret_passphrase
```

You can do basic manipulations with environment file.

```shell
# Create empty .env file / Generate from .env.example
ev new

# Create/update key's value.
ev set KEY value

# Remove key
ev remove KEY

# List all keys
ev list

# If you need help with commands
ev help
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
