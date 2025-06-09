import lygia::lygia::math::cubic;

@compute
fn main(@builtin(global_invocation_id) index: vec3u) -> f32 {
    return cubic(f32(index.x));
}