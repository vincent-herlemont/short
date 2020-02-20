use utils::asset::Asset;

/// Get all [`Asset`]
#[allow(dead_code)]
pub fn get_assets() -> Vec<Asset> {
    vec![
        Asset::new(
            "./src/assets",
            include_str!("./src/assets"),
        ),
        Asset::new(
            "./src/assets/certificate.yaml",
            include_str!("./src/assets/certificate.yaml"),
        ),
        Asset::new(
            "./assets.rs",
            include_str!("./assets.rs"),
        ),
    ]
}
