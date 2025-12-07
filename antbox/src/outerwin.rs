use antbox_engine::Notification;
use antbox_state::GenParams;
use derive_more::TryInto;
use speedy2d::window::{
    KeyScancode, VirtualKeyCode, WindowCreationOptions, WindowHandler, WindowHelper,
    WindowStartupInfo,
};
use speedy2d::{Graphics2D, Window};

use crate::Result;
use crate::notifier::SpeedyNotifier;
use crate::statewin::StateWin;

use WinState::*;

/// # TODO
///
/// - Hide the states privately behind public interface
#[derive(Debug)]
pub struct AntBoxWindow(WinState);

#[derive(Debug, TryInto)]
#[try_into(ref, ref_mut)]
enum WinState {
    Pending(GenParams),
    Generating,
    World(StateWin),
}

impl AntBoxWindow {
    pub fn new(genparams: GenParams) -> Self {
        AntBoxWindow(Pending(genparams))
    }

    pub fn run(self) -> Result<()> {
        assert!(matches!(&self.0, Pending { .. }));

        let w = Window::new_with_user_events(
            env!("CARGO_PKG_NAME"),
            WindowCreationOptions::new_fullscreen_borderless(),
        )?;
        w.run_loop(self);
    }
}

impl WindowHandler<Notification> for AntBoxWindow {
    fn on_user_event(&mut self, helper: &mut WindowHelper<Notification>, notif: Notification) {
        use Notification::NewState;

        match notif {
            NewState(ns) => {
                log::debug!("Received new antbox state: {:?}", ns.gencnt);
                assert!(matches!(&self.0, Generating | World(_)));
                self.0 = World(StateWin::from(ns));
                helper.request_redraw();
            }
        };
    }

    fn on_start(&mut self, helper: &mut WindowHelper<Notification>, info: WindowStartupInfo) {
        let viewsize = *info.viewport_size_pixels();
        let sfactor = info.scale_factor();
        log::info!("viewsize: {:?}, scaling factor: {:?}", viewsize, sfactor);

        let gp: GenParams = std::mem::replace(&mut self.0, Generating)
            .try_into()
            .unwrap();
        let notifier = SpeedyNotifier::from(helper.create_user_event_sender());
        antbox_engine::spawn(gp, notifier);

        helper.request_redraw();
    }

    fn on_draw(&mut self, helper: &mut WindowHelper<Notification>, graphics: &mut Graphics2D) {
        let stwin: &StateWin = (&self.0).try_into().unwrap();
        let winsize = helper.get_size_pixels().into_f32();
        stwin.draw(graphics, winsize);
    }

    fn on_key_down(
        &mut self,
        helper: &mut WindowHelper<Notification>,
        ovkc: Option<VirtualKeyCode>,
        _: KeyScancode,
    ) {
        use VirtualKeyCode::Escape;

        match ovkc {
            Some(Escape) => {
                log::info!("bye!");
                helper.terminate_loop();
            }
            _ => {
                // Ignore
            }
        }
    }
}
