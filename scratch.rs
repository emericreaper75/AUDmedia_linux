use gtk4::{gio, Image};
fn test() {
    let img = Image::new();
    img.set_file(Some(&gio::File::for_path("a")));
    img.set_icon_name(Some("b"));
}
