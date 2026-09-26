mod app;
mod pdf;
mod ui;

fn main() -> Result<(), slint::PlatformError> {
    app::run()
}
