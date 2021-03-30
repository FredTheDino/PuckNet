use luminance_sdl2::{sdl2, GL33Surface};
use std::time::Instant;

mod renderer;
mod semantics;

fn main() {
    let surface = GL33Surface::build_with(|video| video.window("game", 800, 600))
        .expect("Failed to create surface");

    main_loop(surface);
}

fn main_loop(mut surface: GL33Surface) {
    use renderer::*;
    let mut sampler = luminance::texture::Sampler::default();
    sampler.mag_filter = luminance::texture::MagFilter::Nearest;
    let mut renderer = Renderer::new(&mut surface, sampler);

    let (w, h, image) = load_image_from_memory(include_bytes!("../res/coin.png")).unwrap();
    let builder = SpriteSheetBuilder::new(w as usize, h as usize, image).tile_size(16, 16);
    let sheet = renderer.add_sprite_sheet(builder);

    let mut particle_systems = ParticleSystem::new();
    particle_systems.lifetime = RandomProperty::new(1.0, 2.0);
    particle_systems.start_sx = RandomProperty::new(0.01, 0.015);
    particle_systems.start_sy = RandomProperty::new(0.01, 0.015);
    particle_systems.end_sx = RandomProperty::new(0.0, 0.0);
    particle_systems.end_sy = RandomProperty::new(0.0, 0.0);
    particle_systems.v_angle = RandomProperty::new(-std::f32::consts::PI, std::f32::consts::PI);
    particle_systems.v_magnitude = RandomProperty::new(-2.0, 2.0);
    particle_systems.acceleration_angle =
        RandomProperty::new(-std::f32::consts::PI, std::f32::consts::PI);
    particle_systems.acceleration_magnitude = RandomProperty::new(0.2, 0.8);
    particle_systems.angle = RandomProperty::new(-2.0, 2.0);
    particle_systems.angle_velocity = RandomProperty::new(-2.0, 2.0);
    particle_systems.angle_drag = RandomProperty::new(0.0, 2.0);

    let start_t = Instant::now();

    let mut old_t = start_t.elapsed().as_millis() as f32 * 1e-3;
    'app: loop {
        let t = start_t.elapsed().as_millis() as f32 * 1e-3;
        let delta = t - old_t;
        old_t = t;

        for event in surface.sdl().event_pump().unwrap().poll_iter() {
            use sdl2::event::{Event, WindowEvent};
            use sdl2::keyboard::Keycode;
            match event {
                Event::Quit { .. } => {
                    break 'app;
                }
                Event::Window {
                    win_event: WindowEvent::Close,
                    ..
                } => {
                    break 'app;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'app;
                }
                _ => {}
            }
        }

        particle_systems.position[0] = t.cos() * 0.5;
        particle_systems.position[1] = t.sin() * 0.5;
        for _ in 0..5 {
            particle_systems.spawn();
        }
        particle_systems.update(delta);

        renderer.push(
            Rect::new()
                .scale(0.3, 0.3)
                .at(-0.3, 0.0)
                .angle(t)
                .r(t.sin())
                .g(t.sin()),
        );

        renderer.push(
            Rect::new()
                .scale(0.2, 0.2)
                .at(0.3, 0.0)
                .angle(t)
                .r(t.cos())
                .g(t.cos()),
        );

        let region = sheet.grid([0, 1, 2, 3, 2, 1][((t * 10.0) as usize) % 6], 0);
        for x in -5..5 {
            for y in -5..5 {
                renderer.push(
                    Sprite::new(region)
                        .at(x as f32, y as f32)
                        .scale(0.3, 0.3)
                        .angle(t),
                );
            }
        }

        const NUM_BUCKETS: usize = 100;
        let mut buckets = [0; NUM_BUCKETS];
        for _ in 0..10000 {
            let sample = Distribution::Square.sample();
            buckets[(sample * (NUM_BUCKETS as f32)) as usize] += 1;
        }

        for (i, v) in buckets.iter().enumerate() {
            let w = 1.0 / (NUM_BUCKETS as f32);
            let h = (*v as f32) * w * 0.1;
            renderer.push(Rect::new().scale(w, h).at((i as f32) * w, h / 2.0));
        }

        renderer.push_particle_system(&particle_systems);

        renderer.camera.at(-0.5, -0.5);

        if renderer.render(&mut surface).is_err() {
            break 'app;
        }
    }
}
