use extension_traits::extension;
use speedy2d::Window;

use crate::WindowHandlerParams;
use crate::adapter::HandlerAdapter;

#[extension(pub trait WindowExt)]
impl<U> Window<U> {
    fn run_loop_simplified<P>(self, param: P) -> !
    where
        P: WindowHandlerParams<U> + 'static,
    {
        self.run_loop(HandlerAdapter::from(param))
    }
}
