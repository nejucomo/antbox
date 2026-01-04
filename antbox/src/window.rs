use antbox_animation::UpdateSource::{ClockTick, Step};
use antbox_animation::{Animator, RunMode::Running};
use antbox_gameboard::GenParams;
use antbox_geom::Dimensions;
use antbox_render::{Backend, RenderRefWithArg, RenderWithArg as _};
use antbox_s2win::event::{InitEvent, WinEvent};
use antbox_s2win::{Control, S2App};
use antbox_tick_timer::TickTimer;
use mstate::Responder;

use crate::{Result, TARGET_FRAME_RATE, Tick};

pub fn run<R>(rng: R, gp: GenParams) -> Result<()>
where
    R: rand::Rng + 'static,
{
    WinHandler::launch((rng, gp))
}

#[derive(Debug)]
struct WinHandler<R: rand::Rng>(Animator<R>);

impl<R> S2App for WinHandler<R>
where
    R: 'static + rand::Rng,
{
    const APP_NAME: &'static str = env!("CARGO_PKG_NAME");

    type Params = (R, GenParams);
    type Event = Tick;
}

impl<R> From<InitEvent<(R, GenParams), Tick>> for WinHandler<R>
where
    R: rand::Rng,
{
    fn from(init: InitEvent<(R, GenParams), Tick>) -> Self {
        let InitEvent {
            params: (rng, gp),
            ues,
            info: _,
        } = init;

        std::thread::spawn(move || {
            let mut tt = TickTimer::with_frame_rate(TARGET_FRAME_RATE);

            loop {
                tt.sleep_update();
                ues.send_event(Tick).unwrap();
            }
        });

        WinHandler(Animator::new(rng, gp, Running))
    }
}

impl<R> Responder<Tick> for WinHandler<R>
where
    R: rand::Rng,
{
    type Response = Control;

    fn handle(&mut self, _: Tick) -> Self::Response {
        self.0.update(ClockTick);
        Control::RequestRedraw
    }
}

impl<R> Responder<WinEvent> for WinHandler<R>
where
    R: rand::Rng,
{
    type Response = Control;

    fn handle(&mut self, event: WinEvent) -> Control {
        use Control::{Idle, RequestRedraw};
        use antbox_s2win::event::{ButtonPosition::Down, KeyInput::Virtual, WinEvent::Key};
        use speedy2d::window::VirtualKeyCode::{Escape, Return, Space};

        match event {
            Key(Virtual(Down, Escape)) => {
                log::info!("bye!");
                std::process::exit(0);
            }

            Key(Virtual(Down, Space)) => {
                self.0.toggle_run_mode();
                Idle
            }

            Key(Virtual(Down, Return)) => {
                self.0.update(Step);
                RequestRedraw
            }

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
        self.0.render_with_arg(rb, dims);
    }
}
