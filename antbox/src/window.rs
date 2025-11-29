use cellauto::Generation;
use speedy2d::window::{
    KeyScancode, VirtualKeyCode, WindowCreationOptions, WindowHandler, WindowHelper,
    WindowStartupInfo,
};
use speedy2d::{Graphics2D, Window};

use crate::{AntBox, Result, colors};

// TODO: Make this private to this mod (to encapsulate graphics)
pub(crate) const CELL_LENGTH: u32 = 30;

use State::{Active, Pending};

/// # TODO
///
/// - Hide the states privately behind public interface
#[derive(Debug)]
pub struct AntBoxWindow(State);

#[derive(Debug)]
enum State {
    Pending { rngseed: u64, cellprob: f64 },
    Active { foodgrid: Option<Generation> },
}

impl AntBoxWindow {
    pub fn new(rngseed: u64, cellprob: f64) -> Self {
        AntBoxWindow(Pending { rngseed, cellprob })
    }

    pub fn run(self) -> Result<()> {
        assert!(matches!(&self.0, Pending { .. }));

        let w = Window::new_with_user_events(
            env!("CARGO_PKG_NAME"),
            WindowCreationOptions::new_fullscreen_borderless(),
        )?;
        w.run_loop(self);
    }

    fn pop_food_grid(&mut self) -> Option<Generation> {
        match &mut self.0 {
            Active { foodgrid } => foodgrid.take(),
            Pending { .. } => panic!("pop_food_grid() on `Pending` state"),
        }
    }

    fn on_virtual_key_down(&mut self, helper: &mut WindowHelper<Generation>, vkc: VirtualKeyCode) {
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

impl WindowHandler<Generation> for AntBoxWindow {
    fn on_user_event(&mut self, helper: &mut WindowHelper<Generation>, nextgen: Generation) {
        log::debug!("received new foodgrid");
        match &mut self.0 {
            Pending { .. } => panic!("on_user_event() invalid state `Pending`"),
            Active { foodgrid } => {
                *foodgrid = Some(nextgen);
                helper.request_redraw();
            }
        }
    }

    fn on_start(&mut self, helper: &mut WindowHelper<Generation>, info: WindowStartupInfo) {
        let nextstate = match &self.0 {
            &Pending { rngseed, cellprob } => {
                let viewsize = *info.viewport_size_pixels();
                let w = viewsize.x / CELL_LENGTH;
                let h = viewsize.y / CELL_LENGTH;
                let sfactor = info.scale_factor();
                log::info!("viewsize: {:?}, scaling factor: {:?}", viewsize, sfactor);

                let sender = helper.create_user_event_sender();
                std::thread::spawn(move || AntBox::new(rngseed, cellprob, (w, h), sender).run());

                helper.request_redraw();

                Active { foodgrid: None }
            }

            Active { .. } => panic!("on_start on `Active` state"),
        };

        self.0 = nextstate;
    }

    fn on_draw(&mut self, _: &mut WindowHelper<Generation>, graphics: &mut Graphics2D) {
        let cl = CELL_LENGTH as f32;

        graphics.clear_screen(colors::BACKGROUND);

        if let Some(foodgrid) = self.pop_food_grid() {
            log::debug!("drawing popped food grid");
            for (pt, cell) in foodgrid.iter() {
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
        } else {
            log::warn!("No food grid!");
        }
    }

    fn on_key_down(
        &mut self,
        helper: &mut WindowHelper<Generation>,
        ovkc: Option<VirtualKeyCode>,
        _: KeyScancode,
    ) {
        if let Some(keycode) = ovkc {
            self.on_virtual_key_down(helper, keycode)
        }
    }
}
