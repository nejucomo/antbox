mod enumstates;

use antbox_geom::{Bounds, Grid};
use antbox_s2win::event::WinEvent;
use antbox_s2win::{WindowEventHandler, WindowExt as _};
use antbox_state::{GenParams, Spot};
use derive_debug::Dbg;
use movestate::mutable::Update;
use speedy2d::Window;

use crate::Result;

pub fn run<R>(rng: R, gp: GenParams) -> Result<()>
where
    R: rand::Rng + 'static,
{
    let w = Window::new_centered(
        format!("{}-inspect-render", env!("CARGO_PKG_NAME")),
        (800, 600),
    )?;

    w.run_loop_simplified::<IRHandler<R>>((rng, gp))
}

#[derive(Dbg)]
struct IRHandler<R>
where
    R: rand::Rng + 'static,
{
    #[dbg(placeholder = "...")]
    #[allow(dead_code)]
    rng: R,
    grid: Grid<Spot>,
}

impl<R> WindowEventHandler<()> for IRHandler<R>
where
    R: rand::Rng + 'static,
{
    type Params = (R, GenParams);

    fn start(
        (mut rng, gp): (R, GenParams),
        helper: &mut speedy2d::window::WindowHelper<()>,
        _: speedy2d::window::WindowStartupInfo,
    ) -> Self {
        log::debug!("note: ignoring `--cell-prob {}`", gp.cell_prob);
        helper.request_redraw();
        let grid = setup_grid(&mut rng, gp.grid_size);
        IRHandler { rng, grid }
    }
}

impl<'a, R> Update<WinEvent<'a, ()>, ()> for IRHandler<R>
where
    R: rand::Rng + 'static,
{
    fn update(&mut self, WinEvent { helper: _, info }: WinEvent<'a, ()>) {
        use antbox_s2win::event::{
            ButtonPosition::Down, Info::Input, Input::Key, KeyInput::Virtual,
        };
        use speedy2d::window::VirtualKeyCode::Escape;

        match info {
            Input(Key(Virtual(Down, Escape))) => {
                log::info!("bye!");
                std::process::exit(0);
            }

            _ => {
                // Ignore
            }
        }
    }
}

fn setup_grid<R>(rng: &mut R, grid_size: Bounds) -> Grid<Spot>
where
    R: rand::Rng + 'static,
{
    let mut spots = Vec::with_capacity(grid_size.area());
    for spot in enumstates::enumerate_spot_render_states(rng) {
        spots.push(spot);
    }
    for _ in spots.len()..grid_size.area() {
        spots.push(Spot::default());
    }
    Grid::new(grid_size, spots)
}
