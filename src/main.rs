mod app;
mod constants;

use app::*;

fn main() {
    let mut app = App::new();
    std::process::exit(app.main());
}
