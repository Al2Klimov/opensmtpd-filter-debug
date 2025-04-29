use itertools::Itertools;
use std::io::{stderr, stdin, stdout, Write};

fn main() -> std::io::Result<()> {
    let mut std_out = stdout();
    let mut std_err = stderr();

    for l in stdin().lines() {
        let line = l?;
        if line == "config|ready" {
            for event in [
                "link-connect",
                "link-greeting",
                "link-identify",
                "link-tls",
                "link-disconnect",
                "link-auth",
                "tx-reset",
                "tx-begin",
                "tx-mail",
                "tx-rcpt",
                "tx-envelope",
                "tx-data",
                "tx-commit",
                "tx-rollback",
                "protocol-client",
                "protocol-server",
                "filter-report",
                "filter-response",
                "timeout",
            ] {
                writeln!(std_out, "register|report|smtp-in|{}", event)?;
            }

            for event in [
                "connect",
                "helo",
                "ehlo",
                "starttls",
                "auth",
                "mail-from",
                "rcpt-to",
                "data",
                "data-line",
                "commit",
            ] {
                writeln!(std_out, "register|filter|smtp-in|{}", event)?;
            }

            writeln!(std_out, "register|ready")?;
        } else if line.starts_with("filter|") {
            let mut fields = line.split("|");

            fields.next(); // stream
            fields.next(); // protocol version
            fields.next(); // timestamp
            fields.next(); // subsystem

            match (fields.next(), fields.next(), fields.next()) {
                (Some(phase), Some(session), Some(token)) => {
                    if phase == "data-line" {
                        writeln!(
                            std_out,
                            "filter-dataline|{}|{}|{}",
                            session,
                            token,
                            fields.format("|")
                        )?;
                    } else {
                        writeln!(std_out, "filter-result|{}|{}|proceed", session, token)?;
                    }
                }
                _ => {}
            }
        }

        writeln!(std_err, "{}", line)?;
    }

    Ok(())
}
