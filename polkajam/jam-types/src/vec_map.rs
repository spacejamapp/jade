use super::*;
use codec::Error as DecodeError;
use core::fmt;

/// A data-structure providing the storage and lookup of key/value pairs with exclusive keys.
pub trait MapLike<K, V> {
	/// Insert pair into the mapping.
	///
	/// Inserts some pair (`k`, `v`) into the mapping, replacing the existing pair whose key is `k`,
	/// if any.
	///
	/// Returns [Some] value which was replaced, if any.
	fn insert(&mut self, k: K, v: V) -> Option<V>;
	/// Insert multiple pairs from an iterator.
	///
	/// Replaces any existing pairs which share a key with an element of the iterator.
	fn extend(&mut self, iter: impl Iterator<Item = (K, V)>);
	/// Check if a key is in the mapping.
	///
	/// Returns `true` iff the mapping contains a pair whose key equals `k`.
	fn contains_key(&self, k: &K) -> bool;
	/// Remove an item from the mapping by key.
	///
	/// Removes the item with key equal to `k` from the mapping.
	///
	/// Returns the value of the item removed, if any.
	fn remove(&mut self, k: &K) -> Option<V>;
	/// Get the value of the pair with a particular key.
	///
	/// Returns a reference the value of the item in the mapping whose key equals `k`.
	fn get(&self, k: &K) -> Option<&V>;
	/// Get a mutable reference to the value of the pair with a particular key.
	///
	/// Returns a mutable reference the value of the item in the mapping whose key equals `k`.
	fn get_mut(&mut self, k: &K) -> Option<&mut V>;
	/// Get an iterator to all keys in the mapping.
	///
	/// No order is guaranteed.
	fn keys<'a>(&'a self) -> impl Iterator<Item = &'a K>
	where
		K: 'a;
}

#[cfg(feature = "std")]
impl<K: Eq + std::hash::Hash, V> MapLike<K, V> for std::collections::HashMap<K, V> {
	fn insert(&mut self, k: K, v: V) -> Option<V> {
		std::collections::HashMap::<K, V>::insert(self, k, v)
	}
	fn extend(&mut self, iter: impl Iterator<Item = (K, V)>) {
		<std::collections::HashMap<K, V> as Extend<(K, V)>>::extend(self, iter)
	}
	fn contains_key(&self, k: &K) -> bool {
		std::collections::HashMap::<K, V>::contains_key(self, k)
	}
	fn remove(&mut self, k: &K) -> Option<V> {
		std::collections::HashMap::<K, V>::remove(self, k)
	}
	fn get(&self, k: &K) -> Option<&V> {
		std::collections::HashMap::<K, V>::get(self, k)
	}
	fn get_mut(&mut self, k: &K) -> Option<&mut V> {
		std::collections::HashMap::<K, V>::get_mut(self, k)
	}
	fn keys<'a>(&'a self) -> impl Iterator<Item = &'a K>
	where
		K: 'a,
	{
		std::collections::HashMap::<K, V>::keys(self)
	}
}

impl<K: Eq + PartialEq + Ord + PartialOrd, V> MapLike<K, V> for alloc::collections::BTreeMap<K, V> {
	fn insert(&mut self, k: K, v: V) -> Option<V> {
		alloc::collections::BTreeMap::<K, V>::insert(self, k, v)
	}
	fn extend(&mut self, iter: impl Iterator<Item = (K, V)>) {
		<alloc::collections::BTreeMap<K, V> as Extend<(K, V)>>::extend(self, iter)
	}
	fn contains_key(&self, k: &K) -> bool {
		alloc::collections::BTreeMap::<K, V>::contains_key(self, k)
	}
	fn remove(&mut self, k: &K) -> Option<V> {
		alloc::collections::BTreeMap::<K, V>::remove(self, k)
	}
	fn get(&self, k: &K) -> Option<&V> {
		alloc::collections::BTreeMap::<K, V>::get(self, k)
	}
	fn get_mut(&mut self, k: &K) -> Option<&mut V> {
		alloc::collections::BTreeMap::<K, V>::get_mut(self, k)
	}
	fn keys<'a>(&'a self) -> impl Iterator<Item = &'a K>
	where
		K: 'a,
	{
		alloc::collections::BTreeMap::<K, V>::keys(self)
	}
}

