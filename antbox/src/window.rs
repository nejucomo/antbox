use antbox_animation::AnimationState;
use antbox_state::GenParams;
use antbox_tick_timer::TickTimer;
use derive_more::{IsVariant, Unwrap};
use movestate::Slot;
use rand::rngs::StdRng;
use speedy2d::window::{
    KeyScancode, VirtualKeyCode, WindowCreationOptions, WindowHandler, WindowHelper,
    WindowStartupInfo,
};
use speedy2d::{Graphics2D, Window};

use crate::{Result, TARGET_FRAME_RATE, Tick};

use self::RunMode::{Paused, Running};

/// # TODO
///
/// - Hide the states privately behind public interface
#[derive(Debug)]
pub struct AntBoxWindow {
    rng: StdRng,
    ws: Slot<WinState>,
}

#[derive(Debug, IsVariant, Unwrap)]
#[unwrap(ref, ref_mut)]
enum WinState {
    Starting(GenParams),
    Started(Started),
}

#[derive(Debug)]
struct Started {
    mode: RunMode,
    anim: Slot<AnimationState>,
}

#[derive(Copy, Clone, Debug)]
enum RunMode {
    Running,
    Paused,
}

impl RunMode {
    fn toggle(&mut self) {
        *self = match self {
            Running => Paused,
            Paused => Running,
        };
        log::info!("{self:?}");
    }
}

impl AntBoxWindow {
    pub fn run(rng: StdRng, gp: GenParams) -> Result<()> {
        let w = Window::new_with_user_events(
            env!("CARGO_PKG_NAME"),
            WindowCreationOptions::new_fullscreen_borderless(),
        )?;
        w.run_loop(AntBoxWindow {
            rng,
            ws: Slot::from(WinState::Starting(gp)),
        })
    }

    fn launch_tick_timer(&self, helper: &mut WindowHelper<Tick>) {
        assert!(self.ws.is_starting());

        let uev = helper.create_user_event_sender();
        std::thread::spawn(move || {
            let mut tt = TickTimer::with_frame_rate(TARGET_FRAME_RATE);

            loop {
                tt.sleep_update();
                uev.send_event(Tick).unwrap();
            }
        });
    }
}

impl WindowHandler<Tick> for AntBoxWindow {
    fn on_user_event(&mut self, helper: &mut WindowHelper<Tick>, _: Tick) {
        let st = self.ws.unwrap_started_mut();
        if matches!(st.mode, Running) {
            st.anim.update(&mut self.rng);
        }
        helper.request_redraw();
    }

    fn on_start(&mut self, helper: &mut WindowHelper<Tick>, _: WindowStartupInfo) {
        self.launch_tick_timer(helper);

        self.ws.map(|ws| {
            let gp = ws.unwrap_starting();

            WinState::Started(Started {
                mode: Running,
                anim: Slot::from(AnimationState::new(&mut self.rng, gp)),
            })
        });

        helper.request_redraw();
    }

    fn on_draw(&mut self, helper: &mut WindowHelper<Tick>, graphics: &mut Graphics2D) {
        let winsize = helper.get_size_pixels().into_f32();
        self.ws.unwrap_started_ref().anim.draw(graphics, winsize);
    }

    fn on_key_down(
        &mut self,
        _: &mut WindowHelper<Tick>,
        ovkc: Option<VirtualKeyCode>,
        _: KeyScancode,
    ) {
        use VirtualKeyCode::{Escape, Return, Space};

        match ovkc {
            Some(Escape) => {
                log::info!("bye!");
                std::process::exit(0);
            }
            Some(Space) => {
                self.ws.unwrap_started_mut().mode.toggle();
            }
            Some(Return) => {
                let st = self.ws.unwrap_started_mut();
                if matches!(st.mode, Paused) {
                    st.anim.update(&mut self.rng);
                }
            }
            _ => {
                // Ignore
            }
        }
    }
}
