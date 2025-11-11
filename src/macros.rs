#[macro_export]
macro_rules! LOG {
    ($message:expr) => {{
        if cfg!(any(debug_assertions, test)) {
            println!("LOG: {}", $message);
        } else {
            // no-op in release
        }
    }};
}
