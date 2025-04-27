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

            writeln!(std_out, "register|ready")?;
        }

        writeln!(std_err, "{}", line)?;
    }

    Ok(())
}
