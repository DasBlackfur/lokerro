use lokerro::{ErrorExt as _, Result};

fn main() {
    perform_stacked_1().loc().unwrap();
}

fn perform_stacked_1() -> Result<()> {
    perform_stacked_2().loc()?;
    Ok(())
}

fn perform_stacked_2() -> Result<()> {
    perform_stacked_3().loc()?;
    Ok(())
}

fn perform_stacked_3() -> Result<()> {
    perform_bad_result_to_result().loc()?;
    Ok(())
}

fn perform_bad_result_to_result() -> Result<()> {
    external_bad_error()?;
    Ok(())
}

fn external_bad_error() -> core::result::Result<(), std::io::Error> {
    Err(std::io::ErrorKind::Deadlock.into())
}
