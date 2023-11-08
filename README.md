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
git clone git@github.com:TheBuilderJR/c2p.git
cd c2p
chmod +x install.sh
./install.sh
```

This will build `c2p` and install the executable to `/usr/local/bin`.

## Usage

Execute `c2p` from the terminal, providing files or glob patterns:

```sh
c2p [FILES OR PATTERNS...]
```

So for example if you wanted to turn all js files in a /pages directory into a prompt you could run

```sh
c2p pages/**/*.js
```

On macOS, to copy contents to the clipboard, you can use `pbcopy`:

```sh
c2p pages/**/*.js | pbcopy
```

The above command will turn all js files in the pages/ directory into a single prompt and copy it to your clipboard, ready to be pasted into tools such as ChatGPT.

This can be a very powerful tool when working in a complex codebase with multiple files. Here's an example of one successful run where we use c2p to generate a prompt to augment an existing rust web application: https://chat.openai.com/share/3c674621-e526-45b7-bce8-10c38ee6c571

### Arguments

- `files_or_patterns`: A mandatory list of files or glob patterns to read.

## License

`c2p` is made available under the MIT License. See the `LICENSE.md` file for more information.
