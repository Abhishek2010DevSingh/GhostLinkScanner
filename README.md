# 👻 Ghost Link Scanner

Ghost Link Scanner is an ⚡ **asynchronous Rust-based** tool designed to scan a given base URL for **broken (dead) links**. It extracts all anchor (`<a>`) tags, checks their HTTP status, and reports any **non-working** links.

## 🚀 Features
- 🔄 **Asynchronous Scanning**: Uses `tokio` for concurrent requests.
- ⚡ **Fast & Efficient**: Scans multiple links in parallel using `JoinSet`.
- 📜 **Structured Logging**: Logs requests and results with `tracing`.
- 🎨 **Pretty Output**: Displays dead links in a structured table.
- ❌ **Error Handling**: Uses `anyhow` for better error context.

## 🔧 Installation
To use Ghost Link Scanner, ensure you have Rust installed. If not, install it using [Rustup](https://rustup.rs/):

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Then, clone this repository and build the project:

```sh
git clone https://github.com/Abhishek2010DevSingh/GhostLinkScanner
cd GhostLinkScanner
cargo build --release
```

## 🛠️ Usage
Run the scanner with:

```sh
cargo run --release -- --url https://example.com
```

### 📊 Example Output
```
🔍 Starting scan for URL: https://example.com
✅ OK: https://example.com/about
❌ Dead (Status: 404): https://example.com/missing-page

📌 Dead Links Found:
+---+-------------------------------+------------------+
| # | URL                           | Status          |
+---+-------------------------------+------------------+
| 1 | https://example.com/404       | Dead (404)      |
+---+-------------------------------+------------------+
```

## 📦 Dependencies
This project uses the following Rust crates:
- 🔗 [`reqwest`](https://docs.rs/reqwest) - HTTP client
- 📝 [`scraper`](https://docs.rs/scraper) - HTML parsing
- ⚙️ [`tokio`](https://docs.rs/tokio) - Asynchronous runtime
- 🎛️ [`clap`](https://docs.rs/clap) - Command-line argument parsing
- 📢 [`tracing`](https://docs.rs/tracing) - Logging
- 🛠️ [`anyhow`](https://docs.rs/anyhow) - Error handling
- 📊 [`prettytable-rs`](https://docs.rs/prettytable-rs) - Table formatting

## 🤝 Contributing
Feel free to **fork** this repository, create a **branch**, and submit a **pull request**! Contributions are always **welcome**. 🚀

## 📜 License
This project is licensed under the **MIT License**. See `LICENSE` for details.
