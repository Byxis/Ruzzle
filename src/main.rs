use raylib::prelude::*;

enum BlockType {
    Fix,
    Rotation,
    All,
}

fn main() {
    let screen_width = 800;
    let screen_height = 450;

    let (mut rl, thread) = raylib::init()
        .size(screen_width, screen_height)
        .title("Ruzzle")
        .build();

    let camera = Camera3D::perspective(
        Vector3::new(0.0, 10.0, 10.0),
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
        45.0,
    );

    rl.set_target_fps(60);

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::RAYWHITE);

        let cubes = vec![
            BlockPrefab::new(0.0, 0.0, 0.0, Color::RED),
            BlockPrefab::new(2.0, 0.0, 0.0, Color::BLUE),
            BlockPrefab::new(-2.0, 1.0, 0.0, Color::GREEN),
        ];

        {
            let mut d3d = d.begin_mode3D(camera);

            for cube in cubes.iter() {
                cube.draw(&mut d3d);
            }

            d3d.draw_grid(10, 1.0);
        }

        d.draw_text(
            "Welcome to the third dimension!",
            10,
            40,
            20,
            Color::DARKGRAY,
        );
        d.draw_fps(10, 10);
    }
}

// Block type avec sa fonction d'implementation
struct BlockPrefab {
    position: Vector3,
    size: Vector3,
    color: Color,
    block_type: BlockType,
}

impl BlockPrefab {
    fn new(x: f32, y: f32, z: f32, color: Color) -> Self {
        Self {
            position: Vector3::new(x, y, z),
            size: Vector3::new(1.0, 1.0, 1.0),
            color,
            block_type: BlockType::All,
        }
    }

    fn draw(&self, d: &mut RaylibMode3D<RaylibDrawHandle>) {
        d.draw_cube(
            self.position,
            self.size.x,
            self.size.y,
            self.size.z,
            self.color,
        );
        d.draw_cube_wires(
            self.position,
            self.size.x,
            self.size.y,
            self.size.z,
            Color::BLACK,
        );
    }
}
