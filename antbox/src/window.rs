use antbox_animation::AnimationState;
use antbox_gameboard::GenParams;
use antbox_geom::Dimensions;
use antbox_render::{Backend, RenderRefWithArg, RenderWithArg as _};
use antbox_s2win::event::WinEvent;
use antbox_s2win::{Control, UserEventSender, WindowEventHandler, WindowExt as _};
use antbox_tick_timer::TickTimer;
use derive_debug::Dbg;
use derive_more::IsVariant;
use moveslot::MoveSlot;
use mstate::mutable::Update;
use speedy2d::Window;
use speedy2d::window::{WindowCreationOptions, WindowStartupInfo};

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
    fn launch_tick_timer(&self, ues: UserEventSender<Tick>) {
        std::thread::spawn(move || {
            let mut tt = TickTimer::with_frame_rate(TARGET_FRAME_RATE);

            loop {
                tt.sleep_update();
                ues.send_event(Tick).unwrap();
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
        ues: UserEventSender<Tick>,
        _: WindowStartupInfo,
    ) -> Self {
        let anim = MoveSlot::from(AnimationState::new(&mut rng, gp));
        let winst = WinHandler {
            rng,
            mode: Running,
            anim,
        };
        winst.launch_tick_timer(ues);
        winst
    }
}

impl<R> Update<WinEvent<Tick>, Control> for WinHandler<R>
where
    R: rand::Rng,
{
    fn update(&mut self, event: WinEvent<Tick>) -> Control {
        use Control::{Idle, RequestRedraw};
        use antbox_s2win::event::{
            ButtonPosition::Down,
            KeyInput::Virtual,
            WinEvent::{Key, User},
        };
        use speedy2d::window::VirtualKeyCode::{Escape, Return, Space};

        match event {
            User(Tick) => {
                self.anim.update(&mut self.rng);
                RequestRedraw
            }

            Key(Virtual(Down, Escape)) => {
                log::info!("bye!");
                std::process::exit(0);
            }

            Key(Virtual(Down, Space)) => {
                self.mode.toggle();
                Idle
            }

            Key(Virtual(Down, Return)) => {
                // TODO: Apply mode to only game state, allow animations to continue.
                if self.mode.is_paused() {
                    self.anim.update(&mut self.rng);
                };
                RequestRedraw
            }

            // Resize(vector2) => todo!(),
            // FullscreenStatusChanged(_) => todo!(),
            // ScaleFactorChanged(_) => todo!(),
            // Mouse(mouse_event) => todo!(),
            // Unicode(_) => todo!(),
            _ => {
                // Ignored
                Idle
            }
        }
    }
}

impl<R> RenderRefWithArg<Dimensions> for WinHandler<R>
where
    R: rand::Rng,
{
    fn render_ref_with_arg<B: ?Sized + Backend>(&self, rb: &mut B, dims: Dimensions) {
        self.anim.render_with_arg(rb, dims);
    }
}
