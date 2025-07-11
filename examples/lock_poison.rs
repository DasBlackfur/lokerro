use std::{sync::{Arc, Mutex}, thread};

use lokerro::{ErrorExt as _, Result};

fn main() -> Result<()> {
    run_lock().loc()?;

    Ok(())
}

fn run_lock() -> Result<()> {
    let mutex = Arc::new(Mutex::new(1));
    let c_mutex = Arc::clone(&mutex);
    let _ = thread::spawn(move || {
        let mut data = c_mutex.lock().unwrap();
        *data = 2;
        panic!();
    }).join();

    let _guard = mutex.lock()?;

    Ok(())
}

