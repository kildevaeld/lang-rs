#[cfg(feature = "trace")]
macro_rules! trace {
    ($($arg:tt)+) => {
        std::eprintln!($($arg)+)
    };
}

#[cfg(not(feature = "trace"))]
macro_rules! trace {
    ($($arg:tt)+) => {};
}
