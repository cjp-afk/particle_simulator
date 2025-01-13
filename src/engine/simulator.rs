use crate::engine::*;
use rand;
use rand::Rng;

pub struct Simulator<'w> {
    pub(crate) renderer: renderer::Renderer<'w>,
}

impl<'w> Simulator<'w> {
    pub async fn new(window: &'w Window, num_particles: u32) -> Self {

        let mut particles = Vec::with_capacity(num_particles as usize);

        let mut rng = rand::thread_rng();

        for i in 0..num_particles {
            let x = rng.gen_range(-0.5..0.5);
            let y = rng.gen_range(-0.5..0.5);
            let vx = rng.gen_range(-0.002..0.002);
            let vy = rng.gen_range(-0.002..0.002);
            let r = rng.gen_range(0.0..1.0);
            let g = rng.gen_range(0.0..1.0);
            let b = rng.gen_range(0.0..1.0);
            let p = particle::Particle {
                position: [x, y],
                velocity: [vx, vy],
                radius: 0.05,
                _pad0: [0.0, 0.0, 0.0],
                color: [r, g, b, 1.0], // will be overridden by compute
            };
            particles.push(p);
        }

        let renderer = renderer::Renderer::new(&window, particles).await;

        Self {
            renderer,
        }
    }


}