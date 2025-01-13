struct VSIn {
    @location(0) pos: vec2<f32>,
    @location(1) uv: vec2<f32>,

    // Per-instance
    @location(2) inst_pos: vec2<f32>,
    @location(3) inst_radius: f32,
    @location(4) inst_color: vec4<f32>,
};

struct VSOut {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) color: vec4<f32>,
    @location(2) center: vec2<f32>,
    @location(3) radius: f32,
};

@vertex
fn vs_main(in: VSIn) -> VSOut {
    var out: VSOut;
    // Combine quad pos with instance pos in 2D.
    out.position = vec4<f32>(in.pos + in.inst_pos, 0.0, 1.0);
    out.uv = in.uv;
    out.color = in.inst_color;
    out.center = in.inst_pos;
    out.radius = in.inst_radius;
    return out;
}

struct FSIn {
    @builtin(position) pos: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) color: vec4<f32>,
    @location(2) center: vec2<f32>,
    @location(3) radius: f32,
};

@fragment
fn fs_main(in: FSIn) -> @location(0) vec4<f32> {
//     Simple circle check in screen space (or local space).
    let dist = distance(in.uv, vec2<f32>(0.5, 0.5));


    if (dist > in.radius) {
        discard;
    }

    return in.color;
}