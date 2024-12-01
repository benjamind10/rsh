
# rsh - A Rust-Based Shell

`rsh` (Rust Shell) is a lightweight, customizable command-line shell written in Rust. It provides basic shell functionalities like executing commands, autocompletion for commands and paths, and history-based suggestions. Inspired by the functionality of `zsh`, `rsh` is designed to be a starting point for building a more advanced and feature-rich shell.

---

## Features

- **Command Execution**: Run any command available in your system's `PATH`.
- **Built-In Commands**:
  - `cd`: Change directories.
  - `exit`: Exit the shell.
- **Autocomplete**:
  - Command suggestions from system `PATH`.
  - File and directory path suggestions.
  - History-based suggestions for previously entered commands.
- **Graceful Exit**:
  - Handles `Ctrl+C` and `Ctrl+D` to terminate the shell cleanly.

---

## Installation

### Prerequisites
- [Rust](https://www.rust-lang.org/) installed on your system.

### Clone and Build

```bash
git clone https://github.com/yourusername/rsh.git
cd rsh
cargo build --release
```

### Run

```bash
cargo run
```

---

## Usage

### Start the Shell

```bash
cargo run
```

### Basic Commands
- **Run a Command**:
  ```bash
  rsh> ls
  ```
- **Change Directory**:
  ```bash
  rsh> cd /path/to/directory
  ```
- **Exit the Shell**:
  - Type `exit`, or
  - Press `Ctrl+C` or `Ctrl+D`.

### Autocomplete
- Press `Tab` to autocomplete commands, paths, or history:
  - **Command Suggestions**:
    ```bash
    rsh> ls [Tab]
    ```
  - **Path Suggestions**:
    ```bash
    rsh> cd /ho[Tab]
    ```
    Completes to:
    ```bash
    rsh> cd /home/
    ```
  - **History Suggestions**:
    ```bash
    rsh> echo hello
    rsh> ec[Tab]
    ```
    Completes to:
    ```bash
    rsh> echo hello
    ```

---

## Development

### Project Structure

```
rsh/
├── src/
│   ├── main.rs           # Main entry point
│   ├── shell/
│   │   ├── mod.rs        # Shell module
│   │   ├── commands.rs   # Command execution logic
│   │   ├── completer.rs  # Autocompletion logic
│   │   ├── prompt.rs     # (Optional) Prompt display logic
│   │   ├── input.rs      # (Optional) Input handling logic
│   └── utils/
│       ├── mod.rs        # Utility functions
```

### Add New Features
- **Add New Built-In Commands**:
  1. Modify `main.rs` to detect the new command.
  2. Add the corresponding functionality in `shell/commands.rs`.

- **Extend Autocompletion**:
  1. Update `FilePathAndCommandCompleter` in `shell/completer.rs`.

---

## Future Enhancements

- **Command Aliases**: Allow users to define and use aliases for commands.
- **Scripting Support**: Execute scripts directly from the shell.
- **Configuration File**: Allow customization via a `.rshrc` file.
- **Colored Prompts and Output**: Add dynamic and colorful prompts and outputs.

---

## Contributing

Contributions are welcome! Please open issues or submit pull requests on the [GitHub repository](https://github.com/yourusername/rsh).

---

## License

This project is licensed under the MIT License. See the `LICENSE` file for details.

---

## Acknowledgments

- Inspired by the simplicity and functionality of `zsh` and `bash`.
- Built with love using Rust.
