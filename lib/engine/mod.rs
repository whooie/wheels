pub mod wheel;
pub mod hero;
pub mod player;
pub mod game;

/// Call `print!` and automatically flush.
#[macro_export]
macro_rules! print_flush {
    ( ) => {
        print!();
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
    };
    ( $($arg:tt)* ) => {
        print!($($arg)*);
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
    };
}

/// Call `println!` and automatically flush.
#[macro_export]
macro_rules! println_flush {
    ( ) => {
        println!();
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
    };
    ( $($arg:tt)* ) => {
        println!($($arg)*);
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
    };
}

