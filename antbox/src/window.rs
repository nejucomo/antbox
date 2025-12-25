use antbox_animation::AnimationState;
use antbox_s2win::event::WinEvent;
use antbox_s2win::{WindowEventHandler, WindowExt as _};
use antbox_state::GenParams;
use antbox_tick_timer::TickTimer;
use derive_debug::Dbg;
use derive_more::IsVariant;
use moveslot::MoveSlot;
use movestate::mutable::Update;
use speedy2d::Window;
use speedy2d::window::{WindowCreationOptions, WindowHelper, WindowStartupInfo};

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

    w.run_loop_simplified::<WinHandler<R>>((rng, gp))
}

#[derive(Dbg)]
struct WinHandler<R>
where
    R: rand::Rng,
{
    #[dbg(placeholder = "...")]
    rng: R,
    mode: RunMode,
    anim: MoveSlot<AnimationState>,
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
impl<R> WinHandler<R>
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

impl<R> WindowEventHandler<Tick> for WinHandler<R>
where
    R: rand::Rng,
{
    type Params = (R, GenParams);

    fn start(
        (mut rng, gp): (R, GenParams),
        helper: &mut WindowHelper<Tick>,
        _: WindowStartupInfo,
    ) -> Self {
        let anim = MoveSlot::from(AnimationState::new(&mut rng, gp));
        let winst = WinHandler {
            rng,
            mode: Running,
            anim,
        };
        winst.launch_tick_timer(helper);
        helper.request_redraw();
        winst
    }
}

impl<'a, R> Update<WinEvent<'a, Tick>, ()> for WinHandler<R>
where
    R: rand::Rng,
{
    fn update(&mut self, WinEvent { helper, info }: WinEvent<'a, Tick>) {
        use antbox_s2win::event::{
            ButtonPosition::Down,
            Info::{DrawRequest, Input, User},
            Input::Key,
            KeyInput::Virtual,
        };
        use speedy2d::window::VirtualKeyCode::{Escape, Return, Space};

        match info {
            User(Tick) => {
                self.anim.update(&mut self.rng);
                helper.request_redraw();
            }

            DrawRequest(gfx) => {
                let winsize = helper.get_size_pixels().into_f32();
                self.anim.draw(gfx, winsize);
            }

            Input(Key(Virtual(Down, Escape))) => {
                log::info!("bye!");
                std::process::exit(0);
            }

            Input(Key(Virtual(Down, Space))) => {
                self.mode.toggle();
            }

            Input(Key(Virtual(Down, Return))) => {
                // TODO: Apply mode to only game state, allow animations to continue.
                if self.mode.is_paused() {
                    self.anim.update(&mut self.rng);
                };
            }

            // Input(Resize(vector2)) => todo!(),
            // Input(FullscreenStatusChanged(_)) => todo!(),
            // Input(ScaleFactorChanged(_)) => todo!(),
            // Input(Mouse(mouse_event)) => todo!(),
            // Input(Unicode(_)) => todo!(),
            _ => {
                // Ignored
            }
        }
    }
}
