use std::fmt::Debug;

use extension_traits::extension;
use speedy2d::Window;

use crate::WindowEventHandler;
use crate::adapter::HandlerAdapter;

/// Extend [Window] to run a loop passing only pre-start parameters
#[extension(pub trait WindowExt)]
impl<U> Window<U>
where
    U: Debug,
{
    /// Run the window loop with the given [WindowEventHandler::Params]
    fn run_loop_simplified<H>(self, params: H::Params) -> !
    where
        H: WindowEventHandler<U> + 'static,
    {
        self.run_loop(HandlerAdapter::<H, U>::new(params))
    }
}
