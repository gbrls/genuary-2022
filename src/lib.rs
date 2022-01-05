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
