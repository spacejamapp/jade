use super::*;
use bounded_collections::Get;
use codec::{decode_vec_with_len, ConstEncodedLen, Error as DecodeError, Output};
use core::fmt;

/// Trait for vector types which have a bounded length allowing element-wise transformation.
pub trait BoundedMap<T, N: Get<u32>> {
	/// Element-wise transformation of items using the given function.
	///
	/// - `f`: The transformation function.
	///
	/// Returns a new instance with all elements transformed using the function `f`.
	fn map<U>(self, f: impl FnMut(T) -> U) -> BoundedVec<U, N>;
	/// Element-wise transformation of item-references using the given function.
	///
	/// - `f`: The transformation function.
	///
	/// Returns a new instance with all element-references transformed using the function `f`.
	fn map_ref<U>(&self, f: impl FnMut(&T) -> U) -> BoundedVec<U, N>;
	/// Fallible element-wise transformation of items using the given function.
	///
	/// - `f`: The transformation function.
	///
	/// Returns a new instance with all elements transformed using the function `f`, or
	/// `Err` if any invocation of `f` resulted in error.
	fn try_map<U, E>(self, f: impl FnMut(T) -> Result<U, E>) -> Result<BoundedVec<U, N>, E>;
	/// Fallible element-wise transformation of item-references using the given function.
	///
	/// - `f`: The transformation function.
	///
	/// Returns a new instance with all element-references transformed using the function `f`, or
	/// `Err` if any invocation of `f` resulted in error.
	fn try_map_ref<U, E>(&self, f: impl FnMut(&T) -> Result<U, E>) -> Result<BoundedVec<U, N>, E>;
}
impl<T, N: Get<u32>> BoundedMap<T, N> for BoundedVec<T, N> {
	fn map<U>(self, f: impl FnMut(T) -> U) -> BoundedVec<U, N> {
		BoundedVec::truncate_from(self.into_iter().map(f).collect::<Vec<_>>())
	}
	fn map_ref<U>(&self, f: impl FnMut(&T) -> U) -> BoundedVec<U, N> {
		BoundedVec::truncate_from(self.iter().map(f).collect::<Vec<_>>())
	}
	fn try_map<U, E>(self, f: impl FnMut(T) -> Result<U, E>) -> Result<BoundedVec<U, N>, E> {
		self.into_iter()
			.map(f)
			.collect::<Result<Vec<_>, E>>()
			.map(BoundedVec::truncate_from)
	}
	fn try_map_ref<U, E>(&self, f: impl FnMut(&T) -> Result<U, E>) -> Result<BoundedVec<U, N>, E> {
		self.iter().map(f).collect::<Result<Vec<_>, E>>().map(BoundedVec::truncate_from)
	}
}