/// An mapping of key/value pairs stored as pairs ordered by their key in a [Vec].
///
/// This is always efficient for small sizes of mappings, and efficient for large sizes when
/// insertion is not needed (i.e. items are placed into the mapping in bulk).
#[derive(Clone, Encode, Eq, PartialEq, Ord, PartialOrd)]
pub struct VecMap<K, V>(Vec<(K, V)>);
impl<K: Eq + PartialEq + Ord + PartialOrd, V> VecMap<K, V> {
	/// Create a new, empty instance.
	pub fn new() -> Self {
		Self(Vec::new())
	}
	/// Create a new instance from a sorted [Vec].
	///
	/// Returns [Ok] with an instance containing the same items as `v`, or [Err] if `v` is unsorted.
	pub fn from_sorted(v: Vec<(K, V)>) -> Result<Self, ()> {
		let Some((first, _)) = v.first() else { return Ok(Self::new()) };
		v.iter()
			.skip(1)
			.try_fold(first, |a, (e, _)| if a < e { Some(e) } else { None })
			.ok_or(())?;
		Ok(Self(v))
	}
	/// Return the number of items this mapping contains.
	pub fn len(&self) -> usize {
		self.0.len()
	}
	/// Return `true` if this mapping is empty.
	pub fn is_empty(&self) -> bool {
		self.0.is_empty()
	}
	/// Return an iterator over the key/value pairs in this mapping in order.
	pub fn iter(&self) -> core::slice::Iter<(K, V)> {
		self.0.iter()
	}
	/// Return an iterator over the keys in this mapping in order.
	pub fn keys(&self) -> impl Iterator<Item = &K> {
		self.0.iter().map(|(k, _)| k)
	}
	/// Return an iterator over the values in this mapping in order of their corresponding key.
	pub fn values(&self) -> impl Iterator<Item = &V> {
		self.0.iter().map(|(_, v)| v)
	}
	/// Get the value of the pair with a particular key.
	///
	/// Returns a reference the value of the item in the mapping whose key equals `k`.
	pub fn get(&self, k: &K) -> Option<&V> {
		self.0.binary_search_by(|x| x.0.cmp(k)).ok().map(|i| &self.0[i].1)
	}
	/// Get a mutable reference to the value of the pair with a particular key.
	///
	/// Returns a mutable reference the value of the item in the mapping whose key equals `k`.
	pub fn get_mut(&mut self, k: &K) -> Option<&mut V> {
		self.0.binary_search_by(|x| x.0.cmp(k)).ok().map(move |i| &mut self.0[i].1)
	}
	/// Consume this mapping and return a [Vec] of transformed pairs.
	///
	/// Returns the [Vec] of the resultant values of applying `f` to each pair in the mapping in
	/// order.
	pub fn map<U>(self, mut f: impl FnMut(K, V) -> U) -> Vec<U> {
		self.0.into_iter().map(|(k, v)| f(k, v)).collect::<Vec<_>>()
	}
	/// Transform all pairs by reference and return a [Vec] with the results.
	///
	/// Returns the [Vec] of the resultant values of applying `f` to each pair by reference in the
	/// mapping in order.
	pub fn map_ref<U>(&self, mut f: impl FnMut(&K, &V) -> U) -> Vec<U> {
		self.0.iter().map(|(k, v)| f(k, v)).collect::<Vec<_>>()
	}
	/// Return a [Vec] of sorted key/value pairs by cloning this mapping.
	pub fn to_vec(&self) -> Vec<(K, V)>
	where
		K: Clone,
		V: Clone,
	{
		self.0.clone()
	}
	/// Return the [Vec] of sorted key/value pairs by consuming this mapping.
	pub fn into_vec(self) -> Vec<(K, V)> {
		self.0
	}
	/// Insert pair into the mapping.
	///
	/// Inserts some pair (`k`, `v`) into the mapping, replacing the existing pair whose key is `k`,
	/// if any.
	///
	/// Returns [Some] value which was replaced, if any.
	///
	/// NOTE: This does an ordered insert and thus is slow. If you're inserting multiple items, use
	/// [Self::extend] which is much more efficient or consider using an alternative data structure
	/// if you're doing this a lot.
	pub fn insert(&mut self, k: K, v: V) -> Option<(K, V)> {
		match self.0.binary_search_by(|x| x.0.cmp(&k)) {
			Ok(i) => Some(core::mem::replace(&mut self.0[i], (k, v))),
			Err(i) => {
				self.0.insert(i, (k, v));
				None
			},
		}
	}
	/// Insert multiple pairs from an iterator.
	///
	/// Replaces any existing pairs which share a key with an element of the iterator.
	pub fn extend(&mut self, iter: impl IntoIterator<Item = (K, V)>) {
		self.0.splice(0..0, iter);
		self.0.sort_by(|x, y| x.0.cmp(&y.0));
		self.0.dedup_by(|x, y| x.0 == y.0);
	}
	/// Check if a key is in the mapping.
	///
	/// Returns `true` iff the mapping contains a pair whose key equals `k`.
	pub fn contains_key(&self, k: &K) -> bool {
		self.0.binary_search_by(|x| x.0.cmp(k)).is_ok()
	}
	/// Remove an item from the mapping by key.
	///
	/// Removes the item with key equal to `k` from the mapping.
	///
	/// Returns the value of the item removed, if any.
	pub fn remove(&mut self, k: &K) -> Option<(K, V)> {
		match self.0.binary_search_by(|x| x.0.cmp(k)) {
			Ok(i) => Some(self.0.remove(i)),
			Err(_) => None,
		}
	}
	/// Filter items from the mapping.
	///
	/// Removes all pairs from the mapping for which `f` returns `false`.
	pub fn retain(&mut self, mut f: impl FnMut(&K, &V) -> bool) {
		self.0.retain(|x| f(&x.0, &x.1));
	}
	// TODO: Create traits `OrderedSetLike`/`OrderedMapLike` and make work with them.
	/// Compares the pairs in two mappings.
	///
	/// Returns `true` iff every pair in the mapping is not found in `other`.
	pub fn is_disjoint(&self, other: &VecMap<K, V>) -> bool
	where
		V: Ord,
	{
		vec_set::is_disjoint(self.iter(), other.iter())
	}
	/// Compares the keys in two mappings.
	///
	/// Returns `true` iff every key in the mapping is not found in the keys of `other`.
	pub fn keys_disjoint<W>(&self, other: &VecMap<K, W>) -> bool {
		vec_set::is_disjoint(self.keys(), other.keys())
	}
	/// Compares the keys in this mapping with the values in a set.
	///
	/// Returns `true` iff every key in the mapping is not found in `other`.
	pub fn keys_disjoint_with_set(&self, other: &VecSet<K>) -> bool {
		vec_set::is_disjoint(self.keys(), other.iter())
	}
}

