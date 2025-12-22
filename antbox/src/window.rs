use antbox_animation::AnimationState;
use antbox_s2win::event::ButtonPosition;
use antbox_s2win::{WindowExt as _, WindowHandlerParams, WindowHandlerSimplified};
use antbox_state::GenParams;
use antbox_tick_timer::TickTimer;
use derive_debug::Dbg;
use derive_more::IsVariant;
use movestate::Slot;
use speedy2d::window::{VirtualKeyCode, WindowCreationOptions, WindowHelper, WindowStartupInfo};
use speedy2d::{Graphics2D, Window};

use crate::{Result, TARGET_FRAME_RATE, Tick};

use self::RunMode::{Paused, Running};

pub fn run<R>(rng: R, gp: GenParams) -> Result<()>
where
    R: rand::Rng + 'static,
{
    let w = Window::new_with_user_events(
        env!("CARGO_PKG_NAME"),
        WindowCreationOptions::new_fullscreen_borderless(),
    )?;

    w.run_loop_simplified(Params { rng, gp })
}

#[derive(Dbg)]
struct Params<R>
where
    R: rand::Rng,
{
    #[dbg(placeholder = "...")]
    rng: R,
    gp: GenParams,
}

#[derive(Dbg)]
struct Started<R>
where
    R: rand::Rng,
{
    #[dbg(placeholder = "...")]
    rng: R,
    mode: RunMode,
    anim: Slot<AnimationState>,
}

#[derive(Copy, Clone, Debug, IsVariant)]
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

impl<R> WindowHandlerParams<Tick> for Params<R>
where
    R: rand::Rng,
{
    type WHS = Started<R>;

    fn start_handler(mut self, helper: &mut WindowHelper<Tick>, _: WindowStartupInfo) -> Self::WHS {
        let anim = Slot::from(AnimationState::new(&mut self.rng, self.gp));
        let winst = Started {
            rng: self.rng,
            mode: Running,
            anim,
        };
        winst.launch_tick_timer(helper);
        helper.request_redraw();
        winst
    }
}

impl<R> Started<R>
where
    R: rand::Rng,
{
    fn launch_tick_timer(&self, helper: &mut WindowHelper<Tick>) {
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

impl<R> WindowHandlerSimplified<Tick> for Started<R>
where
    R: rand::Rng,
{
    fn on_user_event(&mut self, helper: &mut WindowHelper<Tick>, _: Tick) {
        if matches!(self.mode, Running) {
            self.anim.update(&mut self.rng);
        }
        helper.request_redraw();
    }

    fn on_draw(&mut self, helper: &mut WindowHelper<Tick>, graphics: &mut Graphics2D) {
        let winsize = helper.get_size_pixels().into_f32();
        self.anim.draw(graphics, winsize);
    }

    fn on_key(&mut self, _: &mut WindowHelper<Tick>, vkc: VirtualKeyCode, kpos: ButtonPosition) {
        use VirtualKeyCode::{Escape, Return, Space};

        if kpos.is_up() {
            match vkc {
                Escape => {
                    log::info!("bye!");
                    std::process::exit(0);
                }
                Space => {
                    self.mode.toggle();
                }
                Return => {
                    if self.mode.is_paused() {
                        self.anim.update(&mut self.rng);
                    }
                }
                _ => {
                    // Ignore
                }
            }
        }
    }
}
