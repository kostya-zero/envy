# 📃 Envy

![Crates.io Version](https://img.shields.io/crates/v/envy) ![GitHub branch check runs](https://img.shields.io/github/check-runs/kostya-zero/envy/main)

The environment manager you'll *envy*. 
Envy provides a user-friendly command-line interface to manage environment variables in `.env` files and generate new `.env` files from `.env.example` files.

> [!NOTE]
> This project is in beta. Some changes in newer version may not be backward compatible with previous versions and may require actions from user for an update.

## Installation

You can install Envy with [Cargo](https://doc.rust-lang.org/cargo/) using the following commands:

```shell
# Compile and install Envy.
cargo install envy

# Install precompiled binaries (requires cargo-binstall).
cargo binstall envy
```

You can also install Envy from [GitHub Releases](https://github.com/kostya-zero/envy/releases).

## Usage

If you run Envy without any arguments, it will display the list of all keys in `.env` with their values.

```shell
$ envy
DATABASE_URL: postgres://example:example@localhost:5432/exampledb
JWT_SECRET: very_secret_passphrase
```

You can do basic manipulations with environment file.

```shell
# Create empty .env file / Generate from .env.example
envy new

# Create/update key's value.
envy set KEY value

# Remove key
envy remove KEY

# List all keys
envy list

# If you need help with commands
envy help
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
