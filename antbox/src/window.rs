use antbox_cellauto::Generation;
use antbox_engine::Notification;
use antbox_state::GenParams;
use speedy2d::window::{
    KeyScancode, VirtualKeyCode, WindowCreationOptions, WindowHandler, WindowHelper,
    WindowStartupInfo,
};
use speedy2d::{Graphics2D, Window};

use crate::notifier::SpeedyNotifier;
use crate::{Result, colors};

use State::{Active, Pending};

/// # TODO
///
/// - Hide the states privately behind public interface
#[derive(Debug)]
pub struct AntBoxWindow(State);

#[derive(Debug)]
enum State {
    Pending(GenParams),
    Active { foodgrid: Option<Generation> },
}

impl AntBoxWindow {
    pub fn new(genparams: GenParams) -> Self {
        AntBoxWindow(Pending(genparams))
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

    fn on_virtual_key_down(
        &mut self,
        helper: &mut WindowHelper<Notification>,
        vkc: VirtualKeyCode,
    ) {
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

impl WindowHandler<Notification> for AntBoxWindow {
    fn on_user_event(&mut self, helper: &mut WindowHelper<Notification>, notif: Notification) {
        use Notification::NewState;

        match &mut self.0 {
            Pending { .. } => panic!("on_user_event() invalid state `Pending`"),
            Active { foodgrid } => match notif {
                NewState(ns) => {
                    log::debug!("Received new antbox state: {:?}", ns.gencnt);
                    *foodgrid = Some(ns.food.into());
                    helper.request_redraw();
                }
            },
        }
    }

    fn on_start(&mut self, helper: &mut WindowHelper<Notification>, info: WindowStartupInfo) {
        let viewsize = *info.viewport_size_pixels();
        let sfactor = info.scale_factor();
        log::info!("viewsize: {:?}, scaling factor: {:?}", viewsize, sfactor);

        let nextstate = match &self.0 {
            &Pending(gp) => {
                let notifier = SpeedyNotifier::from(helper.create_user_event_sender());
                antbox_engine::spawn(gp, notifier);

                helper.request_redraw();
                Active { foodgrid: None }
            }

            Active { .. } => panic!("on_start on `Active` state"),
        };

        self.0 = nextstate;
    }

    fn on_draw(&mut self, helper: &mut WindowHelper<Notification>, graphics: &mut Graphics2D) {
        graphics.clear_screen(colors::BACKGROUND);

        if let Some(foodgrid) = self.pop_food_grid() {
            let winsize = helper.get_size_pixels().into_f32();
            let bounds = foodgrid.bounds();
            let w32 = bounds.width as f32;
            let h32 = bounds.height as f32;
            let cell_width = winsize.x / w32;
            let cell_height = winsize.y / h32;

            log::debug!("drawing popped food grid");
            for (pt, cell) in foodgrid.iter() {
                if cell.is_alive() {
                    graphics.draw_circle(
                        (
                            cell_width * (pt.x() as f32) + cell_width / 2.0,
                            cell_height * (pt.y() as f32) + cell_height / 2.0,
                        ),
                        cell_width.min(cell_height) / 2.0,
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
        helper: &mut WindowHelper<Notification>,
        ovkc: Option<VirtualKeyCode>,
        _: KeyScancode,
    ) {
        if let Some(keycode) = ovkc {
            self.on_virtual_key_down(helper, keycode)
        }
    }
}
