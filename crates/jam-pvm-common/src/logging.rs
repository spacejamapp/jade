#[cfg(any(feature = "logging", doc))]
pub mod stuff {
	/// Log a message with the `error` level. Regular formatting may be used.
	#[macro_export]
	macro_rules! error {
		(target=$target:expr,$($arg:tt)*) => {
			$crate::logging::stuff::log_target(0, $target, &alloc::format!($($arg)*));
		};
		($($arg:tt)*) => {
			$crate::logging::stuff::log(0, &alloc::format!($($arg)*));
		};
	}

	/// Log a message with the `warn` level. Regular formatting may be used.
	#[macro_export]
	macro_rules! warn {
		(target=$target:expr,$($arg:tt)*) => {
			$crate::logging::stuff::log_target(1, $target, &alloc::format!($($arg)*));
		};
		($($arg:tt)*) => {
			$crate::logging::stuff::log(1, &alloc::format!($($arg)*));
		};
	}

	/// Log a message with the `info` level. Regular formatting may be used.
	#[macro_export]
	macro_rules! info {
		(target=$target:expr,$($arg:tt)*) => {
			$crate::logging::stuff::log_target(2, $target, &alloc::format!($($arg)*));
		};
		($($arg:tt)*) => {
			$crate::logging::stuff::log(2, &alloc::format!($($arg)*));
		};
	}

	/// Log a message with the `debug` level. Regular formatting may be used.
	#[macro_export]
	macro_rules! debug {
		(target=$target:expr,$($arg:tt)*) => {
			$crate::logging::stuff::log_target(3, $target, &alloc::format!($($arg)*));
		};
		($($arg:tt)*) => {
			$crate::logging::stuff::log(3, &alloc::format!($($arg)*));
		};
	}

	/// Log a message with the `trace` level. Regular formatting may be used.
	#[macro_export]
	macro_rules! trace {
		(target=$target:expr,$($arg:tt)*) => {
			$crate::logging::stuff::log_target(4, $target, &alloc::format!($($arg)*));
		};
		($($arg:tt)*) => {
			$crate::logging::stuff::log(4, &alloc::format!($($arg)*));
		};
	}

	// CAUTION: Not public API. DO NOT USE.
	pub fn log_target(level: u64, target: &str, msg: &str) {
		let t = target.as_bytes();
		let m = msg.as_bytes();
		unsafe {
			crate::imports::log(level, t.as_ptr(), t.len() as u64, m.as_ptr(), m.len() as u64)
		}
	}

	// CAUTION: Not public API. DO NOT USE.
	pub fn log(level: u64, msg: &str) {
		let m = msg.as_bytes();
		unsafe { crate::imports::log(level, core::ptr::null(), 0u64, m.as_ptr(), m.len() as u64) }
	}
}

#[cfg(not(any(feature = "logging", doc)))]
pub mod stuff {
	/// Log a message with the `error` level. Regular formatting may be used.mod stuff {
	#[macro_export]
	macro_rules! error {
		($($arg:tt)*) => {
			{ let _ = ($( $arg, )*); }
		};
	}

	/// Log a message with the `warn` level. Regular formatting may be used.
	#[macro_export]
	macro_rules! warn {
		($($arg:tt)*) => {
			{ let _ = ($( $arg, )*); }
		};
	}

	/// Log a message with the `info` level. Regular formatting may be used.
	#[macro_export]
	macro_rules! info {
		($($arg:tt)*) => {
			{ let _ = ($( $arg, )*); }
		};
	}

	/// Log a message with the `debug` level. Regular formatting may be used.
	#[macro_export]
	macro_rules! debug {
		($($arg:tt)*) => {
			{ let _ = ($( $arg, )*); }
		};
	}

	/// Log a message with the `trace` level. Regular formatting may be used.
	#[macro_export]
	macro_rules! trace {
		($($arg:tt)*) => {
			{ let _ = ($( $arg, )*); }
		};
	}
}
