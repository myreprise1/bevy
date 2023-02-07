Run a prepass before the main pass to generate depth and/or normals texture, sometimes called a thin g-buffer.
These textures are useful for various screen-space effects and reducing overdraw in the main pass.

The prepass only runs for opaque meshes or meshes with an alpha mask. Transparent meshes are ignored.

To enable the prepass, you need to add a prepass component to a `Camera3d`.

[`DepthPrepass`]
[`NormalPrepass`]

The textures are automatically added to the default mesh view bindings. You can also get the raw textures
by querying the [`ViewPrepassTextures`] component on any camera with a prepass component.

The depth prepass will always run and generate the depth buffer as a side effect, but it won't copy it
to a separate texture unless the [`DepthPrepass`] is activated. This means that if any prepass component is present
it will always create a depth buffer that will be used by the main pass.

When using the default mesh view bindings you should be able to use `prepass_depth()`
and `prepass_normal()` to load the related textures. These functions are defined in `bevy_pbr::prepass_utils`.
See the `shader_prepass` example that shows how to use it.

The prepass runs for each `Material`. You can control if the prepass should run per-material by setting the `prepass_enabled`
flag on the `MaterialPlugin`.

Currently only works for 3D.