# mailpipe

[![Crates.io](https://img.shields.io/crates/v/mailpipe?style=flat-square)](https://crates.io/crates/mailpipe)
[![Docs.rs](https://img.shields.io/docsrs/mailpipe?style=flat-square)](https://docs.rs/mailpipe)
[![Rust](https://img.shields.io/badge/rust-2021_edition-orange?style=flat-square&logo=rust)](https://www.rust-lang.org)
![Status](https://img.shields.io/badge/status-early_development-yellow?style=flat-square)

A unified, high-level SMTP and IMAP email engine backend library for Rust, built on top of [async-imap](https://crates.io/crates/async-imap) and [lettre](https://crates.io/crates/lettre). It's the email backend that powers [magpipe](https://github.com/watson/magpipe).

---

## Features

- **IMAP** — Connect, authenticate, and manage sessions over TLS (port 993)
- **SMTP** — *(coming soon)*
- Async-first via [Tokio](https://tokio.rs)
- Credentials are never stored — passwords are passed transiently at connection time

## Installation

```toml
[dependencies]
mailpipe = "0.0.1"
```

## Usage

### IMAP

```rust
use mailpipe::imap::ImapConnector;

#[tokio::main]
async fn main() {
    let connector = ImapConnector::new("imap.example.com", "user@example.com");

    let session = connector
        .connect("your-password")
        .await
        .expect("failed to connect");

    // ... use session ...

    session.logout().await.expect("failed to logout");
}
```

#### Custom port

```rust
let mut connector = ImapConnector::new("imap.example.com", "user@example.com");
connector.port = 143;
```

## Security

Passwords are accepted as a transient `&str` and are never stored on any struct. It is recommended to source credentials from environment variables or a secrets manager rather than hardcoding them.

```rust
let pass = std::env::var("IMAP_PASS").expect("IMAP_PASS not set");
connector.connect(&pass).await?;
```

## Dependencies

| Crate | Purpose |
|---|---|
| `tokio` | Async runtime |
| `async-imap` | IMAP protocol implementation |
| `tokio-native-tls` | TLS encryption |
| `lettre` | SMTP *(upcoming)* |
