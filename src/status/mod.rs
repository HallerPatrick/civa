use xdg::BaseDirectories;

fn show_status() {
    let base_dir = xdg::BaseDirectories::with_prefix("civa");

    if let Ok(dir) = base_dir {
        // dir.find_data_file(path)
    }
}
