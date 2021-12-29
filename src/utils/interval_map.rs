/// Quickly in which interval a value is present
pub struct IntervalMap<K, V> {
    points: Vec<(K, Vec<V>)>,
}

impl<K, V> Default for IntervalMap<K, V> {
    fn default() -> Self {
        Self { points: Vec::new() }
    }
}

use std::cmp::{Ord, Ordering::*};
use std::ops::Range;

impl<K, V> IntervalMap<K, V>
where
    K: Ord,
    V: Clone,
{
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, range: Range<K>, value: V) {
        let position = match self
            .points
            .binary_search_by(|&(ref point, _)| point.cmp(&range.start))
        {
            Ok(position) => position,
            Err(position) => {
                self.points.insert(
                    position,
                    (
                        range.start,
                        self.points
                            .get(position.wrapping_sub(1))
                            .map(|(_, values)| values.clone())
                            .unwrap_or_default(),
                    ),
                );
                position
            }
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
                }
            }
        }
        self.points.push((range.end, Vec::new()))
    }

    pub fn get(&self, key: &K) -> std::slice::Iter<V> {
        let index = match self
            .points
            .binary_search_by(|&(ref point, _)| point.cmp(key))
        {
            Err(index) => {
                if index == 0 {
                    return (&[]).iter();
                } else {
                    index - 1
                }
            }
            Ok(index) => index,
        };
        self.points
            .get(index)
            .map(|point| &point.1[..])
            .unwrap_or(&[])
            .iter()
    }
}

#[cfg(test)]
mod tests {
    use super::IntervalMap;

    #[test]
    fn basic() {
        let mut interval_map = IntervalMap::new();

        interval_map.push(10..50, 1);
        interval_map.push(30..55, 2);

        assert_eq!(interval_map.get(&0).next(), None);
        assert_eq!(interval_map.get(&100).next(), None);

        let mut result = interval_map.get(&20);
        assert_eq!(result.next(), Some(&1));
        assert_eq!(result.next(), None);

        let mut result = interval_map.get(&53);
        assert_eq!(result.next(), Some(&2));
        assert_eq!(result.next(), None);

        let mut result = interval_map.get(&40);
        assert_eq!(result.next(), Some(&1));
        assert_eq!(result.next(), Some(&2));
        assert_eq!(result.next(), None);
    }
}
