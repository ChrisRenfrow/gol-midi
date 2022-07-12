use yagoll::Board;

use std::error::Error;
use std::io::Write;
use std::{io, thread, time};

/// A struct representing application configuration
pub struct Config {
    input_file: String,
    midi: MidiConfig,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let input_file = args[1].clone();
        let midi_cfg_file = args[2].clone();
        let midi = MidiConfig::new(&midi_cfg_file);
        Ok(Config { input_file, midi })
    }
}

/// Represents the board metrics available for mapping to MIDI events
enum BoardMetric {
    Ratio,
    Activity,
}

/// Supported MIDI events
enum MidiMsg {
    Channel,
    Note,
    Pitch,
}

/// Bindings for board metrics to MIDI messages, read from a file
struct MidiConfig {
    bindings: Vec<(BoardMetric, MidiMsg)>,
}

impl MidiConfig {
    fn new(cfg_file: &str) -> Self {
        // TODO:
        MidiConfig { bindings: vec![] }
    }
}

/// The main process loop
pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let mut board = Board::new_from_file(&config.input_file);
    loop {
        let activity = calc_board_activity(&board);
        println!("Activity: {}", activity);
        board.advance_cycle();
        println!("{}", board);
        io::stdout().flush()?;
        thread::sleep(time::Duration::from_millis(1000));
    }
}

/// Return the ratio of alive to dead cells
fn board_alive_to_dead_ratio(board: &Board) -> f32 {
    todo!();
}

/// Return the "activity level" of the board (number of updates between cycles).
fn calc_board_activity(board: &Board) -> f32 {
    // Copy board
    let mut count = 0;
    let width = board.width;
    let height = board.height;
    let mut next = board.clone();
    next.advance_cycle();
    for x in 0..width {
        for y in 0..height {
            count += if board.get(x, y) != next.get(x, y) {
                1
            } else {
                0
            };
        }
    }

    count as f32 / (width as f32 * height as f32)
}
