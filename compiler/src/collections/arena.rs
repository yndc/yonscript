use std::{io::Read, mem, ops::{Index, IndexMut}};

#[derive(Debug)]
pub struct Arena<T> {
    data: Vec<Option<T>>,
    empty: Vec<usize>,
    versions: Vec<i32>,
    len: usize,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ID {
    index: usize,
    version: i32,
}

impl<T> Arena<T> {
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            empty: Vec::new(),
            versions: Vec::new(),
            len: 0,
        }
    }

    pub fn find(&self, id: ID) -> Option<&T> {
        if id.version == self.versions[id.index] {
            Some(self.data[id.index].as_ref().unwrap())
        } else {
            None
        }
    }

    pub fn find_mut(&mut self, id: ID) -> Option<&mut T> {
        if id.version == self.versions[id.index] {
            Some(self.data[id.index].as_mut().unwrap())
        } else {
            None
        }
    }

    pub fn push(&mut self, value: T) -> ID {
        self.len += 1;
        match self.empty.pop() {
            Some(index) => {
                let version = self.versions[index] * -1 + 1;
                self.data[index] = Some(value);
                return ID { index, version };
            }
            None => {
                self.data.push(Some(value));
                self.versions.push(0);
                return ID {
                    index: self.data.len() - 1,
                    version: 0,
                };
            }
        };
    }

    pub fn remove(&mut self, id: ID) -> Option<T> {
        self.len -= 1;
        if id.version == self.versions[id.index] {
            self.empty.push(id.index);
            self.versions[id.index] *= -1;
            mem::take(&mut self.data[id.index])
        } else {
            None
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }
}

impl<T> Index<ID> for Arena<T> {
    type Output = T;

    fn index(&self, index: ID) -> &Self::Output {
        &self.find(index).unwrap()
    }
}

impl<T> IndexMut<ID> for Arena<T> {
    fn index_mut(&mut self, index: ID) -> &mut Self::Output {
        self.find_mut(index).unwrap()
    }
}

// impl<T> IntoIterator for Arena<T> {
//     type Item = T;
//     type IntoIter = ArenaIterator<T>;

//     fn into_iter(self) -> Self::IntoIter {
//         ArenaIterator {
//             arena: self,
//             next: 0,
//         }
//     }
// }

// struct ArenaIterator<T> {
//     arena: Arena<T>,
//     next: usize,
// }

// impl<T> Iterator for ArenaIterator<T> {
//     type Item = T;

//     fn next(&mut self) -> Option<Self::Item> {
//         let mut index = self.next;
//         if index >= self.arena.len() {
//             None
//         } else {
//             while self.arena.versions[index] < 0 {
//                 index += 1;
//             }
//             Some(self.arena.data[index])
//         }
//     }
// }

#[cfg(test)]
mod test {

    #[test]
    fn my_test() {
        assert_eq!(1, 1);
    }
}
