use stage::StageMain;
use errors::*;

/// Build an N64-compatible, bid-endian binary from an input StageMain struct
pub fn build_binary(input: &StageMain, verbose: bool) -> Result<Vec<u8>>
{
    let stage_binary = input.as_bytes()
        .chain_err(||"creating stage binary file")?;

    if verbose {
        println!("Converted stage binary:\n{:?}", stage_binary);
    }

    Ok(stage_binary)
}
