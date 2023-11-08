````markdown
# c2p - Code to Prompt

`c2p` (code to prompt) is a Rust command-line application designed to facilitate the copying of code snippets directly from your filesystem into a clipboard, making it extremely handy for transferring code into prompt-based interfaces or tools like ChatGPT.

## Features

- **Multiple File Handling**: Support for multiple file arguments or glob patterns to process numerous files in one command.
- **Glob Pattern Support**: Utilize glob patterns for file selection to streamline the process of copying file contents.
- **Clipboard Integration**: Direct integration with clipboard utilities like `pbcopy` for macOS, allowing for immediate pasting into other applications.

## Installation

Before installing `c2p`, make sure you have Rust and Cargo installed on your system. If not, please follow the instructions on [the official Rust website](https://www.rust-lang.org/tools/install).

Install `c2p` by cloning the repository and running the provided install script:

```sh
git clone https://your-repository-url/c2p.git
cd c2p
chmod +x install.sh
./install.sh
```
````

This will build `c2p` and install the executable to `/usr/local/bin`.

## Usage

Execute `c2p` from the terminal, providing files or glob patterns:

```sh
c2p [FILES OR PATTERNS...]
```

On macOS, to copy contents to the clipboard, you can use `pbcopy`:

```sh
c2p path/to/source_code.rs | pbcopy
```

The above command will pipe the contents of `source_code.rs` to the clipboard, ready to be pasted into tools such as ChatGPT.

Here's an example of one successful run: https://chat.openai.com/share/3c674621-e526-45b7-bce8-10c38ee6c571

### Arguments

- `files_or_patterns`: A mandatory list of files or glob patterns to read.

## License

`c2p` is made available under the MIT License. See the `LICENSE.md` file for more information.
