use std::io;

use std::process::ExitCode;

fn sub() -> Result<(), io::Error> {
    let i = std::io::stdin();
    let mut il = i.lock();
    //let cnt: usize = rs_line_count::simple::splited2count(il, b'\n');
    let cnt: usize = rs_line_count::reads::reads2count4k(&mut il, b'\n')?;
    println!("{cnt}");
    Ok(())
}

fn main() -> ExitCode {
    sub().map(|_| ExitCode::SUCCESS).unwrap_or_else(|e| {
        eprintln!("{e}");
        ExitCode::FAILURE
    })
}
