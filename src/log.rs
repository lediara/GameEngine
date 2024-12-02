#[macro_export]
macro_rules! Info {
    ($value:expr) => {
        println!("[Info]:    {}", $value);
    }
}

#[macro_export]
macro_rules! Warn {
    ($value:expr) => {
        println!("[Warn]:    {}", $value);
    }
}

#[macro_export]
macro_rules! Error {
    ($value:expr) => {
        println!("[Error]:    {}", $value);
    }
}

pub use Info;
pub use Warn;
pub use Error;
