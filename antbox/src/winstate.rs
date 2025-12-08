use antbox_state::{GenParams, State};
use derive_more::{From, TryInto};
use mealy_machine::UpdateInput;
use speedy2d::Graphics2D;
use speedy2d::dimen::Vec2;

use crate::notifier::SpeedyNotifier;
use crate::stategfx::StateGfx;

#[derive(Debug, From, TryInto)]
#[try_into(owned, ref)]
pub(crate) enum WinState {
    PendingWindowStart(GenParams),
    PendingFirstState,
    Sgfx(StateGfx),
}

impl WinState {
    pub(crate) fn draw(&self, graphics: &mut Graphics2D, view_size: Vec2) {
        self.as_ref().draw(graphics, view_size);
    }
}

impl AsRef<StateGfx> for WinState {
    fn as_ref(&self) -> &StateGfx {
        self.try_into().unwrap()
    }
}

// Updates:
impl UpdateInput<State> for WinState {
    fn update_input(self, st: State) -> Self {
        Self::from(StateGfx::new(st))
    }
}

impl UpdateInput<SpeedyNotifier> for WinState {
    fn update_input(self, notifier: SpeedyNotifier) -> Self {
        let gp: GenParams = self.try_into().unwrap();
        antbox_engine::spawn(gp, notifier);
        Self::PendingFirstState
    }
}
