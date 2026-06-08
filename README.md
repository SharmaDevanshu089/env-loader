# env-loader

A simple Rust crate for reading `.env` files. That's pretty much it.

## Features

- Skips comments 
- Skips Broken Key Value Pairs
- Returns Error
- Thats it DUH !

## What it does

- Reads `.env` files and parses them
- Returns the key-value pairs as basic data structures
- Uses only `std` library (no external dependencies)
- Does not automatically load variables into your environment (just reads and gives you the data)

## Why?

Built for the [env-explorer](https://github.com/SharmaDevanshu089/Env-Explorer) project. Just needed something to write and forget.

## Usage
Use the simple funtion to read and load a hashmap in a variables and print them : |

```
 let env_vars = read_env_file(".env").unwrap();
 println!({:?});
```

## Notes

- Very basic, no feature set
- Not expecting many updates (it does one thing)
- Pull requests welcome (Js Rewrite it imo)

---

MIT LISENCSE , MADE BY DEVANSHU
