#[cfg(not(feature = "std"))]
extern crate alloc;
use core::num::ParseIntError;

#[cfg(not(feature = "std"))]
use alloc::{string::String, vec::Vec};

pub fn hex(d: &[u8]) -> HexDisplay {
	HexDisplay(d)
}

pub fn from_hex(s: &str) -> Result<Vec<u8>, ParseIntError> {
	(0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16)).collect()
}

pub fn to_hex(d: &[u8]) -> String {
	let mut s = String::with_capacity(d.len() * 2);
	for byte in d {
		s.push(char::from_digit((*byte >> 4) as u32, 16).expect("Digit is always valid"));
		s.push(char::from_digit((*byte & 0xf) as u32, 16).expect("Digit is always valid"));
	}
	s
}

#[derive(Eq, PartialEq)]
pub struct HexDisplay<'a>(&'a [u8]);

impl<'a> HexDisplay<'a> {
	/// Create new instance that will display `d` as a hex string when displayed.
	pub fn from<R: AsBytesRef>(d: &'a R) -> Self {
		HexDisplay(d.as_bytes_ref())
	}
}

impl core::fmt::Display for HexDisplay<'_> {
	fn fmt(&self, f: &mut core::fmt::Formatter) -> Result<(), core::fmt::Error> {
		if self.0.len() < 1027 {
			for byte in self.0 {
				f.write_fmt(format_args!("{:02x}", byte))?;
			}
		} else {
			for byte in &self.0[0..512] {
				f.write_fmt(format_args!("{:02x}", byte))?;
			}
			f.write_str("...")?;
			for byte in &self.0[self.0.len() - 512..] {
				f.write_fmt(format_args!("{:02x}", byte))?;
			}
		}
		Ok(())
	}
}

impl core::fmt::Debug for HexDisplay<'_> {
	fn fmt(&self, f: &mut core::fmt::Formatter) -> Result<(), core::fmt::Error> {
		if self.0.iter().all(|b| b.is_ascii_graphic() || b.is_ascii_whitespace()) {
			let s = core::str::from_utf8(self.0).map_err(|_| core::fmt::Error)?;
			return core::fmt::Debug::fmt(&s, f);
		}
		for byte in self.0 {
			f.write_fmt(format_args!("{:02x}", byte))?;
		}
		Ok(())
	}
}

/// Simple trait to transform various types to `&[u8]`
pub trait AsBytesRef {
	/// Transform `self` into `&[u8]`.
	fn as_bytes_ref(&self) -> &[u8];
}

impl AsBytesRef for &[u8] {
	fn as_bytes_ref(&self) -> &[u8] {
		self
	}
}

impl AsBytesRef for [u8] {
	fn as_bytes_ref(&self) -> &[u8] {
		self
	}
}

impl AsBytesRef for Vec<u8> {
	fn as_bytes_ref(&self) -> &[u8] {
		self
	}
}

macro_rules! impl_array {
	( $( $t:ty ),* ) => { $(
		impl AsBytesRef for $t {
			fn as_bytes_ref(&self) -> &[u8] { &self[..] }
		}
	)* }
}

impl_array!([u8; 32]);
