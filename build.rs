fn main() {
    slint_build::compile("views/main-window.slint")
        .expect("failed to compile the Slint user interface");
}
