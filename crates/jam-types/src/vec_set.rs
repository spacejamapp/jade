use super::*;
use alloc::collections::BTreeSet;
use codec::Error as DecodeError;
use core::fmt;

/// A data-structure providing the storage of non-duplicated items and the querying of their
/// inclusion.
pub trait SetLike<T> {
	/// Insert item into the set.
	///
	/// Inserts some item `t` into the set.
	///
	/// Returns `true` iff the item was not already in the set.
	fn insert(&mut self, t: T) -> bool;
	/// Insert multiple items from an iterator.
	fn extend(&mut self, iter: impl Iterator<Item = T>);
	/// Check if a value is in the set.
	///
	/// Returns `true` iff the set contains a value equal to `t`.
	fn contains(&self, t: &T) -> bool;
	/// Remove an item from the set.
	///
	/// Removes the item equal to `t` from the set.
	///
	/// Returns `true` iff the item was previously in the set.
	fn remove(&mut self, t: &T) -> bool;
}

#[cfg(feature = "std")]
impl<T: Eq + std::hash::Hash> SetLike<T> for std::collections::HashSet<T> {
	fn insert(&mut self, t: T) -> bool {
		std::collections::HashSet::<T>::insert(self, t)
	}
	fn extend(&mut self, iter: impl Iterator<Item = T>) {
		<std::collections::HashSet<T> as Extend<T>>::extend(self, iter)
	}
	fn contains(&self, t: &T) -> bool {
		std::collections::HashSet::<T>::contains(self, t)
	}
	fn remove(&mut self, t: &T) -> bool {
		std::collections::HashSet::<T>::remove(self, t)
	}
}

impl<T: Eq + PartialEq + Ord + PartialOrd> SetLike<T> for BTreeSet<T> {
	fn insert(&mut self, t: T) -> bool {
		BTreeSet::<T>::insert(self, t)
	}
	fn extend(&mut self, iter: impl Iterator<Item = T>) {
		<BTreeSet<T> as Extend<T>>::extend(self, iter)
	}
	fn contains(&self, t: &T) -> bool {
		BTreeSet::<T>::contains(self, t)
	}
	fn remove(&mut self, t: &T) -> bool {
		BTreeSet::<T>::remove(self, t)
	}
}

/// An set of items stored in order in a [Vec].
///
/// This is always efficient for small sizes of sets, and efficient for large sizes when
/// insertion is not needed (i.e. items are placed into the set in bulk).
#[derive(Clone, Encode, Eq, PartialEq, Ord, PartialOrd)]
pub struct VecSet<T>(Vec<T>);
impl<T: Eq + PartialEq + Ord + PartialOrd> VecSet<T> {
	/// Create a new, empty instance.
	pub fn new() -> Self {
		Self(Vec::new())
	}
	/// Create a new instance from a sorted [Vec].
	///
	/// Returns [Ok] with an instance containing the same items as `v`, or [Err] if `v` is unsorted.
	pub fn from_sorted(v: Vec<T>) -> Result<Self, ()> {
		let Some(first) = v.first() else { return Ok(Self::new()) };
		v.iter()
			.skip(1)
			.try_fold(first, |a, e| if a < e { Some(e) } else { None })
			.ok_or(())?;
		Ok(Self(v))
	}
	/// Return the number of items this set contains.
	pub fn len(&self) -> usize {
		self.0.len()
	}
	/// Return `true` if this set is empty.
	pub fn is_empty(&self) -> bool {
		self.0.is_empty()
	}
	/// Compares the items in two sets.
	///
	/// Returns `true` iff every item in the set is not found in `other`.
	pub fn is_disjoint(&self, other: &Self) -> bool {
		is_disjoint(&self.0, &other.0)
	}
	/// Return an iterator over the items in this set in order.
	pub fn iter(&self) -> core::slice::Iter<T> {
		self.0.iter()
	}
	/// Consume this set and return a [Vec] of transformed items.
	///
	/// Returns the [Vec] of the resultant values of applying `f` to each item in the set in
	/// order.
	pub fn map<U>(self, f: impl FnMut(T) -> U) -> Vec<U> {
		self.0.into_iter().map(f).collect::<Vec<_>>()
	}
	/// Transform all items by reference and return a [Vec] with the results.
	///
	/// Returns the [Vec] of the resultant values of applying `f` to each item by reference in the
	/// set in order.
	pub fn map_ref<U>(&self, f: impl FnMut(&T) -> U) -> Vec<U> {
		self.0.iter().map(f).collect::<Vec<_>>()
	}
	/// Return a [Vec] of sorted items by cloning this set.
	pub fn to_vec(&self) -> Vec<T>
	where
		T: Clone,
	{
		self.0.clone()
	}
	/// Return a [Vec] of sorted items by consuming this set.
	pub fn into_vec(self) -> Vec<T> {
		self.0
	}
	/// Insert item into the set.
	///
	/// Inserts some item `t` into the set.
	///
	/// Returns `true` iff the item was not already in the set.
	pub fn insert(&mut self, t: T) -> bool {
		match self.0.binary_search(&t) {
			Ok(_) => false,
			Err(i) => {
				self.0.insert(i, t);
				true
			},
		}
	}
	/// Insert multiple items from an iterator.
	pub fn extend(&mut self, iter: impl IntoIterator<Item = T>) {
		self.0.extend(iter);
		self.0.sort_unstable();
		self.0.dedup();
	}
	/// Check if a value is in the set.
	///
	/// Returns `true` iff the set contains a value equal to `t`.
	pub fn contains(&self, t: &T) -> bool {
		self.0.binary_search(t).is_ok()
	}
	/// Remove an item from the set.
	///
	/// Removes the item equal to `t` from the set.
	///
	/// Returns [Some] with the removed item if it was previously in the set.
	pub fn remove(&mut self, t: &T) -> Option<T> {
		match self.0.binary_search(t) {
			Ok(i) => Some(self.0.remove(i)),
			Err(_) => None,
		}
	}
	/// Filter items from the set.
	///
	/// Removes all items from the set for which `f` returns `false`.
	pub fn retain(&mut self, f: impl FnMut(&T) -> bool) {
		self.0.retain(f);
	}
}

