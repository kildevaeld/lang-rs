#[cfg(feature = "trace")]
macro_rules! trace {
    ($($arg:tt)+) => {
        std::println!($($arg)+)
    };
}

#[cfg(not(feature = "trace"))]
macro_rules! trace {
    ($($arg:tt)+) => {};
}
