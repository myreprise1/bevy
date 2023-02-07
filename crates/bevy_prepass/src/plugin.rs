use bevy_app::{App, Plugin};

use super::{DepthPrepass, NormalPrepass};

#[derive(Default)]
pub struct PrepassPlugin;

impl Plugin for PrepassPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<DepthPrepass>()
            .register_type::<NormalPrepass>();
    }
}
