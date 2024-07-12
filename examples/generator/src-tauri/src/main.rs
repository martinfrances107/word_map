// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

extern crate log;
extern crate word_map;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn update(tw: &str) -> String {
    use word_map::block::Block;
    use word_map::block::Blocks;
    use word_map::grid::Grid;
    use word_map::grid::TextWeight;
    use word_map::Orientation;
    use word_map::Point2d;

    static WIDTH: f32 = 800_f32;
    static HEIGHT: f32 = 800_f32;

    // generate_word_map

    // TODO:
    // Make grid  thread safe!!!
    //
    // Grid contains ThreadRng
    // ThredRnd contains Rc<UnsafeCell<ReseedingRng<Core, OsRng>>>
    //
    // I want the grid here  to be in a lazy static but I need to first find a
    // diffent RNG generate.
    println!("tw {:#?}", tw);
    match Grid::parse_pairs(tw) {
        Ok((_, pairs)) => {
            println!("pairs {:#?}", pairs);
            let mut grid = Grid::new(WIDTH, HEIGHT);
            for TextWeight(text, weight) in pairs {
                grid.place_block(text.to_string(), weight as f32);
            }
            let b = Blocks(grid.blocks);
            println!("b {b:#?}");

            match serde_json::to_string(&b) {
                Ok(blocks) => blocks,
                Err(e) => String::from("error converting blocks"),
            }
        }
        Err(_) => String::from("failed to pairs tw into TextWeight"),
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![update])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
