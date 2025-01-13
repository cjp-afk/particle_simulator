// compute.wgsl

struct Particle {
    position: vec2<f32>,
    velocity: vec2<f32>,
    radius: f32,
    color: vec4<f32>,
};

@group(0) @binding(0)
var<storage, read_write> particles: array<Particle>;

// We'll dispatch with workgroup_size(64) as in your Rust code
@compute @workgroup_size(64)
fn main(@builtin(global_invocation_id) gid: vec3<u32>) {
    if (gid.x >= arrayLength(&particles)) {
        // If our invocation index is outside the range of total particles, we do nothing
        return;
    }

     var particle = particles[gid.x];

     particle.position += particle.velocity;

     let radius = 0.005;

     if (particle.position.y <= -1.0 + radius || particle.position.y >= 1.0 - radius) {
        particle.velocity.y = -particle.velocity.y;
     }
     if (particle.position.x <= -1.0 + radius || particle.position.x >= 1.0 - radius) {
             particle.velocity.x = -particle.velocity.x;
     }

     particles[gid.x] = particle;

}
