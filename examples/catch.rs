use lokerro::{ErrorExt as _, ErrorExtCompat as _, Result};

fn main() {
    eprintln!("{}", perform_stacked_1().loc().unwrap_err());
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
    external_bad_error().loc_compat()?;
    Ok(())
}

enum BadError {
    CommonMistake,
}

fn external_bad_error() -> core::result::Result<(), BadError> {
    Err(BadError::CommonMistake)
}
