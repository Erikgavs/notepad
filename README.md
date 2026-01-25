# Notepad App

A simple and lightweight note-taking application built with Rust and [Freya](https://github.com/marc2332/freya).

## Features

- Create notes with title and content
- Delete notes
- Automatic persistence (saves to JSON)
- Clean and minimal interface

## Installation

### Download binary

1. Download `notepad_app` from [Releases](https://github.com/your-username/notepad/releases)
2. Make it executable:
   ```bash
   chmod +x notepad_app
   ```
3. Run it:
   ```bash
   ./notepad_app
   ```

### Build from source

**Prerequisites:** [Rust](https://www.rust-lang.org/tools/install) (edition 2024)

```bash
git clone https://github.com/your-username/notepad.git
cd notepad
cargo build --release
```

The binary will be available at `target/release/notepad_app`.

## Usage

```bash
./target/release/notepad_app
```

Notes are automatically saved to `notas.json` in the current directory.

## License

MIT