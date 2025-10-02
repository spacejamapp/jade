//! This module is extracted from `jam-pvm-common` with fixes

pub use api::*;

/// Log a message with the `error` level. Regular formatting may be used.
#[macro_export]
macro_rules! error {
	(target=$target:expr,$($arg:tt)*) => {
		$crate::logging::log_target(0, $target, &$crate::prelude::format!($($arg)*));
	};
	($($arg:tt)*) => {
		$crate::logging::log(0, &$crate::prelude::format!($($arg)*));
	};
}

/// Log a message with the `warn` level. Regular formatting may be used.
#[macro_export]
macro_rules! warn {
	(target=$target:expr,$($arg:tt)*) => {
		$crate::logging::log_target(1, $target, &$crate::prelude::format!($($arg)*));
	};
	($($arg:tt)*) => {
		$crate::logging::log(1, &$crate::prelude::format!($($arg)*));
	};
}

/// Log a message with the `info` level. Regular formatting may be used.
#[macro_export]
macro_rules! info {
	(target=$target:expr,$($arg:tt)*) => {
		$crate::logging::log_target(2, $target, &$crate::prelude::format!($($arg)*));
	};
	($($arg:tt)*) => {
		$crate::logging::log(2, &$crate::prelude::format!($($arg)*));
	};
}

/// Log a message with the `debug` level. Regular formatting may be used.
#[macro_export]
macro_rules! debug {
	(target=$target:expr,$($arg:tt)*) => {
		$crate::logging::log_target(3, $target, &$crate::prelude::format!($($arg)*));
	};
	($($arg:tt)*) => {
		$crate::logging::log(3, &$crate::prelude::format!($($arg)*));
	};
}

/// Log a message with the `trace` level. Regular formatting may be used.
#[macro_export]
macro_rules! trace {
	(target=$target:expr,$($arg:tt)*) => {
		$crate::logging::log_target(4, $target, &$crate::prelude::format!($($arg)*));
	};
	($($arg:tt)*) => {
		$crate::logging::log(4, &$crate::prelude::format!($($arg)*));
	};
}

#[cfg(any(feature = "logging", doc))]
mod api {
    // CAUTION: Not public API. DO NOT USE.
    pub fn log_target(level: u64, target: &str, msg: &str) {
        let t = target.as_bytes();
        let m = msg.as_bytes();
        unsafe {
            crate::host::import::log(
                level,
                t.as_ptr(),
                t.len() as u64,
                m.as_ptr(),
                m.len() as u64,
            )
        }
    }

    // CAUTION: Not public API. DO NOT USE.
    pub fn log(level: u64, msg: &str) {
        let m = msg.as_bytes();
        unsafe {
            crate::host::import::log(level, core::ptr::null(), 0u64, m.as_ptr(), m.len() as u64)
        }
    }
}

#[cfg(not(any(feature = "logging", doc)))]
mod api {
    // CAUTION: Not public API. DO NOT USE.
    pub fn log_target(level: u64, target: &str, msg: &str) {
        let _ = (level, target, msg);
    }

    // CAUTION: Not public API. DO NOT USE.
    pub fn log(level: u64, msg: &str) {
        let _ = (level, msg);
    }
}
