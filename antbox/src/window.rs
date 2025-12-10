use antbox_animation::AnimationState;
use antbox_state::GenParams;
use antbox_tick_timer::TickTimer;
use mealy_machine::Slot;
use rand::rngs::StdRng;
use speedy2d::window::{
    KeyScancode, VirtualKeyCode, WindowCreationOptions, WindowHandler, WindowHelper,
    WindowStartupInfo,
};
use speedy2d::{Graphics2D, Window};

use crate::{Result, Tick};

/// # TODO
///
/// - Hide the states privately behind public interface
#[derive(Debug)]
pub struct AntBoxWindow {
    rng: StdRng,
    started: bool,
    anim: Slot<AnimationState>,
}

impl AntBoxWindow {
    pub fn run(rng: StdRng, gp: GenParams) -> Result<()> {
        let w = Window::new_with_user_events(
            env!("CARGO_PKG_NAME"),
            WindowCreationOptions::new_fullscreen_borderless(),
        )?;
        w.run_loop(Self::new(rng, gp));
    }

    fn new(mut rng: StdRng, gp: GenParams) -> Self {
        let anim = Slot::from(AnimationState::new(&mut rng, gp));
        AntBoxWindow {
            rng,
            started: false,
            anim,
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
    fn on_user_event(&mut self, helper: &mut WindowHelper<Tick>, _: Tick) {
        self.anim.update_io(&mut self.rng);
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
        _: &mut WindowHelper<Tick>,
        ovkc: Option<VirtualKeyCode>,
        _: KeyScancode,
    ) {
        use VirtualKeyCode::Escape;

        match ovkc {
            Some(Escape) => {
                log::info!("bye!");
                std::process::exit(0);
            }
            _ => {
                // Ignore
            }
        }
    }
}
