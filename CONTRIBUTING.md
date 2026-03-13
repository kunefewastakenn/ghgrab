# Contributing to ghgrab

First off, thank you for considering contributing to **ghgrab**! It's people like you who make the open-source community such an amazing place to learn, inspire, and create.

## How Can I Contribute?

### Reporting Bugs
If you find a bug, please create an issue. Include:
- Your OS and terminal (e.g., Windows 11, Windows Terminal/Powershell).
- Steps to reproduce the issue.
- Any error messages or screenshots.

### Suggesting Enhancements
Have an idea for a cool animation or a new feature? Open an issue and let's talk about it!

### Pull Requests
1. Fork the repo.
2. Create your feature branch (`git checkout -b feature/AmazingFeature`).
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`).
4. Push to the branch (`git push origin feature/AmazingFeature`).
5. Open a Pull Request.

## Development Setup

To build the project locally, you need [Rust](https://rustup.rs/) installed.

```bash
# Clone the repository
git clone https://github.com/abhixdd/ghgrab.git
cd ghgrab

# Build in debug mode
cargo build

# Run the TUI
cargo run
```

### Testing, Formatting, and Linting
Before you submit a Pull Request, please make sure everything is looking good:

```bash
# Run all tests to make sure nothing broke
cargo test

# Format the code
cargo fmt

# Check for any common mistakes
cargo clippy
```

If you've added new features, please try to add a test case in the `tests/` folder to cover it!

## 📜 Code of Conduct
Please be respectful and helpful to others. We aim to keep this project a welcoming space for everyone. See our [LICENSE](LICENSE) for more details.

## Design Guidelines
- **Stay Dynamic**: The TUI should feel alive. Prefer async operations over blocking ones.
- **Micro-animations**: Subtle transitions and toasts make the experience premium.
- **Cross-platform**: Always test on at least two different OSs if possible.


