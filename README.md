# env-loader

A simple Rust crate for reading `.env` files. That's pretty much it.

## What it does

- Reads `.env` files and parses them
- Returns the key-value pairs as basic data structures
- Uses only `std` library (no external dependencies)
- Does not automatically load variables into your environment (just reads and gives you the data)

## Why?

Built for the [env-explorer](https://github.com/SharmaDevanshu089/Env-Explorer) project. Needed something lightweight to parse `.env` files without all the bells and whistles of bigger libraries.

## Usage
Use the simple funtion to read and load a hashmap in a variables

```
 let env_vars = read_env_file(".env").unwrap();
```

## Notes

- Very basic, minimal feature set
- Not expecting many updates (it does one thing)
- Pull requests welcome if you find bugs though

---

Made with ☕ and Rust
