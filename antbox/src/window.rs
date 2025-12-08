use antbox_engine::Notification;
use antbox_state::GenParams;
use speedy2d::window::{
    KeyScancode, VirtualKeyCode, WindowCreationOptions, WindowHandler, WindowHelper,
    WindowStartupInfo,
};
use speedy2d::{Graphics2D, Window};
use state_slot::Slot;

use crate::Result;
use crate::notifier::SpeedyNotifier;
use crate::winstate::WinState;

/// # TODO
///
/// - Hide the states privately behind public interface
#[derive(Debug)]
pub struct AntBoxWindow(Slot<WinState>);

impl AntBoxWindow {
    pub fn run(genparams: GenParams) -> Result<()> {
        let w = Window::new_with_user_events(
            env!("CARGO_PKG_NAME"),
            WindowCreationOptions::new_fullscreen_borderless(),
        )?;
        w.run_loop(Self(Slot::from(WinState::from(genparams))));
    }
}

impl WindowHandler<Notification> for AntBoxWindow {
    fn on_user_event(&mut self, helper: &mut WindowHelper<Notification>, notif: Notification) {
        use Notification::NewState;

        match notif {
            NewState(ns) => {
                log::debug!("Received new antbox state: {:?}", ns.gencnt);
                self.0.update(ns);
                helper.request_redraw();
            }
        };
    }

    fn on_start(&mut self, helper: &mut WindowHelper<Notification>, info: WindowStartupInfo) {
        let viewsize = *info.viewport_size_pixels();
        let sfactor = info.scale_factor();
        log::info!("viewsize: {:?}, scaling factor: {:?}", viewsize, sfactor);

        self.0
            .update(SpeedyNotifier::from(helper.create_user_event_sender()));
        helper.request_redraw();
    }

    fn on_draw(&mut self, helper: &mut WindowHelper<Notification>, graphics: &mut Graphics2D) {
        let winsize = helper.get_size_pixels().into_f32();
        self.0.draw(graphics, winsize);
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
