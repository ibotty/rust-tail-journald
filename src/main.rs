#[macro_use]
extern crate error_chain;
extern crate systemd;
use systemd::journal::{Journal, JournalFiles, JournalSeek, JournalRecord};

mod errors {
    error_chain!{}
}
use errors::*;

pub fn open_journal() -> Result<Journal> {
    let mut j =
        Journal::open(JournalFiles::All, false, false).chain_err(|| "unable to open journal")?;
    j.seek(JournalSeek::Tail).chain_err(|| "cannot seek to head")?;
    Ok(j)
}

fn run() -> Result<()> {
    let mut j = open_journal()?;
    j.watch_all_elements(f).chain_err(|| "unable to watch all elements")?;
    Ok(())
}

pub fn f(rec: JournalRecord) -> systemd::Result<()> {
    for (key, value) in rec.iter() {
        println!("{}: {}", key, value)
    }
    Ok(())
}

fn main() {
    if let Err(ref e) = run() {
        use std::io::Write;
        let stderr = &mut ::std::io::stderr();
        let errmsg = "Error writing to stderr";

        writeln!(stderr, "error: {}", e).expect(errmsg);

        for e in e.iter().skip(1) {
            writeln!(stderr, "caused by: {}", e).expect(errmsg);
        }

        // The backtrace is not always generated. Try to run this example
        // with `RUST_BACKTRACE=1`.
        if let Some(backtrace) = e.backtrace() {
            writeln!(stderr, "backtrace: {:?}", backtrace).expect(errmsg);
        }

        ::std::process::exit(1);
    }
}
