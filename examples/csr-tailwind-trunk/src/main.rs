use sauron::Program;

use csr_tailwind_trunk::App;

pub fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    log::info!("Mount Sauron app to body");
    Program::mount_to_body(App::default());
}
