use interoptopus::ffi_type;
use crate::app::utils::zero;

#[ffi_type]
#[repr(C)]
#[derive(Clone)]
pub struct StructVec5<T> {
    items: [T; 5],
    count: u8,
}

impl<T> StructVec5<T> {
    pub fn new1(item1: T) -> Self {
        Self {
            items: [
                item1,
                zero::<T>(),
                zero::<T>(),
                zero::<T>(),
                zero::<T>(),
            ],
            count: 1,
        }
    }

    pub fn new2(item1: T, item2: T) -> Self {
        Self {
            items: [
                item1,
                item2,
                zero::<T>(),
                zero::<T>(),
                zero::<T>(),
            ],
            count: 2,
        }
    }

    pub fn new3(item1: T, item2: T, item3: T) -> Self {
        Self {
            items: [
                item1,
                item2,
                item3,
                zero::<T>(),
                zero::<T>(),
            ],
            count: 3,
        }
    }

    pub fn new4(item1: T, item2: T, item3: T, item4: T) -> Self {
        Self {
            items: [
                item1,
                item2,
                item3,
                item4,
                zero::<T>(),
            ],
            count: 4,
        }
    }

    pub fn new5(item1: T, item2: T, item3: T, item4: T, item5: T) -> Self {
        Self {
            items: [
                item1,
                item2,
                item3,
                item4,
                item5,
            ],
            count: 5,
        }
    }

    pub fn len(&self) -> u8 { self.count }

    pub fn map<V>(&self, mapper: impl Fn(&T) -> V) -> StructVec5<V> {
        match self.count {
            1 => StructVec5::new1(mapper(&self.items[0])),
            2 => StructVec5::new2(mapper(&self.items[0]), mapper(&self.items[1])),
            3 => StructVec5::new3(mapper(&self.items[0]), mapper(&self.items[1]), mapper(&self.items[2])),
            4 => StructVec5::new4(mapper(&self.items[0]), mapper(&self.items[1]), mapper(&self.items[2]), mapper(&self.items[3])),
            5 => StructVec5::new5(mapper(&self.items[0]), mapper(&self.items[1]), mapper(&self.items[2]), mapper(&self.items[3]), mapper(&self.items[4])),
            idx => { panic!("Unexpected index {}", idx) }
        }
    }

    pub fn add(&mut self, value: T) -> Result<(), String> {
        if self.count >= 5 {
            return Err("Vector is full".to_string());
        }
        self.items[self.count as usize] = value;
        self.count += 1;
        Ok(())
    }

    pub fn iter_mut(&mut self) -> StructVec5IterMut<T> {
        StructVec5IterMut {
            vec: self,
            current: 0,
        }
    }

    pub fn iter_ref(&mut self) -> StructVec5IterRef<T> {
        StructVec5IterRef {
            vec: self,
            current: 0,
        }
    }
}

impl<T> Default for StructVec5<T> {
    fn default() -> Self {
        Self {
            items: [
                zero::<T>(),
                zero::<T>(),
                zero::<T>(),
                zero::<T>(),
                zero::<T>(),
            ],
            count: 0,
        }
    }
}

pub struct StructVec5IterMut<'a, T> {
    vec: &'a mut StructVec5<T>,
    current: u8,
}

impl<'a, T> Iterator for StructVec5IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.vec.count {
            let ptr: *mut T = &mut self.vec.items[self.current as usize];
            let reference = unsafe { &mut *ptr };
            self.current += 1;
            Some(reference)
        } else {
            None
        }
    }
}

pub struct StructVec5IterRef<'a, T> {
    vec: &'a StructVec5<T>,
    current: u8,
}

impl<'a, T> Iterator for StructVec5IterRef<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.vec.count {
            let ptr: *const T = &self.vec.items[self.current as usize];
            let reference = unsafe { &*ptr };
            self.current += 1;
            Some(reference)
        } else {
            None
        }
    }
}