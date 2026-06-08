use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn read_env_file(file_path: &str) -> Result<HashMap<String, String>, std::io::Error> {
    let mut env_vars = HashMap::new();
    let file = File::open(file_path)?;
    let reader = std::io::BufReader::new(file);

    for line in reader.lines() {
        if let Ok(line) = line {
            if Box::new(&line).starts_with('#') || line.trim().is_empty() {
                continue;
            }
            if let Some((key, value)) = line.split_once('=') {
                if !key.trim().is_empty() && !value.trim().is_empty() {
                    env_vars.insert(key.trim().to_string(), value.trim().to_string());
                }
            }
        }
    }
    Ok(env_vars)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_read_env_file() {
        let env_vars = read_env_file(".env").unwrap();
        println!("{:?}", env_vars);
        assert_eq!(env_vars.get("KEY1"), Some(&"VALUE1".to_string()));
        assert_eq!(env_vars.get("KEY2"), Some(&"VALUE2".to_string()));
    }
}
