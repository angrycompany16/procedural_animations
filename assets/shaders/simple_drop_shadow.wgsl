#import bevy_sprite::mesh2d_vertex_output::VertexOutput
#import bevy_pbr::{
    mesh_view_bindings::view,
    utils::coords_to_viewport_uv,
}

// we can import items from shader modules in the assets folder with a quoted path

// Inputs
// - background color
// - shadow color
// - shadow offset distance



@group(1) @binding(0) var<uniform> background_color: vec4<f32>;
@group(1) @binding(1) var<uniform> shadow_color: vec4<f32>;
@group(1) @binding(2) var screen_texture: texture_2d<f32>;
@group(1) @binding(3) var screen_texture_sampler: sampler;
@group(1) @binding(4) var<uniform> shadow_offset: vec2<f32>;

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    let viewport_uv = coords_to_viewport_uv(mesh.position.xy, view.viewport);
    let current_color = textureSample(screen_texture, screen_texture_sampler, viewport_uv);

    let offset_color = textureSample(screen_texture, screen_texture_sampler, viewport_uv + shadow_offset);

    if (length(current_color - background_color) < 0.01) {

        if (length(offset_color - background_color) > 0.01) {
            return shadow_color;
        }
    }
    return current_color;
}