#[macro_export]
macro_rules! parse {
	($name:ident { $($body:tt)* } $($rest:tt)*) => {
		#[test]
		fn $name() {
			$($body)*
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
