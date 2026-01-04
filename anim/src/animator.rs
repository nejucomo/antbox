use antbox_gameboard::GenParams;
use antbox_geom::Dimensions;
use antbox_render::{Backend, RenderRefWithArg, RenderWithArg as _};
use derive_debug::Dbg;
use moveslot::MoveSlot;
use mstate::Update as _;

use crate::antbox::AntboxAnimation;
use crate::upev::UpdateEvent;
use crate::{RunMode, UpdateSource};

/// The coordinator for animation state updates and rendering
#[derive(Dbg)]
pub struct Animator<R>
where
    R: rand::Rng,
{
    #[dbg(placeholder = "...")]
    rng: R,
    msaa: MoveSlot<AntboxAnimation>,
}

impl<R> Animator<R>
where
    R: rand::Rng,
{
    /// Create the [Animator]
    pub fn new(mut rng: R, gp: GenParams, runmode: RunMode) -> Self {
        let msaa = MoveSlot::from(AntboxAnimation::new(&mut rng, gp, runmode));
        Animator { rng, msaa }
    }

    /// Update from the given [UpdateSource]
    pub fn update(&mut self, source: UpdateSource) {
        self.msaa.update(UpdateEvent {
            rng: &mut self.rng,
            source,
        });
    }

    /// Toggle the [RunMode] of the [Animator]
    pub fn toggle_run_mode(&mut self) {
        self.msaa.runmode.toggle();
    }
}

impl<R> RenderRefWithArg<Dimensions> for Animator<R>
where
    R: rand::Rng,
{
    fn render_ref_with_arg<B: ?Sized + Backend>(&self, rb: &mut B, arg: Dimensions) {
        self.msaa.render_with_arg(rb, arg);
    }
}
