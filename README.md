# Mouse Drawing with Ratatui

![Mouse Drawing](https://img.shields.io/badge/Made%20with-Ratatui-blue?style=flat-square)

A simple command-line application that demonstrates how to handle mouse events and draw on the terminal using the **Ratatui** library.

## Features

- **Mouse Interaction**: Draw lines by clicking and dragging the mouse.
- **Color Picker**: Change the drawing color by pressing the spacebar.
- **Exit Option**: Press `q` or `Esc` to quit the application.
- **Terminal Graphics**: Render colorful, interactive graphics in your terminal using `ratatui` and `crossterm`.

## Requirements

- **Rust**: Make sure you have Rust installed. You can install it from [rust-lang.org](https://www.rust-lang.org/).
- **crossterm**: A cross-platform terminal manipulation library.
- **ratatui**: A TUI (Text User Interface) library that lets you create interactive terminal apps.

## Installation

To get started with this project, clone the repository and build it locally.

```bash
git clone https://github.com/oleeeedev/mouse-drawing-ratatu.git
cd mouse-drawing
cargo build --release
