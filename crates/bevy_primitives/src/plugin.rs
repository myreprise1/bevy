use super::{Aabb, CascadesFrusta, CubemapFrusta, Frustum};

use bevy_app::{App, Plugin};

#[derive(Default)]
pub struct PrimitivesPlugin;

impl Plugin for PrimitivesPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Aabb>()
            .register_type::<CascadesFrusta>()
            .register_type::<CubemapFrusta>()
            .register_type::<Frustum>();
    }
}