impl<K: Eq + PartialEq + Ord + PartialOrd, V> MapLike<K, V> for VecMap<K, V> {
	fn insert(&mut self, k: K, v: V) -> Option<V> {
		VecMap::<K, V>::insert(self, k, v).map(|(_, v)| v)
	}
	fn extend(&mut self, iter: impl Iterator<Item = (K, V)>) {
		VecMap::<K, V>::extend(self, iter)
	}
	fn contains_key(&self, k: &K) -> bool {
		VecMap::<K, V>::contains_key(self, k)
	}
	fn remove(&mut self, k: &K) -> Option<V> {
		VecMap::<K, V>::remove(self, k).map(|x| x.1)
	}
	fn get(&self, k: &K) -> Option<&V> {
		VecMap::<K, V>::get(self, k)
	}
	fn get_mut(&mut self, k: &K) -> Option<&mut V> {
		VecMap::<K, V>::get_mut(self, k)
	}
	fn keys<'a>(&'a self) -> impl Iterator<Item = &'a K>
	where
		K: 'a,
	{
		VecMap::<K, V>::keys(self)
	}
}

impl<K: fmt::Debug, V: fmt::Debug> fmt::Debug for VecMap<K, V> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self.0.len() {
			0 => write!(f, "[]"),
			_ => {
				write!(f, "[{:?}=>{:?}", self.0[0].0, self.0[0].1)?;
				for i in 1..self.0.len() {
					write!(f, ", {:?}=>{:?}", self.0[i].0, self.0[i].1)?;
				}
				write!(f, "]")
			},
		}
	}
}

