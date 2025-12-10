use derive_debug::Dbg;
use derive_more::{Deref, DerefMut};
use derive_new::new;
use speedy2d::Graphics2D;

use crate::{Drawable, GridLayout};

/// A wrapper for [Graphics2D] which also includes calculated grid-to-view dimensions
#[derive(Dbg, new, Deref, DerefMut)]
pub struct GfxLayout<'a> {
    #[deref]
    #[deref_mut]
    #[dbg(placeholder = "..")]
    g: &'a mut Graphics2D,

    /// The [GridLayout] for the [Graphics2D] and our implicit grid
    pub grid_layout: GridLayout,
}

impl<'a> GfxLayout<'a> {
    /// Draw an object onto `self`
    pub fn draw<D: Drawable>(&mut self, object: D) {
        object.draw_on(self)
    }
}
