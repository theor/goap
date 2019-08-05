#[macro_use]
extern crate bitflags;
extern crate pathfinding;

use cgmath;

use ggez;
use ggez::event;
use ggez::graphics;
use ggez::graphics::{Color, DrawMode, DrawParam};
use ggez::nalgebra::Point2;
use ggez::timer;
use ggez::{Context, GameResult};
use std::env;
use std::path;

mod goap;
use goap::StateFlag;

type Pos = Point2<f32>;



#[derive(Debug)]
struct Char {
    pos: Pos,
    has_axe: bool,
    wood: u8,
}

#[derive(Debug)]
enum ItemType {
    Tree,
    Branch,
    Axe,
}

#[derive(Debug)]
struct Item {
    pos: Pos,
    item: ItemType,
}

impl Item {
    pub fn new(item: ItemType, x: f32, y: f32) -> Self {
        Self {
            pos: Pos::new(x, y),
            item: item,
        }
    }
}

impl Char {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            pos: Pos::new(x, y),
            has_axe: false,
            wood: 0,
        }
    }
    pub fn to_state(&self) -> StateFlag {
        let mut s = StateFlag::NONE;
        if self.has_axe { s |= StateFlag::HAS_AXE; }
        if self.wood > 0 { s |= StateFlag::HAS_WOOD; }
        s
    }
}



struct MainState {
    chars: Vec<Char>,
    items: Vec<Item>,
    // meshes: Vec<graphics::Mesh>,
    rotation: f32,
}

impl MainState {
    /// Load images and create meshes.
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        // let meshes = vec![build_mesh(ctx)?];//, build_textured_triangle(ctx)?];
        let s = MainState {
            // meshes,
            chars: vec![
                Char::new(100.0, 100.0),
                Char::new(400.0, 100.0),
            ],
            items: vec![
                Item::new(ItemType::Axe, 300.0, 300.0),
                Item::new(ItemType::Tree, 100.0, 300.0),
                Item::new(ItemType::Branch, 500.0, 300.0),
            ],
            rotation: 1.0,
        };

        Ok(s)
    }
}

// fn build_mesh(ctx: &mut Context) -> GameResult<graphics::Mesh> {
//     let mb = &mut graphics::MeshBuilder::new();

//     mb.line(
//         &[
//             Point2::new(200.0, 200.0),
//             Point2::new(400.0, 200.0),
//             Point2::new(400.0, 400.0),
//             Point2::new(200.0, 400.0),
//             Point2::new(200.0, 300.0),
//         ],
//         4.0,
//         Color::new(1.0, 0.0, 0.0, 1.0),
//     )?;

//     mb.ellipse(
//         DrawMode::fill(),
//         Point2::new(600.0, 200.0),
//         50.0,
//         120.0,
//         1.0,
//         Color::new(1.0, 1.0, 0.0, 1.0),
//     );

//     mb.circle(
//         DrawMode::fill(),
//         Point2::new(600.0, 380.0),
//         40.0,
//         1.0,
//         Color::new(1.0, 0.0, 1.0, 1.0),
//     );

//     mb.build(ctx)
// }

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        const DESIRED_FPS: u32 = 60;

        while timer::check_update_time(ctx, DESIRED_FPS) {
            self.rotation += 0.01;
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        for ch in self.chars.iter() {
            // Create and draw a filled rectangle mesh.
            const W:f32 = 50.0;
            const H:f32 = 50.0;
            let rect = graphics::Rect::new(ch.pos.x - W/2.0, ch.pos.y - H/2.0, W, H);
            let r1 =
                graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), rect, graphics::WHITE)?;
            graphics::draw(ctx, &r1, DrawParam::default())?;

        }

        for ch in self.items.iter() {
            // Create and draw a filled rectangle mesh.
            const W:f32 = 50.0;
            const H:f32 = 50.0;
            let rect = graphics::Rect::new(ch.pos.x - W/2.0, ch.pos.y - H/2.0, W, H);
            let color = match ch.item {
                ItemType::Axe => graphics::BLACK,
                ItemType::Tree => graphics::Color::from_rgb(0, 145, 5),
                ItemType::Branch => graphics::Color::from_rgb(145, 107, 0),
            };
            let r1 =
                graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), rect, color)?;
            graphics::draw(ctx, &r1, DrawParam::default())?;

        }


        // // Create and draw a stroked rectangle mesh.
        // let rect = graphics::Rect::new(450.0, 450.0, 50.0, 50.0);
        // let r2 = graphics::Mesh::new_rectangle(
        //     ctx,
        //     graphics::DrawMode::stroke(1.0),
        //     rect,
        //     graphics::Color::new(1.0, 0.0, 0.0, 1.0),
        // )?;
        // graphics::draw(ctx, &r2, DrawParam::default())?;

        // // Draw some pre-made meshes
        // for m in &self.meshes {
        //     graphics::draw(ctx, m, DrawParam::new())?;
        // }

        // Finished drawing, show it all on the screen!
        graphics::present(ctx)?;
        Ok(())
    }
}

pub fn main() -> GameResult {
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    let cb = ggez::ContextBuilder::new("drawing", "ggez").add_resource_path(resource_dir);

    let (ctx, events_loop) = &mut cb.build()?;

    println!("{}", graphics::renderer_info(ctx)?);
    let state = &mut MainState::new(ctx).unwrap();
    event::run(ctx, events_loop, state)
}