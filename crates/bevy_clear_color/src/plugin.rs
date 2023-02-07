use bevy_app::Plugin;
use bevy_render::extract_resource::ExtractResourcePlugin;

use super::{ClearColor, ClearColorConfig};

#[derive(Default)]
pub struct ClearColorPlugin;

impl Plugin for ClearColorPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        app.register_type::<ClearColor>()
            .init_resource::<ClearColor>()
            .add_plugin(ExtractResourcePlugin::<ClearColor>::default())
            .register_type::<ClearColorConfig>();
    }
}
