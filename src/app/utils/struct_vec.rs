use interoptopus::ffi_type;
use crate::app::utils::zero;

#[ffi_type]
#[repr(C)]
#[derive(Clone)]
pub struct StructVec8<T> {
    items: [T; 8],
    count: u8,
}

impl<T> StructVec8<T> {
    pub fn len(&self) -> u8 { self.count }
    pub fn is_empty(&self) -> bool { self.count == 0 }

    pub fn add(&mut self, value: T) -> Result<(), String> {
        if self.count >= 8 {
            return Err("Vector is full".to_string());
        }
        self.items[self.count as usize] = value;
        self.count += 1;
        Ok(())
    }

    pub fn clean(&mut self) {
        for item in &mut self.items {
            *item = zero::<T>();
        }
        self.count = 0;
    }

    pub fn iter_mut(&mut self) -> StructVec5IterMut<T> {
        StructVec5IterMut {
            vec: self,
            current: 0,
        }
    }

    pub fn iter_ref(&self) -> StructVec5IterRef<T> {
        StructVec5IterRef {
            vec: self,
            current: 0,
        }
    }
}

impl<T> Default for StructVec8<T> {
    fn default() -> Self {
        Self {
            items: [
                zero::<T>(),
                zero::<T>(),
                zero::<T>(),
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
    vec: &'a mut StructVec8<T>,
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
    vec: &'a StructVec8<T>,
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

impl<T> FromIterator<T> for StructVec8<T> {
    fn from_iter<V: IntoIterator<Item=T>>(iter: V) -> Self {
        let mut vec = StructVec8::default();
        for val in iter {
            if let Err(error) = vec.add(val) {
                panic!("{}", error)
            }
        }
        vec
    }
}