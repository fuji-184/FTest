use std::backtrace::Backtrace;
use std::fmt;

const RED: &str = "\x1b[41;1m";
const GREEN: &str = "\x1b[92m";
const YELLOW: &str = "\x1b[93m";
const BLUE: &str = "\x1b[94m";
const MAGENTA: &str = "\x1b[95m";
const CYAN: &str = "\x1b[96m";
const WHITE: &str = "\x1b[97m";
const BOLD: &str = "\x1b[1m";
const RESET: &str = "\x1b[0m";

#[cfg(any(test, feature = "trace"))]
pub type MyError = crate::TraceError;

#[cfg(not(any(test, feature = "trace")))]
pub type MyError = Box<dyn std::error::Error>;

pub type Res<T = ()> = std::result::Result<T, MyError>;

pub struct TraceError {
    pub inner: Box<dyn std::error::Error>,
    pub file: &'static str,
    pub line: u32,
    pub column: u32,
    pub backtrace: Backtrace,
    pub caller: bool,
    pub caller_thread: std::thread::ThreadId
}

impl fmt::Debug for TraceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.caller {
          writeln!(f, "{}{:?}{}\nOrigin: {}./{}:{}:{} thread id: {:?}{}", RED, self.inner, RESET, GREEN, self.file, self.line, self.column, self.caller_thread, RESET)?;
        } else {
          writeln!(f, "{}{:?}{}\nCaller: {}./{}:{}:{} thread id: {:?}{}", RED, self.inner, RESET, GREEN, self.file, self.line, self.column, self.caller_thread, RESET)?;
        }

        let bt = self.backtrace.to_string();
        let mut lines = bt.lines();

        let caller_file = self.file;
        let caller_line = self.line.to_string();

        while let Some(_func) = lines.next() {
            if let Some(loc) = lines.next() {
                let l = loc.trim();

                if self.caller {
                  
                if l.contains("src/")
                    && !l.contains("/rustc/")
                    && !l.contains("core/")
                    && !l.contains("std/")
                    && !l.contains("test/")
                    && !l.contains("FTest")
                {
                    if !(l.contains(caller_file) && l.contains(&caller_line)) {
                        if let Some(loc) = l.strip_prefix("at ") {
                          writeln!(f, "Caller: {}{}{}", GREEN, loc, RESET)?;
                        } else {
                          writeln!(f, "Caller: {}{}{}", GREEN, l, RESET)?;
                        }
                        break;
                    }
                }
                
                }
                
            }
        }

        Ok(())
    }
}

impl From<Box<dyn std::error::Error>> for TraceError {
    #[track_caller]
    fn from(err: Box<dyn std::error::Error>) -> Self {
        let loc = std::panic::Location::caller();
        Self {
            inner: err,
            file: loc.file(),
            line: loc.line(),
            column: loc.column(),
            backtrace: Backtrace::capture(),
            caller: true,
            caller_thread: std::thread::current().id()
        }
    }
}

impl From<&str> for TraceError {
    #[track_caller]
    fn from(err: &str) -> Self {
        let loc = std::panic::Location::caller();
        Self {
            inner: err.into(),
            file: loc.file(),
            line: loc.line(),
            column: loc.column(),
            backtrace: Backtrace::capture(),
            caller: true,
            caller_thread: std::thread::current().id()
        }
    }
}

impl From<anyhow::Error> for TraceError {
    #[track_caller]
    fn from(err: anyhow::Error) -> Self {
        let loc = std::panic::Location::caller();
        Self {
            inner: err.into(),
            file: loc.file(),
            line: loc.line(),
            column: loc.column(),
            backtrace: std::backtrace::Backtrace::capture(),
            caller: false,
            caller_thread: std::thread::current().id()
        }
    }
}

#[macro_export]
macro_rules! parse {
	($name:ident { $($body:tt)* } $($rest:tt)*) => {
		#[test]
		fn $name() -> Result<(), $crate::TraceError> {
        $($body)*
        Ok(())
		}
		$crate::parse!($($rest)*);
	};
	($item:item $($rest:tt)*) => {
		$item
		$crate::parse!($($rest)*);
	};
	() => {};
}

#[macro_export]
macro_rules! test {
	($mod_name:ident, { $($t:tt)* }) => {
		#[cfg(test)]
		mod $mod_name {
			#[allow(unused_imports)]
			use super::*;
			$crate::parse!($($t)*);
		}
	};
}

#[macro_export]
macro_rules! thiserror {
    ($err_type:ty) => {
        impl From<$err_type> for TraceError {
            #[track_caller]
            fn from(err: $err_type) -> Self {
                let loc = std::panic::Location::caller();
                Self {
                    inner: Box::new(err),
                    file: loc.file(),
                    line: loc.line(),
                    column: loc.column(),
                    backtrace: std::backtrace::Backtrace::capture(),
                    caller: false,
                    caller_thread: std::thread::current().id(),
                }
            }
        }
    };
}

test!(tes, {
	use crate::*;

	const A: i32 = 10;

	fn f() -> i32 {
		10
	}

	tes {
		let i = A + f();
		assert_eq!(i, 20);
	}
});


#[cfg(feature = "bench")]
#[macro_export]
macro_rules! parse_bench {
	($name:ident { $($setup:tt)* } -> { $($body:tt)* } $($rest:tt)*) => {
		#[bench]
		fn $name(b: &mut test::Bencher) {
			$($setup)*
			b.iter(|| {
				let _ = test::black_box({ $($body)* });
			});
		}
		$crate::parse_bench!($($rest)*);
	};
	($name:ident { $($body:tt)* } $($rest:tt)*) => {
		#[bench]
		fn $name(b: &mut test::Bencher) {
			b.iter(|| {
				let _ = test::black_box({ $($body)* });
			});
		}
		$crate::parse_bench!($($rest)*);
	};
	($item:item $($rest:tt)*) => {
		$item
		$crate::parse_bench!($($rest)*);
	};
	() => {};
}

#[cfg(feature = "bench")]
#[macro_export]
macro_rules! bench {
	($mod_name:ident, { $($t:tt)* }) => {
		#[cfg(test)]
		mod $mod_name {
			#[allow(unused_imports)]
			extern crate test;
			use super::*;
			$crate::parse_bench!($($t)*);
		}
	}
}

#[cfg(not(feature = "bench"))]
#[macro_export]
macro_rules! bench {
    ($($t:tt)*) => {};
}

#[cfg(not(feature = "bench"))]
bench!(benchmark, {
	use crate::*;

	const A: i32 = 10;

	fn f() -> i32 {
		10
	}

	bench {
		1 + 2 + A + f()
	}

	bench_with_setup {
		let i = 10;
	} -> {
		i + 2 + A + f()
	}

});