impl<T: Eq + PartialEq + Ord + PartialOrd> SetLike<T> for VecSet<T> {
	fn insert(&mut self, t: T) -> bool {
		VecSet::<T>::insert(self, t)
	}
	fn extend(&mut self, iter: impl Iterator<Item = T>) {
		VecSet::<T>::extend(self, iter)
	}
	fn contains(&self, t: &T) -> bool {
		VecSet::<T>::contains(self, t)
	}
	fn remove(&mut self, t: &T) -> bool {
		VecSet::<T>::remove(self, t).is_some()
	}
}

impl<T: fmt::Debug> fmt::Debug for VecSet<T> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self.0.len() {
			0 => write!(f, "[]"),
			_ => {
				write!(f, "[{:?}", self.0[0])?;
				for i in 1..self.0.len() {
					write!(f, ", {:?}", self.0[i])?;
				}
				write!(f, "]")
			},
		}
	}
}

impl<T: fmt::Display> fmt::Display for VecSet<T> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self.0.len() {
			0 => write!(f, "[]"),
			_ => {
				write!(f, "[{}", self.0[0])?;
				for i in 1..self.0.len() {
					write!(f, ", {}", self.0[i])?;
				}
				write!(f, "]")
			},
		}
	}
}

impl<T: Decode + Eq + PartialEq + Ord + PartialOrd> Decode for VecSet<T> {
	fn decode<I: codec::Input>(input: &mut I) -> Result<Self, DecodeError> {
		Ok(Self::from_sorted(Vec::<T>::decode(input)?).map_err(|()| "set out-of-order")?)
	}
}

impl<T: Default + Eq + PartialEq + Ord + PartialOrd> Default for VecSet<T> {
	fn default() -> Self {
		Self::new()
	}
}
impl<T> AsRef<[T]> for VecSet<T> {
	fn as_ref(&self) -> &[T] {
		&self.0[..]
	}
}
impl<T> AsMut<[T]> for VecSet<T> {
	fn as_mut(&mut self) -> &mut [T] {
		&mut self.0[..]
	}
}

impl<T> From<VecSet<T>> for Vec<T> {
	fn from(s: VecSet<T>) -> Vec<T> {
		s.0
	}
}
impl<T: Eq + PartialEq + Ord + PartialOrd> From<Vec<T>> for VecSet<T> {
	fn from(mut v: Vec<T>) -> Self {
		v.sort_unstable();
		Self(v)
	}
}
impl<T: Eq + PartialEq + Ord + PartialOrd> FromIterator<T> for VecSet<T> {
	fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
		Vec::<T>::from_iter(iter).into()
	}
}
impl<T: Eq + PartialEq + Ord + PartialOrd> Extend<T> for VecSet<T> {
	fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
		VecSet::<T>::extend(self, iter)
	}
}
impl<T: Eq + PartialEq + Ord + PartialOrd> IntoIterator for VecSet<T> {
	type Item = T;
	type IntoIter = <Vec<T> as IntoIterator>::IntoIter;

	fn into_iter(self) -> Self::IntoIter {
		self.0.into_iter()
	}
}

pub(crate) fn is_disjoint<X: Ord + PartialOrd>(
	a: impl IntoIterator<Item = X>,
	b: impl IntoIterator<Item = X>,
) -> bool {
	let mut ordered_a = a.into_iter();
	let mut ordered_b = b.into_iter();
	let mut a_next = ordered_a.next();
	let mut b_next = ordered_b.next();
	while let (Some(a), Some(b)) = (a_next.as_ref(), b_next.as_ref()) {
		if a == b {
			return false
		}
		if a < b {
			a_next = ordered_a.next();
		} else {
			b_next = ordered_b.next();
		}
	}
	true
}