/// Vector type with a fixed length.
///
/// Can be used similarly to an array but which stores its elements on the heap.
pub struct FixedVec<T, N: Get<u32>>(BoundedVec<T, N>);
impl<T, N: Get<u32>> FixedVec<T, N> {
	pub fn new(t: T) -> Self
	where
		T: Clone,
	{
		Self::from_fn(|_| t.clone())
	}
	pub fn padded(t: &[T]) -> Self
	where
		T: Default + Clone,
	{
		Self::from_fn(|i| t.get(i).cloned().unwrap_or_default())
	}
	pub fn from_fn(f: impl FnMut(usize) -> T) -> Self {
		Self(BoundedVec::truncate_from((0..N::get() as usize).map(f).collect::<Vec<T>>()))
	}
	pub fn get(&self, i: usize) -> Option<&T> {
		if i < N::get() as usize {
			Some(&self.0[i])
		} else {
			None
		}
	}
	pub fn get_mut(&mut self, i: usize) -> Option<&mut T> {
		if i < N::get() as usize {
			Some(&mut self.0[i])
		} else {
			None
		}
	}
	pub fn len(&self) -> usize {
		N::get() as usize
	}
	pub fn is_empty(&self) -> bool {
		N::get() == 0
	}
	pub fn iter(&self) -> core::slice::Iter<T> {
		self.0.iter()
	}
	pub fn iter_mut(&mut self) -> core::slice::IterMut<T> {
		self.0.iter_mut()
	}
	pub fn map<U>(self, f: impl FnMut(T) -> U) -> FixedVec<U, N> {
		FixedVec(BoundedVec::truncate_from(self.0.into_iter().map(f).collect::<Vec<_>>()))
	}
	pub fn map_ref<U>(&self, f: impl FnMut(&T) -> U) -> FixedVec<U, N> {
		FixedVec(BoundedVec::truncate_from(self.0.iter().map(f).collect::<Vec<_>>()))
	}
	pub fn to_bounded(self) -> BoundedVec<T, N> {
		self.0
	}
	pub fn to_vec(self) -> Vec<T> {
		self.0.into()
	}
	pub fn into_vec(self) -> Vec<T> {
		self.0.into()
	}
	pub fn truncate_into_vec(self, len: usize) -> Vec<T> {
		let mut v = self.into_vec();
		v.truncate(len);
		v
	}
	pub fn slide(&mut self, index: usize, insert_at: usize) {
		self.0.slide(index, insert_at);
	}
	pub fn force_insert_keep_left(&mut self, index: usize, element: T) -> Result<Option<T>, T> {
		self.0.force_insert_keep_left(index, element)
	}
	pub fn force_insert_keep_right(&mut self, index: usize, element: T) -> Result<Option<T>, T> {
		self.0.force_insert_keep_right(index, element)
	}
	pub fn force_push(&mut self, element: T) {
		self.0.force_push(element);
	}
	pub fn swap(&mut self, a: usize, b: usize) {
		self.0.as_mut().swap(a, b);
	}
	pub fn as_slice(&self) -> &[T] {
		self.0.as_slice()
	}
	pub fn as_mut_slice(&mut self) -> &mut [T] {
		self.0.as_mut()
	}
	pub fn as_ptr(&self) -> *const T {
		self.0.as_ptr()
	}
	pub fn as_mut_ptr(&mut self) -> *mut T {
		self.0.as_mut().as_mut_ptr()
	}
}

impl<T: Encode, N: Get<u32>> Encode for FixedVec<T, N> {
	fn size_hint(&self) -> usize {
		self.0.size_hint() - 1
	}
	fn encode_to<D: Output + ?Sized>(&self, dest: &mut D) {
		for t in self.0.iter() {
			t.encode_to(dest);
		}
	}
}

impl<T: fmt::Debug, N: Get<u32>> fmt::Debug for FixedVec<T, N> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self.0.len() {
			0 => write!(f, "[]"),
			_ => {
				write!(f, "[{:?}", self.0[0])?;
				for i in 1..self.len() {
					write!(f, ", {:?}", self.0[i])?;
				}
				write!(f, "]")
			},
		}
	}
}

impl<T: ConstEncodedLen, N: Get<u32>> codec::ConstEncodedLen for FixedVec<T, N> {}

impl<N: Get<u32>> fmt::Display for FixedVec<u8, N> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let suffix = self.iter().rev().take_while(|&&x| x == 0).count();
		let body = &self.0[..self.len() - suffix];
		if !body.is_empty() {
			if body.iter().all(|&x| (32..=127).contains(&x)) {
				f.write_fmt(format_args!(
					"\"{}\"",
					core::str::from_utf8(body).expect("ASCII; qed")
				))?;
			}
			if body.len() > 32 {
				f.write_fmt(format_args!(
					"0x{}..{}",
					hex::hex(&body[..8]),
					hex::hex(&body[body.len() - 4..])
				))?;
			} else {
				f.write_fmt(format_args!("0x{}", hex::hex(body)))?;
			}
		}
		if suffix != 0 {
			f.write_fmt(format_args!("{}{suffix}*0", if !body.is_empty() { "+" } else { "" }))?;
		}
		Ok(())
	}
}

impl<T: Decode, N: Get<u32>> Decode for FixedVec<T, N> {
	fn decode<I: codec::Input>(input: &mut I) -> Result<Self, DecodeError> {
		Ok(Self(BoundedVec::truncate_from(decode_vec_with_len(input, N::get() as usize)?)))
	}
}

impl<T: MaxEncodedLen, N: Get<u32>> MaxEncodedLen for FixedVec<T, N> {
	fn max_encoded_len() -> usize {
		T::max_encoded_len() * N::get() as usize
	}
}

