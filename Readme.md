# zpush 🚀
[![Rust](https://github.com/copyleftdev/zpush/actions/workflows/rust.yml/badge.svg)](https://github.com/copyleftdev/zpush/actions/workflows/rust.yml)
Welcome to **zpush** – a Rust-powered CLI tool that makes pushing your environment secrets to GitHub as fun as it is functional! With a colorful UI, clever progress bars, and playful emojis, zpush turns mundane secret management into an enjoyable experience.

---

## Overview 🎨

zpush is designed for developers who need to securely push local environment secrets to a GitHub repository. It automatically detects if you’re in a Git repository root, loads environment variables (from a default or user-specified `.env` file), verifies your GitHub token, and then uses GitHub’s API to upload your secrets. All while keeping you entertained with vibrant progress bars and witty status messages.

---

## Features ✨

- **Automatic Repository Detection**  
  Ensures you’re executing from the root of your Git repository by checking for a `.git` folder and extracting the remote “origin” info.

- **Flexible Environment Loading**  
  Loads your environment variables from a default `.env` file or a user-specified file via the `--env-file` option.

- **Secret File Parsing**  
  Reads your secrets file (each line formatted as `KEY=VALUE`), ignoring empty lines and comments.

- **GitHub Integration**  
  Authenticates with GitHub using a token (make sure to set the `GITHUB_TOKEN` environment variable with proper scopes) and verifies it by calling GitHub’s API.

- **Asynchronous & Responsive**  
  Built on Rust’s async ecosystem (using Tokio and reqwest) for a snappy, non-blocking experience.

- **Vibrant, Emoji-Fueled UI**  
  Enjoy colorful progress bars (courtesy of indicatif) and status messages loaded with fun emojis 😎🎉.

- **Robust Testing & Installation**  
  Comes with a comprehensive test suite and a Makefile that supports building, running, testing, cleaning, and even a user-local install to `$(HOME)/.local/bin`.

---

## Installation 🔧

Make sure you have [Rust](https://www.rust-lang.org/) installed, then clone this repository:

```bash
git clone https://github.com/copyleftdev/zpush.git
cd zpush
```

### Build & Install Locally

Use the provided Makefile to build and install zpush to your local binary directory:

```bash
make install
```

This command will compile the project in release mode and copy the `zpush` binary to `$(HOME)/.local/bin`. Be sure that `$(HOME)/.local/bin` is in your PATH!

---

## Usage ⚙️

1. **Set Up Your GitHub Token**  
   Export your GitHub token (with the required scopes such as `repo`, `admin:repo_hook`, and `secrets`):

   ```bash
   export GITHUB_TOKEN="your_token_here"
   ```

2. **Prepare Your Secrets File**  
   Create a secrets file (e.g., `secrets.txt`) with each secret in the format:

   ```plaintext
   API_KEY=your_api_key
   DB_PASSWORD=your_db_password
   ```

3. **Run zpush**  
   Navigate to your Git repository root (ensure the `.git` folder is present) and run:

   ```bash
   zpush --secrets-file secrets.txt
   ```

   To load a custom environment file instead of the default `.env`, use:

   ```bash
   zpush --secrets-file secrets.txt --env-file custom.env
   ```

Watch the colorful progress bar and fun emojis as your secrets are pushed!

---

## Development & Testing 🧪

Run the full test suite with:

```bash
cargo test
```

The repository includes unit tests for secret parsing, token verification, and other core functions. You can also use the Makefile’s `test` target:

```bash
make test
```

---

## Contributing 🤝

Contributions are welcome! If you have ideas, improvements, or bug fixes:
- Fork this repository.
- Create a feature branch.
- Open a pull request with a clear description of your changes.

For more details, please review our [CONTRIBUTING guidelines](CONTRIBUTING.md) (if provided).

---

## License 📄

zpush is released under the **MIT License**. See the [LICENSE](LICENSE) file for more details.

---

## Acknowledgements 💖

Built with passion by the copyleftdev community. Enjoy a vibrant, efficient, and fun secret management experience – and happy coding! 😎✨
