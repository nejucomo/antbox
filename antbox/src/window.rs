use antbox_animation::AnimationState;
use antbox_s2win::event::WinEvent;
use antbox_s2win::{WindowEventHandler, WindowExt as _};
use antbox_state::GenParams;
use antbox_tick_timer::TickTimer;
use derive_debug::Dbg;
use derive_more::IsVariant;
use movestate::{Transform, Update as _};
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
    anim: AnimationState,
}

#[derive(Copy, Clone, Debug, IsVariant)]
enum RunMode {
    Running,
    Paused,
}

impl RunMode {
    fn toggled(self) -> Self {
        let next = match self {
            Running => Paused,
            Paused => Running,
        };
        log::info!("{next:?}");
        next
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
        let anim = AnimationState::new(&mut rng, gp);
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

impl<'a, R> Transform<WinEvent<'a, Tick>> for WinHandler<R>
where
    R: rand::Rng,
{
    type Next = Self;

    fn transform(self, WinEvent { helper, info }: WinEvent<'a, Tick>) -> Self::Next {
        use antbox_s2win::event::{
            ButtonPosition::Down,
            Info::{DrawRequest, Input, User},
            Input::Key,
            KeyInput::Virtual,
        };
        use speedy2d::window::VirtualKeyCode::{Escape, Return, Space};

        match info {
            User(Tick) => {
                let WinHandler {
                    mut rng,
                    mode,
                    anim,
                } = self;

                let anim = match mode {
                    Running => anim.update(&mut rng),
                    Paused => anim,
                };

                helper.request_redraw();

                WinHandler { rng, mode, anim }
            }

            DrawRequest(gfx) => {
                let winsize = helper.get_size_pixels().into_f32();
                self.anim.draw(gfx, winsize);
                self
            }

            Input(Key(Virtual(Down, Escape))) => {
                log::info!("bye!");
                std::process::exit(0);
            }
            Input(Key(Virtual(Down, Space))) => WinHandler {
                mode: self.mode.toggled(),
                ..self
            },
            Input(Key(Virtual(Down, Return))) => {
                let WinHandler {
                    mut rng,
                    mode,
                    anim,
                } = self;

                let anim = if mode.is_paused() {
                    anim.update(&mut rng)
                } else {
                    anim
                };

                WinHandler { rng, mode, anim }
            }

            // Input(Resize(vector2)) => todo!(),
            // Input(FullscreenStatusChanged(_)) => todo!(),
            // Input(ScaleFactorChanged(_)) => todo!(),
            // Input(Mouse(mouse_event)) => todo!(),
            // Input(Unicode(_)) => todo!(),
            _ => {
                // Ignored
                self
            }
        }
    }
}
