use std::io::{self, Write};

// const DONE_COLOR: &str = "\x1b[32m"; // green
const DONE_COLOR: &str = "\x1b[38;2;255;120;0m"; // orange
const TODO_COLOR: &str = "\x1b[90m"; // gray
const TEXT_COLOR: &str = "\x1b[90m"; // gray
const RESET_COLOR: &str = "\x1b[0m";
use once_cell::sync::Lazy;

static STR_DONE: Lazy<String> = Lazy::new(|| format!("{}{}{}", DONE_COLOR, '━', TODO_COLOR));
static STR_AFTER: Lazy<String> = Lazy::new(|| format!("╺"));
static STR_TODO: Lazy<String> = Lazy::new(|| format!("━"));

pub fn print_loading_progress(progress: i32, total: i32) {
    let i = (progress * 40) / (if total != 0 { total } else { 1 });
    print!(
        "\r{}Loading... {}{} ",
        TEXT_COLOR,
        (0..40)
            .map(|j| if j > i + 1 {
                &**STR_TODO
            } else if j == i + 1 {
                &**STR_AFTER
            } else {
                &**STR_DONE
            })
            .collect::<String>(),
        RESET_COLOR
    );
    io::stdout().flush().unwrap();
}

// pub fn test_print_loading() {
//     let start_time = Instant::now();
//     for i in 0..100 {
//         print_loading_progress(i, 100, start_time);
//         sleep(Duration::from_millis(3));
//     }
//     println!();
// }
