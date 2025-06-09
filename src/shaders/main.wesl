import lygia::math::cubic::cubic;
import lygia::version::LYGIA_VERSION_MINOR;

@compute
fn main(@builtin(global_invocation_id) index: vec3u) -> f32 {
    return cubic(f32(index.x + LYGIA_VERSION_MINOR));
}