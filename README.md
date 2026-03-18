# helsinki

Quickly apply pre-configured git config profiles to local repositories.

When working with multiple Git accounts (work, personal, OSS, etc.), you need to set `user.name`, `user.email`, and other config per repository. **helsinki** saves these as named profiles and applies them with a single command.

## Usage

### Register profiles

```sh
helsinki config work user.name "John Doe"
helsinki config work user.email "work@company.com"

helsinki config personal user.name "johndoe"
helsinki config personal user.email "me@example.com"
helsinki config personal user.signingkey "ABC123"
```

### Apply a profile

```sh
# Specify directly
helsinki set work

# Or select interactively
helsinki set
```

### Other commands

```sh
# View a config value
helsinki config work user.name
# => John Doe

# List all profiles
helsinki list

# Remove a profile
helsinki remove work

# Require local user config globally (prevents git from guessing identity)
helsinki global
```

### Config file

Profiles are stored in `~/.config/helsinki/helsinki.toml`:

```toml
[work]
user.name = "John Doe"
user.email = "work@company.com"

[personal]
user.name = "johndoe"
user.email = "me@example.com"
user.signingkey = "ABC123"
```

The `XDG_CONFIG_HOME` environment variable is respected if set.

## Installation

### From crates.io

```sh
cargo install helsinki-cli
```

### From source

```sh
cargo install --path .
```

### Build from repository

```sh
git clone https://github.com/nazuna315/helsinki.git
cd helsinki
cargo build --release
```

## License

[MIT](LICENSE)
