use std::cell::UnsafeCell;

pub struct Cell<T> {
    value: UnsafeCell<T>
}

// implemented by UnsafeCell.
// syntax is only available in nightly mode.
// impl <T> !Sync for Cell<T> {}


// unsafe impl<T> Sync for Cell<T> {}

impl<T> Cell<T> {
    pub fn new(value: T) -> Self {
        Cell {
            value: UnsafeCell::new(value),
        }
    }

    pub fn set(&self, value: T) {
        // SAFETY: we know no-one is concurrently mutating self.value (cause of !Sync in UnsafeCell)
        // SAFETY: we know we're not invalidating any references, cause we ain't giving out any
        unsafe { *self.value.get() = value; }
    }

    pub fn get(&self) -> T
    where
        T : Copy
    {
        // SAFETY: since only only thread can mutate at any time, we are sure not to be modifying value
        // cause current thread is executing this fn.
        unsafe{ *self.value.get() }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use super::Cell;
}
