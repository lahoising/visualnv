mod app;

use app::App;

fn main() -> Result<(), app::Error> {
    App::new()?.run()
}
