mod enumstates;

use antbox_geom::{Bounds, Grid};
use antbox_s2win::event::WinEvent;
use antbox_s2win::{WindowEventHandler, WindowExt as _};
use antbox_state::{GenParams, Spot};
use derive_debug::Dbg;
use movestate::mutable::Update;
use rand::rngs::StdRng;
use speedy2d::Window;

use crate::Result;
use crate::inspect_render::enumstates::EnumerateRenderStates as _;

pub fn run(rng: StdRng, gp: GenParams) -> Result<()> {
    let w = Window::new_centered(
        &format!("{}-inspect-render", env!("CARGO_PKG_NAME")),
        (800, 600),
    )?;

    w.run_loop_simplified::<IRHandler>((rng, gp))
}

#[derive(Dbg)]
struct IRHandler {
    #[dbg(placeholder = "...")]
    rng: StdRng,
    grid: Grid<Spot>,
}

impl WindowEventHandler<()> for IRHandler {
    type Params = (StdRng, GenParams);

    fn start(
        (mut rng, gp): (StdRng, GenParams),
        helper: &mut speedy2d::window::WindowHelper<()>,
        _: speedy2d::window::WindowStartupInfo,
    ) -> Self {
        log::debug!("note: ignoring `--cell-prob {}`", gp.cell_prob);
        helper.request_redraw();
        let grid = setup_grid(&mut rng, gp.grid_size);
        IRHandler { rng, grid }
    }
}

impl<'a> Update<WinEvent<'a, ()>, ()> for IRHandler {
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

fn setup_grid(rng: &mut StdRng, grid_size: Bounds) -> Grid<Spot> {
    let mut spots = Vec::with_capacity(grid_size.area());
    for spot in enumerate_spot_render_states(rng) {
        spots.push(spot);
    }
    for _ in spots.len()..grid_size.area() {
        spots.push(Spot::default());
    }
    Grid::new(grid_size, spots)
}
