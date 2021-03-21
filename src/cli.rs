#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        eprintln!("{}: {}", env!("CARGO_PKG_NAME"), format!($($arg)*));
    };
}

#[macro_export]
macro_rules! ask_string {
    ($($arg:tt)*) => {
        {
            use termion::input::TermRead;
            use std::io::Write;

            let stdin = std::io::stdin();
            let mut stdin = stdin.lock();
            let stdout = std::io::stdout();
            let mut stdout = stdout.lock();
            let stderr = std::io::stderr();
            let _stderr = stderr.lock();

            write!(stdout, "{} ", format!($($arg)*)).ok();
            stdout.flush().ok();
            match stdin.read_line().ok().flatten() {
                Some(line) => line,
                None => {
                    write!(stdout, "\n").ok();
                    std::process::exit(1)
                }
            }
        }
    };
}

#[macro_export]
macro_rules! ask_bool {
    ($($arg:tt)*) => {
        {
            match ask_string!($($arg)*).trim().to_lowercase().as_str() {
                "y" | "yes" => true,
                _ => false,
            }
        }
    };
}

#[macro_export]
macro_rules! ask_secret {
    ($($arg:tt)*) => {
        {
            use termion::input::TermRead;
            use std::io::Write;

            let stdin = std::io::stdin();
            let mut stdin = stdin.lock();
            let stdout = std::io::stdout();
            let mut stdout = stdout.lock();
            let stderr = std::io::stderr();
            let _stderr = stderr.lock();

            write!(stdout, "{} ", format!($($arg)*)).ok();
            stdout.flush().ok();
            let secret = stdin.read_passwd(&mut stdout)
                .ok()
                .flatten();

            write!(stdout, "\n").ok();
            secret.unwrap_or_else(|| std::process::exit(1))
        }
    };
}
