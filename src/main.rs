use tetra::graphics::{self, Color, Rectangle, DrawParams};
use tetra::graphics::mesh::{GeometryBuilder, Mesh, ShapeStyle};
use tetra::{Context, ContextBuilder, State, Result};
use tetra::math::Vec2;
use std::collections::HashMap;

// Size of a field
const FIELD_WIDTH: f32 = 640.0;
const FIELD_HEIGHT: f32 = 640.0;

// 20 rectangles in a signle row
const ROW_PARTS: f32 = 20.0;

// Length of a side of a rectangle (square)
const RECT_SIZE: f32 = FIELD_WIDTH / ROW_PARTS;


// A sctructure of a single cell on the field
// Cell has and id(number), a position (coordinates) and a mesh (texture)
struct Cell{
    id: i32,
    pos: Vec2<f32>,
    mesh: Mesh,
}


impl Cell{
    // Constructor for a cell
    fn new(id: i32, pos: Vec2<f32>, ctx: &mut Context) -> Cell{
        let mesh = Mesh::rectangle(ctx, ShapeStyle::Fill, Rectangle::new(0.0, 0.0, RECT_SIZE, RECT_SIZE)); 
        match mesh{
            Ok(mesh) => return Cell{id, pos, mesh},
            // TODO a more fancy way to handle it?
            Err(e) => panic!("{}", e)
        }
       
    }
}


// Struct contains a whole game state
struct GameState {
    // A hashmap of coordinates of cells
    // {cell_ID -> coordinates}
    coords: HashMap<i32, Vec2<f32>>,
    // Vector of cells to be located on the field 
    cells: Vec<Cell>
    

}

impl GameState{
    // A constructor for a new game state
    fn new(ctx: &mut Context) -> Result<GameState>{
        // A vector of cells 
        let mut cells = Vec::new();
        let mut coords = HashMap::new();

        // Initialize all coordinates
        let examples = [[10.0,10.0], [150.0, 150.0]];
        for (i, c) in examples.iter().enumerate() {
            coords.insert(i as i32, Vec2::new(c[0] as f32, c[1] as f32)); 
            
        } 

        // Initialize all cells with those coordinates
        for (key, (id, coords)) in coords.iter().enumerate() {
            let cell = Cell::new(*id as i32, *coords, ctx);
            cells.push(cell);
        }   
        
        Ok(GameState{coords, cells})
    }

}


// Implement library trait for custom sctructure
impl State for GameState {
    fn draw(&mut self, ctx: &mut Context) -> Result{
        // Color of the field
        graphics::clear(ctx, Color::rgb(0.2, 0.2, 0.2));

        // Draw rectangles
        for cell in self.cells.iter(){
            cell.mesh.draw(ctx, DrawParams::new()
                .position(Vec2::new(cell.pos[0], cell.pos[1]))
                .color(Color::rgb(0.0, 1.0, 0.0))
                );
        }

        Ok(())
    }


}

fn main() -> Result{
    // Create a Context with titled window
    ContextBuilder::new("Life", FIELD_WIDTH as i32, FIELD_HEIGHT as i32)
    .quit_on_escape(true)
    .build()?
    // Or just GameState::mew (sugar)
    .run(|ctx| GameState::new(ctx))
}
