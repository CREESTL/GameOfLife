use tetra::graphics::{self, Color, Rectangle, DrawParams};
use tetra::graphics::mesh::{Mesh, ShapeStyle};
use tetra::graphics::text::{Font, Text};
use tetra::{Context, ContextBuilder, State, Result};
use tetra::window::{set_mouse_visible, quit};
use tetra::math::Vec2;
use tetra::input::{self, MouseButton, Key};
use tetra::time::Timestep;
// Similar to HashMap but with ordered indexing
use indexmap::IndexMap;


// Size of a field
const FIELD_WIDTH: f32 = 640.0;
const FIELD_HEIGHT: f32 = 640.0;

// 20 cells in a signle row
const ROW_PARTS: i32 = 20;

// Length of a side of a cell
const CELL_SIZE: f32 = FIELD_WIDTH / ROW_PARTS as f32;

// Width of the line of the grid
const LINE_WIDTH: f32 = 2.0;

// Width of menu part
const MENU_WIDTH: f32 = 100.0;

// Indent of a status text from the field
// Indent to the right and down
const STATUS_TEXT_INDENTS: (f32, f32) = (MENU_WIDTH / 4.0, 80.0 as f32);

// A sctructure of a single cell on the field
struct Cell{
    // ID of the cell
    id: i32,
    // Position of cell's upper left corner
    pos: Vec2<f32>,
    mesh: Mesh,
    // Status of the cell (alive/dead)
    alive: bool,
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
            Err(_) => panic!("Couldn`t create a cell!")
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


// Status text of the game
// It's eaither "Running" or "Paused"
struct StatusText{
    // Position of the text on the window
    pos: Vec2<f32>,
    text: Text,
}


impl StatusText{
    // Constructor of a status text
    fn new(ctx: &mut Context, pos: Vec2<f32>) -> StatusText{
        let font = Font::vector(ctx, "./resources/DejaVuSansMono.ttf", 21.0);
        let f = match font {
            Ok(font) => font,
            Err(_) => panic!("Can't read a font file!"),
        };
        let text = Text::new(
            "New Attempt", 
            f,
            );

        StatusText{pos, text}
    }
}


// Struct contains a whole game state
struct GameState {
    // Is the game running
    running: bool,
    // Vector of lines to form a grid
    grid: Vec<Line>,
    // A map of coordinates of cells
    // {cell_ID -> coordinates}
    cell_coords: IndexMap<i32, Vec2<f32>>,
    // Vector of all cells on the field 
    cells: Vec<Cell>,
    // Coordinates of a mouse
    mouse_coords: Vec2<f32>,
    // Game status text
    status_text: StatusText, 
}

impl GameState{
    // A constructor for a new game state
    fn new(ctx: &mut Context) -> Result<GameState>{
        let mut cells = Vec::new();
        let mut cell_coords = IndexMap::new();
        let mut grid = Vec::new();
        let mouse_coords = Vec2::new(FIELD_WIDTH / 2.0, FIELD_HEIGHT / 2.0);
        // By default the game is not running
        let running = false;
        // By default text indicates that game is stopped
        let status_text = StatusText::new(ctx, Vec2::new(FIELD_WIDTH + STATUS_TEXT_INDENTS.0, STATUS_TEXT_INDENTS.1));

        // Initialize all cell coordinates
        let mut x: f32 = 0.0;
        let mut y: f32 = 0.0;
        let mut id: i32 = 0;
        // Cell shouldn't be drawn after the last vertical line
        while x <= FIELD_WIDTH - 1.0 {
            while y <= FIELD_HEIGHT - 1.0 {
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
        while x <= FIELD_WIDTH + 1.0 {
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

        Ok(GameState{running, grid, cell_coords, cells, mouse_coords, status_text})
    }
    

    // Function resets the game state
    fn reset(&mut self){
        self.running = false;
        self.status_text.text.set_content("New Attempt");
        for cell in self.cells.iter_mut(){
            cell.alive = false;
        }
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

        // Draw text
        self.status_text.text.draw(ctx, DrawParams::new()
            .position(self.status_text.pos)
            .color(Color::rgb(0.643, 0.258, 0.862))
            );
        

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

    
    // TODO find a way to make different timesteps for mouse input and update rate
    // Function to update the state
    fn update(&mut self, ctx: &mut Context) -> Result{

        self.mouse_coords = input::get_mouse_position(ctx).round();

        // Revive or kill a cell with a LMB
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

        // Start or pause the game with SPACE
        if input::is_key_pressed(ctx, Key::Space){
            self.running = !self.running;
            match self.running {
                true => self.status_text.text.set_content("Running"),
                false => self.status_text.text.set_content("Paused"),
            };
        }
    

        // Reset the game with R
        if input::is_key_pressed(ctx, Key::R){
            self.reset();
        }


        // Quit the game with Q
        if input::is_key_pressed(ctx, Key::Q){
            quit(ctx);
        }

        // Main part - updating cells coordinates and alive statuses
        if self.running {

            // Vector of indexes of cells that should be alive in the next iteration
            // Contains `usize` elements!
            let mut next_cells = Vec::new();

            for id in 0..self.cells.len() {

                // Convert id to i32 to do calculations
                let id32 = id as i32;
                
                // Indexes of neighbours of the cell
                let n_ids = [
                    id32 - ROW_PARTS,
                    id32 + ROW_PARTS,
                    id32 - 1,
                    id32 + 1,
                    id32 - (ROW_PARTS - 1),
                    id32 + (ROW_PARTS - 1),
                    id32 - (ROW_PARTS + 1),
                    id32 + (ROW_PARTS + 1),
                ];

                // A number of alive neighbours of the cell
                let mut alive_neighbours = 0;
                // Create a list all 8 neighbour cells
                for n_id in n_ids{
                    // If the neighbour is alive and the distance to the neighbour is less than length of cell side multiplied by 2 - increment the 
                    // number of alive neighbours
                    if let Some(n_cell) = self.cells.get(n_id as usize) { 
                        if n_cell.alive && (self.cells[id].pos[1] as i32 - n_cell.pos[1] as i32).abs() <= (CELL_SIZE * 2.0) as i32{
                            alive_neighbours += 1;
                        }
                    }
                }
                
                // Check the total number of alive neighbours  
                match alive_neighbours {
                    // Cell survives if it has 2 or 3 neighbours
                    // Cell revives if it has 3 neighbours
                    // Cell dies in all other cases
                    // Add indexes of cells that should be alive in the next iteration
                    2 => {
                        if self.cells[id].alive == true {
                            next_cells.push(id);
                        }
                    },
                    3 => {
                        next_cells.push(id);
                    },
                    _ => ()
                };
            }


            // If none of cells should be alive on the next iteration - kill all of them
            if next_cells.len() == 0{
                for cell in self.cells.iter_mut(){
                    cell.alive = false;
                }
            // Else - only leave alive those from next cells
            } else {
                // Iterate through the cells and check if cell's ID is in the next cells
                for i in 0..self.cells.len(){
                    for j in 0..next_cells.len(){
                        // If it is - this cell should be alive
                        if self.cells[i].id == next_cells[j] as i32{
                            self.cells[i].alive = true;
                            break;
                        // If it is not - this cell should be dead
                        } else {
                            self.cells[i].alive = false;
                        }

                    }
                }
            }
        }

        Ok(())
    }   
}

fn main() -> Result {
    // Create a Context with titled window
    ContextBuilder::new("Game of Life", (FIELD_WIDTH + 200.0) as i32, (FIELD_HEIGHT + 0.0)  as i32)
    .timestep(Timestep::Fixed(5.0)) // How many times a second the State::update() runs
    .quit_on_escape(true)
    .build()?
    // Or just GameState::mew (sugar)
    .run(|ctx| GameState::new(ctx))
}
