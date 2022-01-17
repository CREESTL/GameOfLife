use tetra::graphics::{self, Color, Rectangle, DrawParams};
use tetra::graphics::mesh::{GeometryBuilder, Mesh, ShapeStyle};
use tetra::{Context, ContextBuilder, State, Result};
use tetra::window::set_mouse_visible;
use tetra::math::Vec2;
use tetra::input::{self, MouseButton};
use tetra::time::Timestep;
// Similar to HashMap but with ordered indexing
use indexmap::IndexMap;


// Size of a field
const FIELD_WIDTH: f32 = 640.0;
const FIELD_HEIGHT: f32 = 640.0;

// 20 cells in a signle row
const ROW_PARTS: f32 = 20.0;

// Length of a side of a cell
const CELL_SIZE: f32 = FIELD_WIDTH / ROW_PARTS;

// Width of the line of the grid
const LINE_WIDTH: f32 = 2.0;

// A sctructure of a single cell on the field
// Cell has and id(number), a position (coordinates) and a mesh (texture)
struct Cell{
    // ID of the cell
    id: i32,
    // Position of cell's upper left corner
    pos: Vec2<f32>,
    mesh: Mesh,
    // Status of the cell (alive/dead)
    alive: bool
}


impl Cell{
    // Constructor for a cell
    fn new(id: i32, pos: Vec2<f32>, alive: bool, ctx: &mut Context) -> Cell{
        // Mesh should be a bit smaller for the grid lines to fit
        let gap = LINE_WIDTH * 0.5;
        let mesh = Mesh::rectangle(ctx, ShapeStyle::Fill, Rectangle::new(0.0 + gap , 0.0 + gap, CELL_SIZE - 2.0 * gap, CELL_SIZE - 2.0 * gap)); 
        match mesh{
            Ok(mesh) =>  Cell{id, pos, mesh, alive},
            // TODO a more fancy way to handle it?
            Err(e) => panic!("{}", e)
        }
       
    }
}

// A single line
struct Line{
    width: f32,
    points: [Vec2<f32>; 2],
    mesh: Mesh,
}

impl Line{
    // Constructor for a line
    fn new(width: f32, points: [Vec2<f32>; 2], ctx: &mut Context) -> Line{
        let mesh = Mesh::polyline(ctx, width, &points);
        match mesh{
            Ok(mesh) =>  Line{width, points, mesh},
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
    // Vector of all cells on the field 
    cells: Vec<Cell>,
    // Coordinates of a mouse
    mouse_coords: Vec2<f32>,

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
        // Coordinates of the mouse 
        let mouse_coords = Vec2::new(FIELD_WIDTH / 2.0, FIELD_HEIGHT / 2.0);
        
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
            x += CELL_SIZE; }
        
        // Initialize all cells with those coordinates
        for (_num, (id, coords)) in cell_coords.iter().enumerate() {
            // All cells are initialized as dead ones
            let cell = Cell::new(*id as i32, *coords, false, ctx);
            cells.push(cell);
         }   

        // Initialize all grid lines with a constant set of coordinates
        x = 0.0;
        y = 0.0;
        // Vertical lines
        while x <= FIELD_WIDTH {
            let line = Line::new(LINE_WIDTH, [Vec2::new(x, y), Vec2::new(x, FIELD_HEIGHT)], ctx);
            grid.push(line);
            x += CELL_SIZE;
        }

        x = 0.0;
        y = 0.0;
        // Horizontal lines
        while y <= FIELD_HEIGHT {
            let line = Line::new(LINE_WIDTH, [Vec2::new(x, y), Vec2::new(FIELD_WIDTH, y)], ctx);
            grid.push(line);
            y += CELL_SIZE;
        }
        
        // Make mouse cursor visible on the field
        match set_mouse_visible(ctx, true){
            Ok(_) => (),
            Err(_) => panic!("Can not see the mouse!"),
        }


        Ok(GameState{grid, cell_coords, cells, mouse_coords})
    }
    
    // Function to find a corresponding cell for the cursor
    fn point_to_cell(&self) -> i32 {
        let mouse_x = self.mouse_coords[0];
        let mouse_y = self.mouse_coords[1];
        for (_, cell) in self.cells.iter().enumerate(){
            // First check the lower right corner of the cell
            if (mouse_x <= cell.pos[0] + CELL_SIZE) && (mouse_y <= cell.pos[1] + CELL_SIZE){
                // Then check the upper left corner of the cell
                if (mouse_x >= cell.pos[0]) && (mouse_y >= cell.pos[1]){
                    return cell.id
                }
            }   
        }

        // Return -1 if none matches
        -1
    }   

}


// Implement library trait for custom sctructure
impl State for GameState {
    // Function to draw all meshes
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
            // *only alive cells
            if cell.alive {
                cell.mesh.draw(ctx, DrawParams::new()
                .position(Vec2::new(cell.pos[0], cell.pos[1]))
                .color(Color::rgb(0.0, 1.0, 0.0))
                );

            }
        }             
        
        Ok(())
    }
    
    // Function to update the state
    fn update(&mut self, ctx: &mut Context) -> Result{
        self.mouse_coords = input::get_mouse_position(ctx).round();

        // The button is actually pressed for a few milliseconds and the program captures that
        // So even if a use doesn't move the mouse, the program captures several values of mouse
        // coordinate
        if input::is_mouse_button_pressed(ctx, MouseButton::Left){
            let pointed_cell_id =  self.point_to_cell();
            if let Some(mut cell) = self.cells.get_mut(pointed_cell_id as usize) {
                if cell.alive == false {
                    cell.alive = true;
                } else {
                    cell.alive = false;
                }

            }
       }

        

        Ok(())
    }   



}

fn main() -> Result{
    // Create a Context with titled window
    ContextBuilder::new("Life", FIELD_WIDTH as i32, FIELD_HEIGHT as i32)
    .timestep(Timestep::Fixed(60.0)) // How many times a second the State::update() runs
    .quit_on_escape(true)
    .build()?
    // Or just GameState::mew (sugar)
    .run(|ctx| GameState::new(ctx))
}
