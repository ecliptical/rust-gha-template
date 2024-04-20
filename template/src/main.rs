use anyhow::bail;
use gha::*;
use std::env;
use std::fs::File;
use std::io::BufWriter;

fn main() -> anyhow::Result<()> {
    let mut arg1 = String::default();
    let mut arg2 = false;

    for arg in env::args().skip(1) {
        let Some((name, value)) = arg.split_once('=') else {
            bail!("illegal argument: {arg}");
        };

        match name {
            "--arg1" => arg1 = value.to_string(),
            "--arg2" => arg2 = value == "true",
            _ => bail!("unknown argument: {name}"),
        }
    }

    debug!("arg1 = {arg1}");
    debug!("arg2 = {arg2}");

    let out = File::options()
        .create(true)
        .append(true)
        .open(github_output())?;

    let mut out = BufWriter::new(out);
    append_name_value(
        &mut out,
        "out1",
        &format!("arg1 is {arg1} and arg2 is {arg2}"),
    )?;

    Ok(())
}
