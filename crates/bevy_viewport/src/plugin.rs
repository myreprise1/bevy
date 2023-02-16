use bevy_app::Plugin;

use super::Viewport;

pub struct ViewportPlugin;

impl Plugin for ViewportPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        app.register_type::<Viewport>()
            .register_type::<Option<Viewport>>();
    }
}
