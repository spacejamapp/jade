//! jade command line interface

use cjam::{clap, cmd::App};

fn main() {
    let app = App::parse();
    app.run().unwrap()
}
