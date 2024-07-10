// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

extern crate word_map;

use word_map::block::Block;
use word_map::grid::Grid;
use word_map::grid::TextWeight;

static WIDTH: f32 = 800_f32;
static HEIGHT: f32 = 800_f32;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn update(input: &str) -> Vec<Block> {
    // generate_word_map

    // TODO:
    // Make grid  thread safe!!!
    //
    // Grid contains ThreadRng
    // ThredRnd contains Rc<UnsafeCell<ReseedingRng<Core, OsRng>>>
    //
    // I want the grid here  to be in a lazy static but I need to first find a
    // diffent RNG generate.
    match Grid::parse_pairs(input) {
        Ok((_, pairs)) => {
            let mut grid = Grid::new(WIDTH, HEIGHT);
            for TextWeight(text, weight) in pairs {
                grid.place_block(text.to_string(), weight as f32);
            }
            grid.blocks
        }
        Err(_) => {
            vec![]
        }
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![update])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