impl<K: fmt::Display, V: fmt::Display> fmt::Display for VecMap<K, V> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self.0.len() {
			0 => write!(f, "[]"),
			_ => {
				write!(f, "[{}=>{}", self.0[0].0, self.0[0].1)?;
				for i in 1..self.0.len() {
					write!(f, ", {}=>{}", self.0[i].0, self.0[i].1)?;
				}
				write!(f, "]")
			},
		}
	}
}

impl<K: Decode + Eq + PartialEq + Ord + PartialOrd, V: Decode> Decode for VecMap<K, V> {
	fn decode<I: codec::Input>(input: &mut I) -> Result<Self, DecodeError> {
		Ok(Self::from_sorted(Vec::<(K, V)>::decode(input)?).map_err(|()| "set out-of-order")?)
	}
}

impl<K, V> Default for VecMap<K, V> {
	fn default() -> Self {
		Self(Vec::new())
	}
}
impl<K, V> AsRef<[(K, V)]> for VecMap<K, V> {
	fn as_ref(&self) -> &[(K, V)] {
		&self.0[..]
	}
}
impl<K, V> AsMut<[(K, V)]> for VecMap<K, V> {
	fn as_mut(&mut self) -> &mut [(K, V)] {
		&mut self.0[..]
	}
}

impl<K, V> From<VecMap<K, V>> for Vec<(K, V)> {
	fn from(s: VecMap<K, V>) -> Vec<(K, V)> {
		s.0
	}
}
impl<K: Eq + PartialEq + Ord + PartialOrd, V> From<Vec<(K, V)>> for VecMap<K, V> {
	fn from(mut v: Vec<(K, V)>) -> Self {
		v.sort_by(|x, y| x.0.cmp(&y.0));
		v.dedup_by(|x, y| x.0 == y.0);
		Self(v)
	}
}
impl<K: Eq + PartialEq + Ord + PartialOrd, V> FromIterator<(K, V)> for VecMap<K, V> {
	fn from_iter<I: IntoIterator<Item = (K, V)>>(iter: I) -> Self {
		Vec::<(K, V)>::from_iter(iter).into()
	}
}
impl<K: Eq + PartialEq + Ord + PartialOrd, V> Extend<(K, V)> for VecMap<K, V> {
	fn extend<I: IntoIterator<Item = (K, V)>>(&mut self, iter: I) {
		VecMap::<K, V>::extend(self, iter)
	}
}
impl<K: Eq + PartialEq + Ord + PartialOrd, V> IntoIterator for VecMap<K, V> {
	type Item = (K, V);
	type IntoIter = <Vec<(K, V)> as IntoIterator>::IntoIter;

	fn into_iter(self) -> Self::IntoIter {
		self.0.into_iter()
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn encode_decode_works() {
		let v = VecMap::<u32, u32>::from_iter((0..100).map(|j| ((j * 97) % 101, j)));
		println!("{v}");
		let e = v.encode();
		let d = VecMap::<u32, u32>::decode(&mut &e[..]).unwrap();
		assert_eq!(v, d);
	}
}
