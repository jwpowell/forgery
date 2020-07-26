use std::collections::VecDeque;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Error {
    Empty,
    Full,
}

#[derive(Debug)]
struct Chunk<T> {
    item: Option<T>,
    repeat: usize,
}

impl<T> Chunk<T> {
    fn space() -> Chunk<T> {
        Chunk::spaces(1)
    }

    fn spaces(count: usize) -> Chunk<T> {
        Chunk {
            item: None,
            repeat: count,
        }
    }

    fn item(item: T) -> Chunk<T> {
        Chunk::items(item, 1)
    }

    fn items(item: T, count: usize) -> Chunk<T> {
        Chunk {
            item: Some(item),
            repeat: count,
        }
    }

    fn is_space(&self) -> bool {
        self.item.is_none()
    }

    fn is_item(&self) -> bool {
        self.item.is_some()
    }
}
#[derive(Debug)]
pub struct Belt<T> {
    /// Chunks on the belt
    chunks: VecDeque<Chunk<T>>,

    /// Number of items the belt can hold
    capacity: usize,

    /// Number of items in the belt currently
    item_count: usize,
}

impl<T> Belt<T>
where
    T: Clone + Eq,
{
    /// Creates a belt with the specified capacity
    pub fn new(capacity: usize) -> Belt<T> {
        let mut belt = Belt {
            chunks: VecDeque::new(),
            capacity,
            item_count: 0,
        };

        belt.init();

        belt
    }

    fn init(&mut self) {
        self.chunks.push_back(Chunk::spaces(self.capacity));
        self.item_count = 0;
    }

    /// Returns the number of items this belt can hold
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Returns the number of items the belt holds currently
    pub fn count(&self) -> usize {
        self.item_count
    }

    /// Returns the element at the front of the belt
    pub fn peek_front(&self) -> Option<&T> {
        for chunk in self.chunks.iter() {
            if chunk.repeat > 0 {
                return chunk.item.as_ref();
            }
        }

        unreachable!("should always have a non-zero length chunk on belt");
    }

    /// Returns the element at the back of the belt
    pub fn peek_back(&self) -> Option<&T> {
        for chunk in self.chunks.iter().rev() {
            if chunk.repeat > 0 {
                return chunk.item.as_ref();
            }
        }

        unreachable!("should always have a non-zero length chunk on belt");
    }

    /// Indicates if `take()` can be successfully called
    pub fn can_take(&self) -> bool {
        self.peek_front().is_some()
    }

    /// Indicates if `put()` can be successfully called
    pub fn can_put(&self) -> bool {
        self.peek_back().is_none()
    }

    /// Remove zero-length chunks from the front of the belt
    fn strip_front(&mut self) {
        while self.chunks.front().unwrap().repeat == 0 {
            self.chunks.pop_front();
        }
    }

    /// Removes zero-length chunks from the back of the belt
    fn strip_back(&mut self) {
        while self.chunks.back().unwrap().repeat == 0 {
            self.chunks.pop_back();
        }
    }

    /// Removes zero-length chunks from the front and back of the belt
    fn strip(&mut self) {
        self.strip_front();
        self.strip_back();
    }

    /// Takes an element from the front of the belt
    pub fn take(&mut self) -> Result<T, Error> {
        self.strip_front();

        let front = self.chunks.front_mut().unwrap();

        if front.is_space() {
            return Err(Error::Empty);
        }

        let item = front.item.clone().unwrap();

        front.repeat -= 1;
        self.strip_front();

        let front = self.chunks.front_mut().unwrap();

        if front.is_space() {
            front.repeat += 1;
        } else {
            self.chunks.push_front(Chunk::space());
        }

        self.item_count -= 1;

        Ok(item)
    }

    pub fn put(&mut self, item: T) -> Result<(), Error> {
        self.strip_back();

        let back = self.chunks.back_mut().unwrap();

        if back.is_item() {
            return Err(Error::Full);
        }

        back.repeat -= 1;
        self.strip_back();

        let back = self.chunks.back_mut().unwrap();

        if let Some(ref it) = back.item {
            if it == &item {
                back.repeat += 1;
            } else {
                self.chunks.push_back(Chunk::item(item));
            }
        } else {
            self.chunks.push_back(Chunk::item(item));
        }

        self.item_count += 1;

        Ok(())
    }

    fn push_back_space(&mut self) {
        let back = self.chunks.back_mut().unwrap();

        if back.is_space() {
            back.repeat += 1;
        } else {
            self.chunks.push_back(Chunk::space());
        }
    }

    /// Advances the belt one space forward. Returns `Some(position)` to
    /// indicate that all items before `position` moved forward one position.
    /// Returns `None` if the belt has no space.
    pub fn advance(&mut self) -> Option<usize> {
        let mut pos = self.capacity;

        for chunk in self.chunks.iter_mut() {
            if chunk.is_space() && chunk.repeat > 0 {
                chunk.repeat -= 1;

                self.push_back_space();
                return Some(pos);
            }

            pos -= chunk.repeat;
        }

        None
    }

    /// Clears the belt
    pub fn clear(&mut self) {
        self.chunks.clear();
        self.init();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn init_01() {
        const CAPACITY: usize = 5;
        let mut belt: Belt<u32> = Belt::new(CAPACITY);

        assert!(!belt.can_take());
        assert!(belt.can_put());
        assert_eq!(belt.capacity(), CAPACITY);
        assert_eq!(belt.count(), 0);
    }

    #[test]
    fn put_01() {
        const CAPACITY: usize = 5;
        let mut belt: Belt<u32> = Belt::new(CAPACITY);

        assert_eq!(belt.put(101), Ok(()));

        assert!(!belt.can_take());
        assert!(!belt.can_put());
        assert_eq!(belt.capacity(), CAPACITY);
        assert_eq!(belt.count(), 1);
    }

    #[test]
    fn put_to_saturate_01() {
        const CAPACITY: usize = 5;
        let mut belt = Belt::new(CAPACITY);

        for i in 1..=CAPACITY {
            assert!(belt.can_put());
            assert_eq!(belt.capacity(), CAPACITY);
            assert_eq!(belt.count(), i - 1);

            assert_eq!(belt.put(i), Ok(()));

            assert!(!belt.can_put());
            assert_eq!(belt.capacity(), CAPACITY);
            assert_eq!(belt.count(), i);

            if i < CAPACITY {
                assert_eq!(belt.advance(), Some(CAPACITY));
            } else {
                assert_eq!(belt.advance(), None);
            }
        }

        assert!(belt.can_take());
        assert!(!belt.can_put());
        assert_eq!(belt.capacity(), CAPACITY);
        assert_eq!(belt.count(), 5);
    }

    #[test]
    fn take_to_empty_01() {
        const CAPACITY: usize = 5;
        let mut belt = Belt::new(CAPACITY);

        for i in 1..=CAPACITY {
            belt.put(i);
            belt.advance();
        }

        assert!(belt.can_take());
        assert!(!belt.can_put());
        assert_eq!(belt.capacity(), CAPACITY);
        assert_eq!(belt.count(), 5);

        for i in 1..=CAPACITY {
            assert_eq!(belt.take(), Ok(i));
            assert_eq!(belt.advance(), Some(CAPACITY));
        }

        assert!(!belt.can_take());
        assert!(belt.can_put());
        assert_eq!(belt.capacity(), CAPACITY);
        assert_eq!(belt.count(), 0);
    }

    #[test]
    fn put_with_gaps_01() {
        const CAPACITY: usize = 5;
        let mut belt = Belt::new(CAPACITY);

        belt.put('a');
        belt.advance();
        belt.advance();

        dbg!(&belt);
        assert!(!belt.can_take());
        assert!(belt.can_put());
        assert_eq!(belt.capacity(), CAPACITY);
        assert_eq!(belt.count(), 1);
    }

    #[test]
    fn put_with_gaps_02() {
        const CAPACITY: usize = 5;
        let mut belt = Belt::new(CAPACITY);

        belt.put('a');
        belt.advance();
        belt.advance();
        belt.advance();
        belt.advance();

        dbg!(&belt);
        assert!(belt.can_take());
        assert!(belt.can_put());
        assert_eq!(belt.capacity(), CAPACITY);
        assert_eq!(belt.count(), 1);
    }

    #[test]
    fn put_with_gaps_03() {
        const CAPACITY: usize = 5;
        let mut belt = Belt::new(CAPACITY);

        belt.put('a');
        belt.advance();
        belt.put('b');
        belt.advance();

        belt.put('c');
        belt.advance();
        belt.advance();

        belt.put('d');

        dbg!(&belt);
        assert_eq!(belt.advance(), Some(2));
    }
}
