extern crate systemd;
use std::io::Result;
use systemd::journal::{Journal, JournalFiles, JournalSeek, JournalRecord};

pub fn open_journal() -> Result<Journal> {
    let mut j = Journal::open(JournalFiles::All, false, false)?;
    j.seek(JournalSeek::Tail)?;
    Ok(j)
}

fn run() -> Result<()> {
    let mut j = open_journal()?;
    for candidate in j.iterate_from_cursor_waiting_for_new_records() {
        match candidate {
            Err(error) => {
                // Error code 74 is BADMSG
                // Skip invalid records (due to corrupt journal)
                if error.raw_os_error() != Some(74) {
                    return Err(error);
                }
            },
            Ok(item) => f(item)?
        }
    }
    Ok(())
}

pub fn f(rec: JournalRecord) -> Result<()> {
    println!("{:?}", rec);
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
