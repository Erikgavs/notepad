# Notepad App

<div align="center">

![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)
![Freya](https://img.shields.io/badge/Freya-GUI-blue?style=for-the-badge)
![License](https://img.shields.io/badge/License-MIT-green?style=for-the-badge)
![Platform](https://img.shields.io/badge/Platform-Linux-yellow?style=for-the-badge&logo=linux&logoColor=white)

**A simple, lightweight, and fast note-taking application built with Rust and Freya.**

[Features](#-features) • [Installation](#-installation) • [Usage](#-usage) • [Contributing](#-contributing)

</div>

---

## 📋 Overview

Notepad App is a minimalist desktop application designed for quick and efficient note-taking. Built entirely in Rust, it offers exceptional performance and a clean, distraction-free interface. Your notes are automatically saved in JSON format, ensuring your data is always secure and easily accessible.

> **Note:** Currently, this application is only compatible with **Linux**. Windows support is actively being developed and will be available in future releases.

---

## ✨ Features

| Feature | Description |
|---------|-------------|
| **Create Notes** | Add new notes with custom title and content through an intuitive popup dialog |
| **Delete Notes** | Remove unwanted notes with a single click |
| **Auto-Save** | All changes are automatically persisted to a local JSON file |
| **Lightweight** | Minimal resource usage thanks to Rust's efficiency |
| **Fast Startup** | Launch instantly with no loading delays |
| **Clean UI** | Distraction-free interface focused on productivity |
| **Portable Data** | Notes stored in human-readable JSON format |

---

## 🛠️ Tech Stack

- **Language:** [Rust](https://www.rust-lang.org/) (Edition 2024)
- **GUI Framework:** [Freya](https://github.com/marc2332/freya) v0.3.4
- **Serialization:** [Serde](https://serde.rs/) + serde_json

---

## 💻 System Requirements

### Supported Platforms

| Platform | Status |
|----------|--------|
| Linux | ✅ Fully Supported |
| Windows | 🚧 In Development |
| macOS | 📋 Planned |

### Prerequisites

- **For running the binary:**
  - Linux operating system
  - GUI environment (X11/Wayland)

- **For building from source:**
  - [Rust](https://www.rust-lang.org/tools/install) toolchain (Edition 2024)
  - Cargo (included with Rust)

---

## 📥 Installation

### Option 1: Download Pre-built Binary (Recommended)

1. Go to the [Releases](https://github.com/Erikgavs/notepad/releases) page
2. Download the latest `notepad_app` binary
3. Make it executable and run:

```bash
chmod +x notepad_app
./notepad_app
```

### Option 2: Build from Source

Clone the repository and build with Cargo:

```bash
# Clone the repository
git clone https://github.com/Erikgavs/notepad.git
cd notepad

# Build in release mode (optimized)
cargo build --release

# The binary will be at: target/release/notepad_app
```

---

## 🚀 Usage

### Starting the Application

```bash
# If built from source
./target/release/notepad_app

# Or if downloaded binary
./notepad_app
```

### Basic Operations

| Action | How to |
|--------|--------|
| **Create a note** | Click the "New note" button, fill in the title and content, then confirm |
| **View notes** | All notes are displayed in the main window as a list |
| **Delete a note** | Click the "Remove" button on any note |

### Data Storage

Your notes are automatically saved to a file called `notas.json` in the same directory where the application runs. This file:

- Is created automatically on first use
- Updates instantly when you add or delete notes
- Uses human-readable JSON format
- Can be backed up or transferred to another machine

**Example `notas.json` structure:**

```json
[
  {
    "title": "My First Note",
    "content": "This is the content of my note."
  },
  {
    "title": "Shopping List",
    "content": "Milk, eggs, bread, butter"
  }
]
```

---

## 📁 Project Structure

```
notepad/
├── Cargo.toml          # Project manifest and dependencies
├── README.md           # This file
├── .gitignore          # Git exclusions
├── notas.md            # Development notes
└── src/
    └── main.rs         # Application source code
```

---

## 🔧 Development

### Running in Development Mode

```bash
cargo run
```

### Building for Production

```bash
cargo build --release
```

### Code Architecture

The application follows a simple and clean architecture:

- **`Note` struct:** Represents a single note with `title` and `content` fields
- **`load_notes()`:** Reads and deserializes notes from the JSON file
- **`save_notes()`:** Serializes and writes notes to the JSON file
- **`app()`:** Main UI component using Freya's reactive signal system

---

## 🤝 Contributing

Contributions are welcome! Here's how you can help:

1. **Fork** the repository
2. **Create** a feature branch (`git checkout -b feature/amazing-feature`)
3. **Commit** your changes (`git commit -m 'Add amazing feature'`)
4. **Push** to the branch (`git push origin feature/amazing-feature`)
5. **Open** a Pull Request

### Ideas for Contribution

- [ ] Windows compatibility
- [ ] macOS compatibility
- [ ] Note search functionality
- [ ] Categories/Tags for notes
- [ ] Dark/Light theme toggle
- [ ] Export to different formats (TXT, Markdown)
- [ ] Note encryption
- [ ] Cloud sync support

---

## 🐛 Known Issues

- Application currently only runs on Linux
- Notes are stored locally (no cloud sync yet)

---

## 📄 License

This project is licensed under the **MIT License** - see the [LICENSE](LICENSE) file for details.

---

## 👤 Author

Developed with ❤️ by [Erikgavs](https://github.com/Erikgavs)

---

## ⭐ Support

If you find this project useful, please consider giving it a star on GitHub! It helps others discover the project.

---

<div align="center">

**[Back to Top](#notepad-app)**

</div>
