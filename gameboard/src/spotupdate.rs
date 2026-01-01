use antbox_grid::GridCoord;
use derive_new::new;

use crate::Field;

#[derive(Debug, new)]
pub(crate) struct SpotUpdate<'a, R> {
    pub(crate) rng: &'a mut R,
    pub(crate) field: &'a mut Field,
    pub(crate) pt: GridCoord,
}
