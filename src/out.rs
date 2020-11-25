#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        println!("{}: {}", env!("CARGO_PKG_NAME"), format!($($arg)*));
    }
}
