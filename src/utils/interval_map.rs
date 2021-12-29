/// Quickly in which interval a value is present
pub struct IntervalMap<K,V> {
	points: Vec<(K, Vec<V>)>,
}

impl<K, V> Default for IntervalMap<K, V> {
	fn default() -> Self {
		Self { points: Vec::new() }
	}
}

use std::ops::Range;
use std::cmp::{Ord, Ordering::*};

impl<K, V> IntervalMap<K, V>
	where K: Ord, V: Clone
{
	pub fn new() -> Self {
		Self::default()
	}

	pub fn push(&mut self, range: Range<K>, value: V) {
		let position = match self.points
			.binary_search_by(
				|&(ref point, _)| point.cmp(&range.start)
			)
		{
			Ok(position) => position,
			Err(position) => {
				self.points.insert(
					position, (range.start, Vec::new())
				);
				position
			},
		};
		for (i, point) in self.points.iter_mut().enumerate().skip(position) {
			match point.0.cmp(&range.end) {
				Less => point.1.push(value.clone()),
				Equal => return,
				Greater => {
					let mut new_point = (range.end, point.1.clone());
					new_point.1.push(value);
					self.points.insert(i, new_point);
					return;
				},
			}
		}
		self.points.push((range.end, Vec::new()))
	}

	pub fn get(&self, key: &K) -> std::slice::Iter<V> {
		let index = match self.points.binary_search_by(|&(ref point, _)| point.cmp(key)) {
			Err(index) => if index == 0 {
				return (&[]).iter()
			} else {
				index - 1
			}
			Ok(index) => index,
		};
		self.points.get(index)
			.map(|point| &point.1[..]).unwrap_or(&[])
			.iter()
	}
}
