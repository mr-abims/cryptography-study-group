use generic_array::{ArrayLength, GenericArray};

#[cfg(test)]
pub mod testing;

pub struct GenArrayWrapper<T, N: ArrayLength<T>> {
    inner: GenericArray<T, N>,
}

impl<T, N: ArrayLength<T>> GenArrayWrapper<T, N> {
    pub fn new(gen_array: GenericArray<T, N>) -> Self {
        Self { inner: gen_array }
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub fn get_index(&self, idx: usize) -> Option<&T> {
        self.inner.get(idx)
    }
}
