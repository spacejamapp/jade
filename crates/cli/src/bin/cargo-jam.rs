use cargo_jam::cmd::App;
use clap::Parser;

fn main() {
    let app = App::parse();
    app.run().unwrap()
}
