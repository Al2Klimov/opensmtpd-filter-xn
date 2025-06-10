use std::io::{Write, stderr, stdin, stdout};

fn main() -> std::io::Result<()> {
    let mut std_out = stdout().lock();
    let mut std_err = stderr().lock();

    for l in stdin().lines() {
        let line = l?;
        let mut fields = line.split("|");

        match fields.next() {
            Some("config") => match fields.next() {
                Some("ready") => {
                    writeln!(std_out, "register|filter|smtp-in|ehlo")?;
                    writeln!(std_out, "register|ready")?;
                }
                _ => {}
            },
            Some("filter") => {
                fields.next(); // protocol version
                fields.next(); // timestamp
                fields.next(); // subsystem

                match (fields.next(), fields.next(), fields.next(), fields.next()) {
                    (Some("ehlo"), Some(session), Some(token), Some(mta)) => {
                        writeln!(
                            std_out,
                            "filter-result|{}|{}|{}",
                            session,
                            token,
                            if mta.contains("xn--") {
                                writeln!(std_err, "Denying")?;
                                "reject|550 Forbidden"
                            } else {
                                writeln!(std_err, "Allowing")?;
                                "proceed"
                            }
                        )?;
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }

    Ok(())
}
