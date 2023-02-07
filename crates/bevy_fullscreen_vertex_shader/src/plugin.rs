use bevy_app::Plugin;
use bevy_asset::load_internal_asset;
use bevy_render::render_resource::Shader;

use crate::FULLSCREEN_SHADER_HANDLE;

#[derive(Default)]
pub struct FullscreenVertexShaderPlugin;

impl Plugin for FullscreenVertexShaderPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        load_internal_asset!(
            app,
            FULLSCREEN_SHADER_HANDLE,
            "fullscreen.wgsl",
            Shader::from_wgsl
        );
    }
}
