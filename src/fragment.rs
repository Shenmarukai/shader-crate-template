use glam::{Vec2, Vec4};
use spirv_std::spirv;

/// Fragment shader that uses UV coords passed in from the vertex shader
/// to render a simple gradient.
#[spirv( fragment )]
pub fn fragment( in_uv: Vec2, frag_color: &mut Vec4 ) {
    *frag_color = Vec4::new( in_uv.x, in_uv.y, 0.0, 1.0 );
}
