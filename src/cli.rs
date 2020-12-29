macro_rules! info {
    ($($arg:tt)*) => {
        eprintln!("{}: {}", env!("CARGO_PKG_NAME"), format!($($arg)*));
    };
}

macro_rules! ask_string {
    ($($arg:tt)*) => {
        {
            use termion::input::TermRead;

            eprint!("{} ", format!($($arg)*));

            let stdin = std::io::stdin();
            let mut stdin = stdin.lock();

            match stdin.read_line().ok().flatten() {
                Some(line) => line,
                None => {
                    eprintln!();
                    "".into()
                }
            }
        }
    };
}

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

macro_rules! ask_secret {
    ($($arg:tt)*) => {
        {
            use termion::input::TermRead;

            eprint!("{} ", format!($($arg)*));

            let stdin = std::io::stdin();
            let mut stdin = stdin.lock();
            let stdout = std::io::stdout();
            let mut stdout = stdout.lock();

            let secret = stdin.read_passwd(&mut stdout)
                .ok()
                .flatten()
                .unwrap_or("".into());

            eprintln!();
            secret
        }
    };
}
