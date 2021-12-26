# Rice Package Installer

> This program can install packages from lists.

[![asciicast](https://asciinema.org/a/458622.svg)](https://asciinema.org/a/458622)

# Usage

```bash
rice-pkg-installer [FILENAME]
```

# Test Without Installing
```bash
git clone https://github.com/LukasDoesDev/rice-pkg-installer
cd rice-pkg-installer

cargo build --release
export PATH="$PWD/target/release:$PATH"

rice-pkg-installer
```
