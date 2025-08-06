use std::{fmt::Debug, ops::Deref, sync::Arc};

use once_cell::sync::OnceCell;

/// A late initialized container which carries an [`Arc`].
/// This is used in cases where recursive dependency is needed.
/// The value provided must be an Arc.
#[derive(Debug)]
pub struct LateInit<T>(OnceCell<Arc<T>>);

impl<T> Deref for LateInit<T> {
    type Target = Arc<T>;

    fn deref(&self) -> &Self::Target {
        self.0.get().unwrap()
    }
}

impl<T: Debug> LateInit<T> {
    pub fn new() -> Self {
        Self(OnceCell::new())
    }

    pub fn init_value(&self, value: Arc<T>) {
        self.0.set(value).unwrap();
    }
}
