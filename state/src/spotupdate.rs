use antbox_geom::BoundPoint;
use derive_new::new;

use crate::State;

#[derive(Debug, new)]
pub(crate) struct SpotUpdate<'a, R> {
    pub(crate) rng: &'a mut R,
    pub(crate) state: &'a mut State,
    pub(crate) pt: BoundPoint,
}
