use tetra::graphics::{self, Color, Rectangle};
use tetra::graphics::mesh::{GeometryBuilder, Mesh, ShapeStyle};
use tetra::{Context, ContextBuilder, State, Result};
use tetra::math::Vec2;


const FIELD_WIDTH: f32 = 640.0;
const FIELD_HEIGHT: f32 = 640.0;

// 20 rectangles in a signel string
const PARTS: f32 = 20.0;

// Length of a side of a rectangle (square)
const RECT_SIZE: f32 = FIELD_WIDTH / PARTS;

// Struct contains a whole game state
struct GameState {
    rects: Mesh
}

impl GameState{
    // A constructor of all shapes 
    fn new(ctx: &mut Context) -> Result<GameState>{

        // Create a pair of fixed rectangles. They are a single shape
        let rects = GeometryBuilder::new()
            .set_color(Color::rgb(0.0, 1.0, 0.0))
            // Last parameters are coordinates of rectangles relative to the whole large shape - not the whole window!
            .rectangle(ShapeStyle::Fill, Rectangle::new(FIELD_WIDTH / 2.0 - RECT_SIZE, FIELD_HEIGHT / 2.0 - RECT_SIZE, RECT_SIZE, RECT_SIZE))?
            .build_mesh(ctx)?;


        Ok(GameState{rects})
    }
}


// Implement library trait for custom sctructure
impl State for GameState {
    fn draw(&mut self, ctx: &mut Context) -> Result{
        // Color of the field
        graphics::clear(ctx, Color::rgb(0.2, 0.2, 0.2));

        // Draw rectangles
        // The last parameter is a coordinate for the shape containing ractangles to be drawn at
        // All squares form a single shape that should be drawn at the upper left corner
        self.rects.draw(ctx, Vec2::new(0.0, 0.0));

        Ok(())
    }
}

fn main() -> Result{
    // Create a Context with titiled window
    ContextBuilder::new("Life", FIELD_WIDTH as i32, FIELD_HEIGHT as i32)
    .quit_on_escape(true)
    .build()?
    // The argument must return structure implementing State
    .run(GameState::new)
}