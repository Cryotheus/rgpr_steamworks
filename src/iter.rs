use crate::Private;
use futures::Stream;
use num_traits::One;
use std::collections::HashSet;
use std::fmt::{Debug, Formatter};
use std::hash::Hash;
use std::iter::{Fuse, FusedIterator};
use std::ops::{Add, Deref, DerefMut};
use std::pin::Pin;
use std::task::{Context, Poll};

/// Adapater which removes duplicate items and [fuses] the iterator.
///
/// Hangs when adapting a cyclic iterator.
///
/// [fuses]: Fuse
pub struct Clean<I: Iterator> {
	recording: HashSet<I::Item>,
	iter: Fuse<I>,
}

impl<I: Iterator> Clean<I> {
	/// Returns a [`HashSet`] containing all previously yielded items.
	pub fn as_record(&self) -> &HashSet<I::Item> {
		&self.recording
	}

	/// Consumes the iterator returning its [`HashSet`] which contains previously yielded items.
	pub fn into_record(self) -> HashSet<I::Item> {
		self.recording
	}
}

impl<I: Iterator> Iterator for Clean<I>
where
	<I as Iterator>::Item: Clone + Eq + Hash,
{
	type Item = I::Item;

	fn next(&mut self) -> Option<Self::Item> {
		//iterate until we find an entry we haven't seen
		while let Some(item) = self.iter.next() {
			if self.recording.contains(&item) {
				continue;
			}

			self.recording.insert(item.clone());

			return Some(item);
		}

		None
	}
}

impl<I: Iterator> FusedIterator for Clean<I> where Self: Iterator {}

impl<I: Debug + Iterator> Debug for Clean<I>
where
	I::Item: Debug,
{
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("Clean").field("recording", &self.recording).field("iter", &self.iter).finish()
	}
}

/// For iterators sourced from the Steam API,
/// which typically have race conditions.
///
/// Not meant to be externally implemented or used directly,
/// Nominally contained inside an [`Unreliable`],
/// which safely implements [`Iterator`] for the `SteamApiIterator`.
pub unsafe trait SteamApiIterator {
	/// The type the iterator yields.
	type Item;

	/// The integer type used for the called Steam API functions.
	///
	/// Typically [`c_int`] or [`c_uint`].
	///
	/// [`c_int`]: std::ffi::c_int
	/// [`c_uint`]: std::ffi::c_uint
	type Index: num_traits::PrimInt;

	/// Skips `n` entries without iterating.
	fn skip(&mut self, n: impl Into<Self::Index>) {
		let cursor = self.steam_api_cursor(Private);
		*cursor = cursor.add(n.into());
	}

	/// Wraps the iterator into a type which implements [`Iterator`] for `SteamApiIterator` implementers.
	/// Used internally in interfaces which provide unreliable iterators.
	fn wrap(self) -> Unreliable<Self>
	where
		Self: Sized,
	{
		unsafe { self.steam_api_setup(Private) };

		Unreliable(self)
	}

	/// # Private
	#[doc(hidden)]
	fn steam_api_cursor(&mut self, _: Private) -> &mut Self::Index;

	/// # Private
	///
	/// # Safety
	#[doc(hidden)]
	unsafe fn steam_api_get(&self, index: Self::Index, _: Private) -> Option<Self::Item>;

	/// # Private
	/// Called by [`Unreliable`]'s [`Iterator`] implementation.
	#[doc(hidden)]
	fn steam_api_next(&mut self, _: Private) -> Option<Self::Item> {
		let cursor = self.steam_api_cursor(Private);
		let index = *cursor;
		*cursor = cursor.add(Self::Index::one());

		unsafe { self.steam_api_get(index, Private) }
	}

	/// Call the function from the Steam API for updating the API's list and count.
	/// Otherwise the index functions in the Steam API may not have the list.
	///
	/// # Private
	/// This is called automatically by [`wrap`].
	///
	/// # Safety
	/// This function is marked as `unsafe` as convenience for the implementations,
	/// which always require and `unsafe` block.
	///
	/// Ensure calling this function is safe,
	/// even if it is called more than once on the same iterator.
	///
	/// [`wrap`]: Self::wrap
	#[doc(hidden)]
	unsafe fn steam_api_setup(&self, _: Private) {}
}

/// For asynchronous iterators sourced from the Steam API,
/// which typically have race conditions.
///
/// Not meant to be externally implemented or used directly,
/// Nominally contained inside an [`Unreliable`],
/// which safely implements [`Stream`] for the `SteamApiStream`.
pub unsafe trait SteamApiStream {
	type Item;

	/// Wraps the stream into a type which implements [`Stream`] for `SteamApiStream` implementers.
	/// Used internally in interfaces which provide unreliable iterators.
	fn wrap(mut self) -> Unreliable<Self>
	where
		Self: Sized,
	{
		unsafe { self.steam_api_setup(Private) };

		Unreliable(self)
	}

	/// # Private
	///
	/// [Unimplemented] by default, as only a few Steam API iterators are async.
	///
	/// [Unimplemented]: unimplemented!
	fn steam_api_poll(self: Pin<&mut Self>, cx: &mut Context<'_>, _: Private) -> Poll<Option<Self::Item>>;

	/// # Private
	///
	/// Call count or initialization functions here.
	unsafe fn steam_api_setup(&mut self, _: Private) {}
}

/// An iterator or stream which sources data from the Steam API,
/// is unreliable, and typically suffers race conditions.
///
/// The Steam API unfortunately has race conditions when iterating lists.
/// Lists provided by the Steam API may be mutated during iteration,
/// even if iteration happens as quickly as possible.
/// This causes entries to be repeated or skipped during iteration.
///
/// Fortunately, the items in the lists themselves do not have these issues.
///
/// # Solutions
/// **If the [`Item`] type does not implement [`Hash`]**
/// iterate as you normally would, or [collect] into a [`Vec`].
///
/// **If ordering is not important**
/// you can [collect] the iterator into a [`HashSet`] to resolve issues with duplicates and gaps.
///
/// **If ordering is preferred**
/// use [`clean`] for an adaptor which removes duplicates,
/// [fuses] the iterator,
/// and maintains ordering to the best of its ability.
///
/// [`clean`]: Unreliable::clean
/// [collect]: Iterator::collect
/// [`Hash`]: Hash
/// [`HashSet`]: HashSet
/// [fuses]: Iterator::fuse
/// [`Item`]: Iterator::Item
/// [`Vec`]: Vec
#[repr(transparent)]
pub struct Unreliable<I>(pub(crate) I);

impl<I> Unreliable<I> {
	/// Wraps the interator in a [`Clean`] to improve reliability.
	/// Entries may still be skipped, as the underlying iterator is natively unrealiable.
	pub fn clean(self) -> Clean<I>
	where
		I: Iterator,
	{
		Clean {
			recording: HashSet::new(),
			iter: self.0.fuse(),
		}
	}
}

impl<I> Deref for Unreliable<I> {
	type Target = I;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl<I> DerefMut for Unreliable<I> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}

impl<I: SteamApiIterator> Iterator for Unreliable<I> {
	type Item = I::Item;

	fn next(&mut self) -> Option<Self::Item> {
		self.0.steam_api_next(Private)
	}
}

impl<I: SteamApiStream> Stream for Unreliable<I> {
	type Item = I::Item;

	fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
		let stream = unsafe { self.map_unchecked_mut(|Unreliable(stream)| stream) };

		stream.steam_api_poll(cx, Private)
	}
}

impl<I: Debug> Debug for Unreliable<I> {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		f.debug_tuple("Unreliable").field(&self.0).finish()
	}
}
