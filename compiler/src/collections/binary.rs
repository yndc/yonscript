/**T : PartialOrd
 * Binary vector is a vector that is automatically sorted on insertion
 */
use std::ops::{Index, IndexMut};

struct BinaryVec<T>(Vec<T>)
where
    T: PartialOrd;

impl<T> BinaryVec<T>
where
    T: PartialOrd,
{
    pub fn new() -> BinaryVec<T> {
        BinaryVec(Vec::new())
    }

    pub fn push(&mut self, value: T) {
        if self.0.is_empty() {
            self.0.push(value);
        } else {
            let (mut start, mut end) = (0 as usize, self.len() - 1);
            loop {
                println!("start: {} end: {} mid: {}", start, end, mid(start, end));
                let range = end - start;
                if range <= 1 {
                    for i in start..(end + 1) {
                        if self[i].gt(&value) {
                            self.0.insert(i, value);
                            return;
                        }
                    }
                    self.0.push(value);
                    return;
                } else {
                    let pivot = mid(start, end);
                    if self[pivot].gt(&value) {
                        end = pivot;
                    } else {
                        start = pivot;
                    }
                }
            }
        }
    }

    // pub fn insert(&mut self, index: usize, value: T) {

    // }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl<T> Index<usize> for BinaryVec<T>
where
    T: PartialOrd,
{
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<T> IndexMut<usize> for BinaryVec<T>
where
    T: PartialOrd,
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

pub struct BinaryVecIterator<'a, T>
where
    T: PartialOrd,
{
    vec: &'a BinaryVec<T>,
    index: usize,
}

impl<'a, T> Iterator for BinaryVecIterator<'a, T>
where
    T: PartialOrd,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index == self.vec.0.len() {
            None
        } else {
            let item = &self.vec.0[self.index];
            self.index += 1;
            Some(item)
        }
    }
}

impl<'a, T> IntoIterator for &'a BinaryVec<T>
where
    T: PartialOrd,
{
    type Item = &'a T;
    type IntoIter = BinaryVecIterator<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        BinaryVecIterator {
            vec: &self,
            index: 0,
        }
    }
}

fn mid(start: usize, end: usize) -> usize {
    start + ((end - start) / 2)
}

#[cfg(test)]
mod test {
    use crate::collections::binary::BinaryVec;
    use rand;
    use std;

    #[test]
    fn push() {
        let n = 100000;
        let mut v: BinaryVec<i32> = BinaryVec::new();
        for _ in 0..n {
            v.push(rand::random::<i32>().abs() % 10);
        }
        assert_eq!(v.len(), n);
        let mut last = std::i32::MIN;
        for item in &v {
            if *item < last {
                panic!("wrong order")
            }
            last = *item;
        }
    }
}