impl<T: Clone, N: Get<u32>> Clone for FixedVec<T, N> {
	fn clone(&self) -> Self {
		Self(self.0.clone())
	}
}
impl<T: Eq, N: Get<u32>> Eq for FixedVec<T, N> where T: Eq {}
impl<T: PartialEq, N: Get<u32>> PartialEq for FixedVec<T, N> {
	fn eq(&self, other: &Self) -> bool {
		self.0[..] == other.0[..]
	}
}
impl<T: Default, N: Get<u32>> Default for FixedVec<T, N> {
	fn default() -> Self {
		Self::from_fn(|_| T::default())
	}
}
impl<T, N: Get<u32>> AsRef<[T]> for FixedVec<T, N> {
	fn as_ref(&self) -> &[T] {
		&self.0[..]
	}
}
impl<T, N: Get<u32>> AsMut<[T]> for FixedVec<T, N> {
	fn as_mut(&mut self) -> &mut [T] {
		&mut self.0[..]
	}
}
impl<T, N: Get<u32>> core::ops::Index<usize> for FixedVec<T, N> {
	type Output = T;
	fn index(&self, i: usize) -> &T {
		&self.0[i]
	}
}
impl<T, N: Get<u32>> core::ops::IndexMut<usize> for FixedVec<T, N> {
	fn index_mut(&mut self, i: usize) -> &mut T {
		&mut self.0[i]
	}
}
impl<T, N: Get<u32>> core::ops::Index<core::ops::Range<usize>> for FixedVec<T, N> {
	type Output = [T];

	fn index(&self, index: core::ops::Range<usize>) -> &Self::Output {
		&self.0[index]
	}
}
impl<T, N: Get<u32>> core::ops::IndexMut<core::ops::Range<usize>> for FixedVec<T, N> {
	fn index_mut(&mut self, index: core::ops::Range<usize>) -> &mut Self::Output {
		&mut self.0[index]
	}
}
impl<T, N: Get<u32>> core::ops::Index<core::ops::RangeFull> for FixedVec<T, N> {
	type Output = [T];

	fn index(&self, index: core::ops::RangeFull) -> &Self::Output {
		&self.0[index]
	}
}
impl<T, N: Get<u32>> core::ops::IndexMut<core::ops::RangeFull> for FixedVec<T, N> {
	fn index_mut(&mut self, index: core::ops::RangeFull) -> &mut Self::Output {
		&mut self.0[index]
	}
}
impl<T, N: Get<u32>> From<FixedVec<T, N>> for Vec<T> {
	fn from(s: FixedVec<T, N>) -> Vec<T> {
		s.to_vec()
	}
}
impl<T, N: Get<u32>> TryFrom<Vec<T>> for FixedVec<T, N> {
	type Error = ();
	fn try_from(v: Vec<T>) -> Result<Self, ()> {
		if v.len() != N::get() as usize {
			panic!("Invalid length");
		}
		Ok(Self(BoundedVec::truncate_from(v)))
	}
}
impl<T, N: Get<u32>> From<FixedVec<T, N>> for BoundedVec<T, N> {
	fn from(s: FixedVec<T, N>) -> BoundedVec<T, N> {
		s.0
	}
}
impl<T, N: Get<u32>> TryFrom<BoundedVec<T, N>> for FixedVec<T, N> {
	type Error = ();
	fn try_from(v: BoundedVec<T, N>) -> Result<Self, ()> {
		if v.len() != N::get() as usize {
			return Err(())
		}
		Ok(Self(v))
	}
}
impl<'a, T: Clone, N: Get<u32>> TryFrom<&'a [T]> for FixedVec<T, N> {
	type Error = ();
	fn try_from(v: &'a [T]) -> Result<Self, ()> {
		if v.len() != N::get() as usize {
			return Err(())
		}
		Ok(Self(BoundedVec::truncate_from(v.to_vec())))
	}
}
#[cfg(feature = "serde")]
impl<'de, T, S: Get<u32>> serde::Deserialize<'de> for FixedVec<T, S>
where
	T: serde::Deserialize<'de>,
{
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		use serde::de::Error;
		let result = BoundedVec::deserialize(deserializer)?;
		if result.len() != S::get() as usize {
			return Err(D::Error::custom("Invalid FixedVec length"))
		}
		Ok(Self(result))
	}
}

#[cfg(feature = "serde")]
impl<T, S: Get<u32>> serde::Serialize for FixedVec<T, S>
where
	T: serde::Serialize,
{
	fn serialize<SR>(&self, serializer: SR) -> Result<SR::Ok, SR::Error>
	where
		SR: serde::Serializer,
	{
		self.0.serialize(serializer)
	}
}
