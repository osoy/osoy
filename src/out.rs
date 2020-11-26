#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        eprintln!("{}: {}", env!("CARGO_PKG_NAME"), format!($($arg)*));
    }
}
