use nannou::prelude::*;

pub fn some_fn_from_lib() -> Option<i32> {
    Some(42)
}

pub fn capture_path(app: &nannou::App) -> std::path::PathBuf {
    app.project_path()
        .unwrap()
        .join("renders")
        .join(app.exe_name().unwrap())
        .join(format!("{:003}", app.elapsed_frames()))
        .with_extension("png")
}

pub trait SaveAnimation {
    fn save_animation(&self) { }
}

impl SaveAnimation for nannou::App {
    fn save_animation(&self) {
        self.main_window().capture_frame(capture_path(self))
    }
}
