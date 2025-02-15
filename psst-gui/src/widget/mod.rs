mod dispatcher;
mod empty;
pub mod icons;
mod link;
mod maybe;
mod promise;
pub mod remote_image;
mod theme;
mod utils;

use std::sync::Arc;

use druid::{
    widget::ControllerHost, Data, Env, EventCtx, Menu, MouseButton, MouseEvent, Selector, Widget,
};

pub use dispatcher::ViewDispatcher;
pub use empty::Empty;
pub use icons::Icon;
pub use link::Link;
pub use maybe::Maybe;
pub use promise::Async;
pub use remote_image::RemoteImage;
pub use theme::ThemeScope;
pub use utils::{Border, Clip, Logger};

use crate::{
    controller::{ExClick, OnCmdAsync},
    data::AppState,
};

pub trait MyWidgetExt<T: Data>: Widget<T> + Sized + 'static {
    fn link(self) -> Link<T> {
        Link::new(self)
    }

    fn clip<S>(self, shape: S) -> Clip<S, Self> {
        Clip::new(shape, self)
    }

    fn on_right_click(
        self,
        func: impl Fn(&mut EventCtx, &MouseEvent, &mut T, &Env) + 'static,
    ) -> ControllerHost<Self, ExClick<T>> {
        ControllerHost::new(self, ExClick::new(Some(MouseButton::Right), func))
    }

    fn on_cmd_async<U: Data + Send, V: Data + Send>(
        self,
        selector: Selector<U>,
        request: impl Fn(U) -> V + Sync + Send + 'static,
        preflight: impl Fn(&mut EventCtx, &mut T, U) + 'static,
        response: impl Fn(&mut EventCtx, &mut T, (U, V)) + 'static,
    ) -> OnCmdAsync<Self, T, U, V> {
        OnCmdAsync::new(
            self,
            selector,
            Box::new(preflight),
            Arc::new(request),
            Box::new(response),
        )
    }

    fn context_menu(
        self,
        func: impl Fn(&T) -> Menu<AppState> + 'static,
    ) -> ControllerHost<Self, ExClick<T>> {
        self.on_right_click(move |ctx, event, data, _env| {
            ctx.show_context_menu(func(data), event.window_pos);
        })
    }
}

impl<T: Data, W: Widget<T> + 'static> MyWidgetExt<T> for W {}
