use antbox_animation::UpdateSource::{ClockTick, Step};
use antbox_animation::{AntboxAnimation, RunMode::Running, UpdateEvent};
use antbox_gameboard::GenParams;
use antbox_geom::Dimensions;
use antbox_render::{Backend, RenderRefWithArg, RenderWithArg as _};
use antbox_s2win::event::WinEvent;
use antbox_s2win::{Control, UserEventSender, WindowEventHandler, WindowExt as _};
use antbox_tick_timer::TickTimer;
use derive_debug::Dbg;
use moveslot::MoveSlot;
use mstate::{Responder, Update as _};
use speedy2d::Window;
use speedy2d::window::{WindowCreationOptions, WindowStartupInfo};

use crate::{Result, TARGET_FRAME_RATE, Tick};

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
    anim: MoveSlot<AntboxAnimation>,
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
        let anim = MoveSlot::from(AntboxAnimation::new(&mut rng, gp, Running));
        let winst = WinHandler { rng, anim };
        winst.launch_tick_timer(ues);
        winst
    }
}

impl<R> Responder<WinEvent<Tick>> for WinHandler<R>
where
    R: rand::Rng,
{
    type Response = Control;

    fn handle(&mut self, event: WinEvent<Tick>) -> Control {
        use Control::{Idle, RequestRedraw};
        use antbox_s2win::event::{
            ButtonPosition::Down,
            KeyInput::Virtual,
            WinEvent::{Key, User},
        };
        use speedy2d::window::VirtualKeyCode::{Escape, Return, Space};

        match event {
            User(Tick) => {
                self.anim.update(UpdateEvent::new(&mut self.rng, ClockTick));
                RequestRedraw
            }

            Key(Virtual(Down, Escape)) => {
                log::info!("bye!");
                std::process::exit(0);
            }

            Key(Virtual(Down, Space)) => {
                self.anim.runmode.toggle();
                Idle
            }

            Key(Virtual(Down, Return)) => {
                self.anim.update(UpdateEvent::new(&mut self.rng, Step));
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
