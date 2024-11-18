# Silo 🔄

## Overview

Silo is a lightweight command-line utility for converting between YAML and JSON file formats with ease.

## Features

- Convert YAML to JSON
- Convert JSON to YAML
- Support for file input and output
- Simple and intuitive CLI

## Installation

```bash
cargo install silo
```

## Usage

### Convert YAML to JSON

```bash
silo path/to/file.yaml --json [-o output.json]
```

### Convert JSON to YAML

```bash
silo path/to/file.json --yaml [-o output.yaml]
```

### Options

- `filepath`: Path to the input file (required)
- `--json`, `-j`: Convert to JSON format
- `--yaml`, `-y`: Convert to YAML format
- `--output`, `-o`: Optional output file path (prints to console if not specified)

## Requirements

- Rust
- Cargo

## Dependencies

- serde
- serde_json
- serde_yaml
- structopt

## License

[The GNU General Public License v3.0 ](LICENSE)

## Contributing

Pull requests are welcome. For major changes, please open an issue first to discuss proposed modifications.
