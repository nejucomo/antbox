use antbox_gameboard::{BoardState as AntboxState, GenParams};
use antbox_geom::Dimensions;
use antbox_render::{Backend, RenderRefWithArg, RenderWithArg, Renderable};
use antbox_tick_timer::{RateLimiter, TickTimer};
use mstate::MStateIn;

use crate::abrender::spots_into_renderable;
use crate::gridlayout::GridLayout;
use crate::wyrgrid::WyrGrid;
use crate::{RunMode, UpdateEvent, layers};

const ANTBOX_FRAME_RATE: f64 = 5.0;

/// Encapsulate a [AntboxState] with extra animation-specific state
#[derive(Debug)]
pub struct AntboxAnimation {
    /// The runmode of the [AntboxState]
    pub runmode: RunMode,
    antbox: RateLimiter<AntboxState>,
    wyrgrid: WyrGrid,
}

impl AntboxAnimation {
    /// Initialize
    pub fn new<R: rand::Rng>(rng: &mut R, gp: GenParams, runmode: RunMode) -> Self {
        let antbox = gp.generate_state(rng);
        let antbox = RateLimiter::new(antbox, TickTimer::with_frame_rate(ANTBOX_FRAME_RATE));
        let wyrgrid = WyrGrid::new(antbox.bounds(), rng);

        AntboxAnimation {
            runmode,
            antbox,
            wyrgrid,
        }
    }
}

impl<'r, R> MStateIn<UpdateEvent<'r, R>> for AntboxAnimation
where
    R: rand::Rng,
{
    type Next = Self;

    fn into_with(self, upev: UpdateEvent<'r, R>) -> Self {
        if self.runmode.is_running() || upev.source.is_step() {
            AntboxAnimation {
                antbox: self.antbox.into_with(upev.rng),
                ..self
            }
        } else {
            self
        }
    }
}

impl RenderRefWithArg<Dimensions> for AntboxAnimation {
    fn render_ref_with_arg<B: ?Sized + Backend>(&self, rb: &mut B, view_size: Dimensions) {
        let layout = GridLayout::new(self.antbox.bounds(), view_size);

        (
            layers::Background,
            spots_into_renderable(&self.antbox, layout, &self.wyrgrid),
            layers::WireFrame.with_render_arg(layout),
        )
            .render_to(rb);
    }
}
