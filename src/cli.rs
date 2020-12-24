macro_rules! info {
    ($($arg:tt)*) => {
        eprintln!("{}: {}", env!("CARGO_PKG_NAME"), format!($($arg)*));
    };
}

macro_rules! ask {
    ($($arg:tt)*) => {
        {
            eprint!("{}: {} ", env!("CARGO_PKG_NAME"), format!($($arg)*));
            let mut input = String::new();
            match std::io::stdin().read_line(&mut input) {
                Ok(_) => match input.trim().to_lowercase().as_str() {
                    "y" | "yes" => true,
                    _ => false
                }
                Err(err) => {
                    info!("could not read from stdin: {}", err);
                    false
                }
            }
        }
    };
}
