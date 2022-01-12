use tetra::graphics::{self, Color, Rectangle, DrawParams};
use tetra::graphics::mesh::{GeometryBuilder, Mesh, ShapeStyle};
use tetra::{Context, ContextBuilder, State, Result};
use tetra::math::Vec2;
// Similar to HashMap but with ordered indexing
use indexmap::IndexMap;


// Size of a field
const FIELD_WIDTH: f32 = 640.0;
const FIELD_HEIGHT: f32 = 640.0;

// 20 cells in a signle row
const ROW_PARTS: f32 = 20.0;

// Length of a side of a cell
const CELL_SIZE: f32 = FIELD_WIDTH / ROW_PARTS;


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
        let mesh = Mesh::rectangle(ctx, ShapeStyle::Fill, Rectangle::new(0.0, 0.0, CELL_SIZE, CELL_SIZE)); 
        match mesh{
            Ok(mesh) => return Cell{id, pos, mesh},
            // TODO a more fancy way to handle it?
            Err(e) => panic!("{}", e)
        }
       
    }
}

// A single line
struct Line{
    width: f32,
    points: [Vec2<f32>; 2],
    mesh: Mesh
}

impl Line{
    // Constructor for a line
    fn new(width: f32, points: [Vec2<f32>; 2], ctx: &mut Context) -> Line{
        let mesh = Mesh::polyline(ctx, width, &points);
        match mesh{
            Ok(mesh) => return Line{width, points, mesh},
            Err(e) => panic!("{}", e)
        }
    }
}


// Struct contains a whole game state
struct GameState {
    // Vector of lines to form a grid
    grid: Vec<Line>,
    // A map of coordinates of cells
    // {cell_ID -> coordinates}
    cell_coords: IndexMap<i32, Vec2<f32>>,
    // Vector of cells to be located on the field 
    cells: Vec<Cell>,

}

impl GameState{
    // A constructor for a new game state
    fn new(ctx: &mut Context) -> Result<GameState>{
        // A vector of cells 
        let mut cells = Vec::new();
        // A vector of coordinates of each cell (upper left corner)
        let mut cell_coords = IndexMap::new();
        // A vector of coordinates to build a grid 
        let mut grid = Vec::new();
        
        
        // Initialize all cell coordinates
        let mut x: f32 = 0.0;
        let mut y: f32 = 0.0;
        let mut id: i32 = 0;
        while x <= FIELD_WIDTH {
            while y <= FIELD_HEIGHT {
                cell_coords.insert(id, Vec2::new(x, y));
                y += CELL_SIZE;
                id += 1;
            }
            y = 0.0;
            x += CELL_SIZE;
        }
        
        // Initialize all cells with those coordinates
         for (num, (id, coords)) in cell_coords.iter().enumerate() {
             if num % 2 == 0 {
                 let cell = Cell::new(*id as i32, *coords, ctx);
                 cells.push(cell);
             }
         }   

        // Initialize all grid lines with a constant set of coordinates
        x = 0.0;
        y = 0.0;
        // Vertical lines
        while x <= FIELD_WIDTH {
           let line = Line::new(2.0, [Vec2::new(x, y), Vec2::new(x, FIELD_HEIGHT)], ctx);
           grid.push(line);
           x += CELL_SIZE;
        }

        x = 0.0;
        y = 0.0;
        
        // Horizontal lines
        while y <= FIELD_HEIGHT {
            let line = Line::new(2.0, [Vec2::new(x, y), Vec2::new(FIELD_WIDTH, y)], ctx);
            grid.push(line);
            y += CELL_SIZE;
        }


        Ok(GameState{grid, cell_coords, cells})
    }

}


// Implement library trait for custom sctructure
impl State for GameState {
    fn draw(&mut self, ctx: &mut Context) -> Result{
        // Color of the field
        graphics::clear(ctx, Color::rgb(0.2, 0.2, 0.2));

        // Draw grid
        for line in self.grid.iter(){
            line.mesh.draw(ctx, DrawParams::new()
                       .color(Color::rgb(1.0, 0.0, 0.0))
                       );
        }   
        // Draw cells 
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
