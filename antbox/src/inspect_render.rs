use antbox_animation::{GridLayout, WyrGrid, layers, spots_into_renderable};
use antbox_gameboard::{GenParams, Spot};
use antbox_geom::Dimensions;
use antbox_grid::{Bounds, Grid};
use antbox_render::{Backend, RenderRefWithArg, RenderWithArg as _, Renderable as _};
use antbox_s2win::event::WinEvent;
use antbox_s2win::{Control, UserEventSender, WindowEventHandler, WindowExt as _};
use derive_debug::Dbg;
use movestate::mutable::Update;
use speedy2d::Window;

use crate::Result;

pub fn run<R>(rng: R, gp: GenParams) -> Result<()>
where
    R: rand::Rng + 'static,
{
    let w =
        Window::new_fullscreen_borderless(format!("{}-inspect-render", env!("CARGO_PKG_NAME")))?;

    w.run_loop_simplified::<IRHandler<R>>((rng, gp))
}

#[derive(Dbg)]
struct IRHandler<R>
where
    R: rand::Rng + 'static,
{
    #[dbg(placeholder = "...")]
    rng: R,
    grid: Grid<Spot>,
    wyrgrid: WyrGrid,
}

impl<R> WindowEventHandler<()> for IRHandler<R>
where
    R: rand::Rng + 'static,
{
    type Params = (R, GenParams);

    fn start(
        (mut rng, gp): (R, GenParams),
        _: UserEventSender<()>,
        _: speedy2d::window::WindowStartupInfo,
    ) -> Self {
        log::debug!("note: ignoring `--cell-prob {}`", gp.cell_prob);
        let grid = setup_grid(&mut rng, gp.grid_size);
        let wyrgrid = WyrGrid::new(grid.bounds(), &mut rng);
        IRHandler { rng, grid, wyrgrid }
    }
}

impl<R> Update<WinEvent<()>, Control> for IRHandler<R>
where
    R: rand::Rng + 'static,
{
    fn update(&mut self, event: WinEvent<()>) -> Control {
        use Control::{Idle, RequestRedraw};
        use antbox_s2win::event::{ButtonPosition::Down, KeyInput::Virtual, WinEvent::Key};
        use speedy2d::window::VirtualKeyCode::{Escape, Space};

        match event {
            Key(Virtual(Down, Escape)) => {
                log::info!("bye!");
                std::process::exit(0);
            }

            Key(Virtual(Down, Space)) => {
                self.wyrgrid = WyrGrid::new(self.grid.bounds(), &mut self.rng);
                RequestRedraw
            }

            _ => {
                // Ignore
                Idle
            }
        }
    }
}

impl<R> RenderRefWithArg<Dimensions> for IRHandler<R>
where
    R: rand::Rng + 'static,
{
    fn render_ref_with_arg<B: ?Sized + Backend>(&self, rb: &mut B, view_size: Dimensions) {
        let layout = GridLayout::new(self.grid.bounds(), view_size);

        (
            layers::Background,
            spots_into_renderable(&self.grid, layout, &self.wyrgrid),
            layers::WireFrame.with_render_arg(layout),
        )
            .render_to(rb);
    }
}

fn setup_grid<R>(rng: &mut R, grid_size: Bounds) -> Grid<Spot>
where
    R: rand::Rng + 'static,
{
    let mut spots = Vec::with_capacity(grid_size.area());
    spots.extend(Spot::interesting_values(rng));
    log::info!("Rendering {} spot states...", spots.len());

    for _ in spots.len()..grid_size.area() {
        spots.push(Spot::default());
    }
    Grid::new(grid_size, spots)
}
