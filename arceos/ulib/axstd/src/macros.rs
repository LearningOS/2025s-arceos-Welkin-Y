//! Standard library macros
/// Prints to the standard output.
///
/// Equivalent to the [`println!`] macro except that a newline is not printed at
/// the end of the message.
///
/// [`println!`]: crate::println
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        $crate::io::__print_impl(format_args!($($arg)*));
    }
}

/// Prints to the standard output, with a newline.
#[macro_export]
macro_rules! println {
    () => { $crate::print!("\n") };
    ($fmt:literal $(, $arg:tt)*) => {{
        if $fmt.starts_with("[WithColor]:") {
            $crate::io::__print_impl(format_args!(concat!("\x1b[31m", $fmt, "\x1b[0m\n") $(, $arg)*));
        } else {
            $crate::io::__print_impl(format_args!(concat!($fmt, "\n") $(, $arg)*));
        }
    }};
}
