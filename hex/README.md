# hex

A command-line tool for converting between hexadecimal and decimal numbers.

## Installation

1. Clone this repository:

```bash
git clone git@github.com:heyskylark/bin.git
cd hex
```

2. Run the installation script:

```bash
chmod +x install.sh
./install.sh
```

The tool will be installed as `hex` in `$HOME/.skylark/bin/`. The installation script will automatically add this directory to your PATH if it's not already there.

If this is your first time installing, you'll need to reload your shell configuration:

```bash
source ~/.zshrc
# or
source ~/.bashrc
```

## Usage

Convert hex to decimal:

```bash
hex 0xff -x
# Output: 255
```

Convert decimal to hex:

```bash
hex 255 -i
# Output: 0xff
```

## Options

- `-x, --hex`: Force hex to integer conversion
- `-i, --int`: Force integer to hex conversion

## Examples

```bash
# Convert hex to decimal
hex 0xff -x
hex 0x1a -x

# Convert decimal to hex
hex 255 -i
hex 26 -i
```
