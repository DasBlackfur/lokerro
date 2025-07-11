# Lokerro - Yet another error handeling crate

There is no *real* reason for this to exist. I simply prefer this.

Lokerro aims to provide simple error handling, comparable to `anyhow`.
It however has a few advantages:
- Relatively inexpensive location information compared to full backtraces
- Compatibility with bad error types (those who do not implement Error)
- Compatibility with complex error types (those who have hundres of lifetimes in their type specification)
It also comes with a disadvantage you should consider if you are building a library:
- Its error type cannot be used in other error handling code, meaning you will be locked into using lokerro for all downstream handling

Here is an example how lokerro might be used in an application:
```rust
use std::{collections::HashMap, fs::File, io::Read};
use lokerro::{ErrorExt, Result};

fn main() -> Result<()> {
    let result = load_settings().loc_msg("Loading settings failed")?;

    println!("{result:?}");

    Ok(())
}

fn load_settings() -> Result<HashMap<u32, String>> {
    let mut config = HashMap::new();
    let config_1 = process_file("this_file_may_not_exist.txt").loc()?;
    let config_2 = process_file("this_file_neither.txt").loc()?;

    config.insert(1, config_1);
    config.insert(2, config_2);

    Ok(config)
}

fn process_file(path: &str) -> Result<String> {
    let mut file = File::open(path)?;
    let mut string = String::new();

    file.read_to_string(&mut string)?;
    Ok(string)
}
```

This example will exit with code 1 and print out the following:
```
Error: Loading settings failed in examples\realistic.rs:5:34
Caused by: lokerro::Error in examples\realistic.rs:14:64
Caused by: std::io::error::Error in examples\realistic.rs:24:20
```