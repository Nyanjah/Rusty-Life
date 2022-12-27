use minifb::*;

mod conway;
use conway::*;

const WIDTH:usize = 450*2;
const HEIGHT:usize = 200*2;
fn main(){
    // Setting up the window
    let mut window = Window::new(
        "Conway's Game of Life - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions {
            borderless: false,
            title: true,
            resize: true,
            scale: Scale::X2,
            scale_mode: ScaleMode::Stretch,
            ..Default::default()
        },
    ).unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to update rate to retain  ~1 sec / frame
    window.limit_update_rate(Some(std::time::Duration::from_secs_f32(0.02)));

    let mut paused:bool = false;
    let mut simulation = Simulation::new(String::from("Game of Life"),WIDTH as u32, HEIGHT as u32);
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    while window.is_open() && !window.is_key_down(Key::Escape) {
        if window.is_key_released(Key::R){
            simulation.randomize_world();
        }
        if paused{
            window.update();
            if !window.is_key_down(Key::X){
                paused = false;

            }
            continue;
        }
        if window.is_key_down(Key::X){
            paused = true;
            window.update();
            continue;
        }

        
        let world_state = simulation.get_world();
        // Update the buffer with the simulation world data
        for y in (0..HEIGHT).rev(){
            for x in 0..WIDTH{
                let cell_is_alive = world_state.grid[x][y].alive;
                let count = world_state.grid[x][y].neighbor_count;
                buffer.push({
                    if cell_is_alive && count == 2 {(255 as u32) << 8}
                    else if cell_is_alive && count == 3 {(255 as u32) << 16}
                    else if cell_is_alive {(255 as u32)}
                    else{0}});
            }
        }
        
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
        buffer.clear();
        simulation.step(); 
        println!("{}",simulation.population);
    }



}



