use crate::prelude::*;

mod sun;

pub use self::sun::*;

// INFO: Not making something pub means it can be 'imported' (but import statement will still be needed)
// to work across modules
pub trait Asset2D {
    fn tick(&self, ctx: &Draw);
}
