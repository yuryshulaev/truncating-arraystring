pub use arrayvec::{ArrayString, CapacityError};
use std::fmt;

#[derive(Debug)]
pub struct TruncatingArrayString<const CAP: usize>(pub ArrayString<CAP>);

impl<const CAP: usize> TruncatingArrayString<CAP> {
	pub fn new() -> Self {
		Self(ArrayString::<CAP>::new())
	}

	pub fn try_push_str_truncate<'a>(&mut self, s: &'a str) -> Result<(), CapacityError<&'a str>> {
		let remaining_capacity = self.0.capacity() - self.0.len();

		if s.len() < remaining_capacity {
			self.0.push_str(s);
			Ok(())
		} else {
			let (fits, rest) = s.split_at(floor_char_boundary(s, remaining_capacity));
			self.0.push_str(fits);
			Err(CapacityError::new(rest))
		}
	}
}

impl<const CAP: usize> fmt::Display for TruncatingArrayString<CAP> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.0.fmt(f)
	}
}

impl<const CAP: usize> fmt::Write for TruncatingArrayString<CAP> {
	fn write_char(&mut self, c: char) -> fmt::Result {
		self.0.try_push(c).map_err(|_| fmt::Error)
	}

	fn write_str(&mut self, s: &str) -> fmt::Result {
		self.try_push_str_truncate(s).map_err(|_| fmt::Error)
	}
}

// https://doc.rust-lang.org/std/primitive.str.html#method.floor_char_boundary
fn floor_char_boundary(s: &str, index: usize) -> usize {
	if index >= s.len() {
		s.len()
	} else {
		let lower_bound = index.saturating_sub(3);
		let new_index = s.as_bytes()[lower_bound..=index]
			.iter()
			.rposition(|b| is_utf8_char_boundary(*b));

		// SAFETY: we know that the character boundary will be within four bytes
		unsafe { lower_bound + new_index.unwrap_unchecked() }
	}
}

// https://doc.rust-lang.org/beta/src/core/num/mod.rs.html#883
const fn is_utf8_char_boundary(b: u8) -> bool {
	// This is bit magic equivalent to: b < 128 || b >= 192
	(b as i8) >= -0x40
}

#[cfg(test)]
mod tests {
	use std::fmt::Write;
	use super::*;

	#[test]
	fn it_truncates() {
		let mut buf = TruncatingArrayString::<5>::new();
		assert_eq!(write!(buf, "{}", "12"), Ok(()));
		assert_eq!(write!(buf, "{}", "3456789"), Err(std::fmt::Error));
		assert_eq!(&buf.0[..], "12345");
	}

	#[test]
	fn it_truncates_at_char_boundary() {
		let mut buf = TruncatingArrayString::<5>::new();
		assert_eq!(write!(buf, "{}", "α"), Ok(()));
		assert_eq!(write!(buf, "{}", "βγ"), Err(std::fmt::Error));
		assert_eq!(&buf.0[..], "αβ");
	}
}
