pub mod bloom;
pub mod core_2d;
pub mod core_3d;
pub mod fxaa;
pub mod tonemapping;
pub mod upscaling;

pub mod prelude {
    #[doc(hidden)]
    pub use crate::{
        core_2d::{Camera2d, Camera2dBundle},
        core_3d::{Camera3d, Camera3dBundle},
    };
}

use crate::{
    bloom::BloomPlugin, core_2d::Core2dPlugin, core_3d::Core3dPlugin, fxaa::FxaaPlugin,
    tonemapping::TonemappingPlugin, upscaling::UpscalingPlugin,
};
use bevy_app::{App, Plugin};

#[derive(Default)]
pub struct CorePipelinePlugin;

impl Plugin for CorePipelinePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(Core2dPlugin)
            .add_plugin(Core3dPlugin)
            .add_plugin(TonemappingPlugin)
            .add_plugin(UpscalingPlugin)
            .add_plugin(BloomPlugin)
            .add_plugin(FxaaPlugin);
    }
}
