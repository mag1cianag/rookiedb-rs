use std::path::Path;

fn main() {
    let path = Path::new("/tmp/foo/bar.txt");
    let components = path.components().collect::<Vec<_>>();
}