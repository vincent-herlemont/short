use std::collections::HashMap;

#[allow(dead_code)]
pub fn get_all() -> HashMap<&'static str, &'static str> {
    let mut out = HashMap::new();
    out.insert("assets/certificate.yaml", include_str!("assets/certificate.yaml"));
    out
}
