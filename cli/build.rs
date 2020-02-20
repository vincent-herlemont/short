use include_walk::walk;

fn main() {
    walk("./")
        .filter(|e| e.path().to_string_lossy().contains("assets"))
        .method("get_assets")
        .str()
        .to("./src/assets.rs")
        .unwrap();
}
