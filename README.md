# env-loader

Reads `.env` files and gives you the key-value pairs. No bells, no whistles, no dependencies.

## What it does

- Parses `.env` files into a `HashMap<String, String>`
- Skips comments (`#`) and empty lines
- Skips malformed lines instead of panicking
- Returns a `Result` so you handle the error your way
- Zero external dependencies — pure `std`
- Does **not** load variables into your environment, just reads them

## Why?

Built for the [Env-Explorer](https://github.com/SharmaDevanshu089/Env-Explorer) project. Needed something small to read `.env` files without pulling in a full dotenv crate.

## Install

Add this to your `Cargo.toml`:

```toml
[dependencies]
env-loader = "1.0.0"
```

## Usage

```rust
use env_loader::read_env_file;

fn main() {
    let env_vars = read_env_file(".env").unwrap();

    for (key, value) in &env_vars {
        println!("{} = {}", key, value);
    }

    // or grab a specific key
    if let Some(token) = env_vars.get("GITHUB_TOKEN") {
        println!("token: {}", token);
    }
}
```

Your `.env` file:

```env
# this is a comment, gets skipped
GITHUB_TOKEN=ghp_xxxx
API_KEY=some_secret

BROKEN_LINE_GETS_SKIPPED
```

## Notes

- Does one thing, does it fine
- Not planning major updates
- PRs welcome if something's broken

---

MIT License — Devanshu Sharma