extern crate systemd;
use systemd::journal::{Journal, JournalFiles, JournalSeek, JournalRecord};
use systemd::Result;

pub fn open_journal() -> Result<Journal> {
    let mut j =
        Journal::open(JournalFiles::All, false, false)?;
    j.seek(JournalSeek::Tail)?;
    Ok(j)
}

fn run() -> Result<()> {
    let mut j = open_journal()?;
    j.watch_all_elements(f)?;
    Ok(())
}

pub fn f(rec: JournalRecord) -> Result<()> {
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

        ::std::process::exit(1);
    }
}
