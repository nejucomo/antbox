use rand::Rng;
use speedy2d::window::{
    KeyScancode, VirtualKeyCode, WindowHandler, WindowHelper, WindowStartupInfo,
};
use speedy2d::{Graphics2D, Window};

use crate::{AntBox, Result, colors};

use State::{Initialized, Ready};

// TODO: Make this private to this mod (to encapsulate graphics)
pub(crate) const CELL_LENGTH: u32 = 30;

/// # TODO
///
/// - Hide the states privately behind public interface
pub struct AntBoxWindow<R>(Option<State<R>>)
where
    R: Rng + 'static;

enum State<R>
where
    R: Rng + 'static,
{
    Ready { rng: R, cellprob: f64 },
    Initialized { antbox: AntBox<R> },
}

impl<R> AntBoxWindow<R>
where
    R: Rng + 'static,
{
    pub fn new(rng: R, cellprob: f64) -> Self {
        AntBoxWindow(Some(Ready { rng, cellprob }))
    }

    pub fn run(self) -> Result<()> {
        assert!(matches!(self.0.as_ref().unwrap(), Ready { .. }));

        let w = Window::<()>::new_fullscreen_borderless(env!("CARGO_PKG_NAME"))?;
        w.run_loop(self);
    }

    fn antbox_mut(&mut self) -> &mut AntBox<R> {
        match self.0.as_mut() {
            Some(Initialized { antbox }) => antbox,
            state => panic!("invalid state {state:#?}"),
        }
    }

    fn on_virtual_key_down(&mut self, helper: &mut WindowHelper<()>, vkc: VirtualKeyCode) {
        use VirtualKeyCode::Escape;

        match vkc {
            Escape => {
                log::info!("bye!");
                helper.terminate_loop();
            }
            _ => {
                // Ignore
            }
        }
    }
}

impl<R> WindowHandler<()> for AntBoxWindow<R>
where
    R: Rng + 'static,
{
    fn on_start(&mut self, helper: &mut WindowHelper<()>, info: WindowStartupInfo) {
        match self.0.take().unwrap() {
            Ready { rng, cellprob } => {
                let viewsize = *info.viewport_size_pixels();
                let sfactor = info.scale_factor();
                log::info!("viewsize: {:?}, scaling factor: {:?}", viewsize, sfactor);
                let antbox = AntBox::new(rng, cellprob, viewsize);
                self.0 = Some(Initialized { antbox });
                helper.request_redraw();
            }

            Initialized { antbox } => {
                panic!("multiple on_start events: {antbox:#?}");
            }
        }
    }

    fn on_draw(&mut self, _: &mut WindowHelper<()>, graphics: &mut Graphics2D) {
        let antbox = self.antbox_mut();
        let cl = CELL_LENGTH as f32;

        graphics.clear_screen(colors::BACKGROUND);

        for (pt, cell) in antbox.food_grid().iter() {
            if cell.is_alive() {
                graphics.draw_circle(
                    (
                        cl * (pt.x() as f32) + cl / 2.0,
                        cl * (pt.y() as f32) + cl / 2.0,
                    ),
                    cl / 2.0,
                    colors::FOOD,
                );
            }
        }
    }

    fn on_key_down(
        &mut self,
        helper: &mut WindowHelper<()>,
        ovkc: Option<VirtualKeyCode>,
        _: KeyScancode,
    ) {
        if let Some(keycode) = ovkc {
            self.on_virtual_key_down(helper, keycode)
        }
    }
}

impl<R> std::fmt::Debug for AntBoxWindow<R>
where
    R: Rng + 'static,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("AntBoxWindow").field(&self.0).finish()
    }
}

impl<R> std::fmt::Debug for State<R>
where
    R: Rng + 'static,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ready { rng: _, cellprob } => f
                .debug_struct("Ready")
                .field("rng", &"..")
                .field("cellprob", cellprob)
                .finish(),
            Self::Initialized { antbox } => f
                .debug_struct("Initialized")
                .field("antbox", antbox)
                .finish(),
        }
    }
}
