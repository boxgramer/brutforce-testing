
# BruteForce HTTP Tool

A simple Rust-based HTTP brute force tool that supports dynamic parameters, wordlist combinations, and high-performance concurrency using `tokio`.

## ✨ Features

- Supports HTTP methods: `GET`, `POST`, `PUT`, `DELETE`
- Static parameters or wordlist-based input
- Concurrent HTTP requests with `tokio`
- Tabular output: URL, method, status code, RTT (ms), and body size
- Fast performance using `reqwest` and async runtime

## 🚀 Usage

### 🔧 Build

```bash
cargo build --release
```

### ▶️ Run

#### 1. Using Static Parameters

```bash
./bruteforce -u https://example.com -m POST -p username:admin -p password:admin
```

#### 2. Using Wordlists

```bash
./bruteforce -u https://example.com/login -m POST -w username:user.txt -w password:pass.txt
```

> Use the format `key:path_to_file` for the `--wordlist` argument.

### ⌘ CLI Arguments

| Argument        | Alias | Description                                                   |
|------------------|-------|---------------------------------------------------------------|
| `--url`          | `-u`  | Target URL to test                                            |
| `--method`       | `-m`  | HTTP method (`GET`, `POST`, `PUT`, `DELETE`)                 |
| `--params`       | `-p`  | Static parameters in `key:value` format                      |
| `--wordlist`     | `-w`  | Wordlist input as `key:path/to/wordlist.txt`                |

## 📆 Example Wordlist Format

```
username:usernames.txt
password:passwords.txt
```

Content of `usernames.txt`:
```
admin
root
test
```

## 📅 Output Format

The result will be displayed as a table in the terminal:

```
+------------------------+--------+--------+----------+-------------+
| URL                    | Method | Status | RTT(ms) | Size Body   |
+------------------------+--------+--------+----------+-------------+
| https://example.com    | POST   | 200    | 89      | 1234        |
+------------------------+--------+--------+----------+-------------+
```

## 📄 Disclaimer

**This tool is intended for educational purposes only.**  
Any misuse of this tool for malicious or illegal activities is strictly prohibited.  
The author is **not responsible** for any damage or consequences caused by the use of this tool.  
Use it **only on systems you own or have explicit permission to test**.

## 🛠 Dependencies

- [tokio](https://crates.io/crates/tokio)
- [reqwest](https://crates.io/crates/reqwest)
- [clap](https://crates.io/crates/clap)
- [prettytable-rs](https://crates.io/crates/prettytable-rs)
- [futures](https://crates.io/crates/futures)

## 📄 License

MIT License.

---

Built with ❤️ using Rust.

