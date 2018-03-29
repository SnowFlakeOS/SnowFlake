#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::console::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => (print!("\n"));
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}

#[macro_export]
macro_rules! scanf {
	( $string:expr, $sep:expr, $($x:ty),+ ) => {{
		let mut iter = $string.split($sep);
		($(iter.next().and_then(|word| word.parse::<$x>().ok()),)*)
	}}
}