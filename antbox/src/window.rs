use antbox_state::GenParams;
use antbox_tick_timer::TickTimer;
use mealy_machine::Slot;
use speedy2d::window::{
    KeyScancode, VirtualKeyCode, WindowCreationOptions, WindowHandler, WindowHelper,
    WindowStartupInfo,
};
use speedy2d::{Graphics2D, Window};

use crate::anim::AnimationState;
use crate::{Result, Tick};

/// # TODO
///
/// - Hide the states privately behind public interface
#[derive(Debug)]
pub struct AntBoxWindow {
    started: bool,
    anim: Slot<AnimationState>,
}

impl AntBoxWindow {
    pub fn run(gp: GenParams) -> Result<()> {
        let w = Window::new_with_user_events(
            env!("CARGO_PKG_NAME"),
            WindowCreationOptions::new_fullscreen_borderless(),
        )?;
        w.run_loop(Self::new(gp));
    }

    fn new(gp: GenParams) -> Self {
        AntBoxWindow {
            started: false,
            anim: Slot::from(AnimationState::new(gp.generate_state())),
        }
    }

    fn launch_tick_timer(&self, helper: &mut WindowHelper<Tick>) {
        assert!(!self.started);

        let uev = helper.create_user_event_sender();
        std::thread::spawn(move || {
            let mut tt = TickTimer::default();

            loop {
                tt.sleep_check();
                uev.send_event(Tick).unwrap();
            }
        });
    }
}

impl WindowHandler<Tick> for AntBoxWindow {
    fn on_user_event(&mut self, helper: &mut WindowHelper<Tick>, tick: Tick) {
        self.anim.update(tick);
        helper.request_redraw();
    }

    fn on_start(&mut self, helper: &mut WindowHelper<Tick>, info: WindowStartupInfo) {
        let viewsize = *info.viewport_size_pixels();
        let sfactor = info.scale_factor();
        log::info!("viewsize: {:?}, scaling factor: {:?}", viewsize, sfactor);

        assert!(!self.started);
        self.launch_tick_timer(helper);
        helper.request_redraw();
    }

    fn on_draw(&mut self, helper: &mut WindowHelper<Tick>, graphics: &mut Graphics2D) {
        let winsize = helper.get_size_pixels().into_f32();
        self.anim.draw(graphics, winsize);
    }

    fn on_key_down(
        &mut self,
        helper: &mut WindowHelper<Tick>,
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
