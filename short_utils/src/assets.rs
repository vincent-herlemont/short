use std::collections::HashMap;

#[allow(dead_code)]
pub fn get_all() -> HashMap<&'static str, &'static str> {
    let mut out = HashMap::new();
    out.insert("assets/other_conf.yaml", include_str!("assets/other_conf.yaml"));
    out.insert("assets/test/test.js", include_str!("assets/test/test.js"));
    out.insert("assets/altered_aws_template.yaml", include_str!("assets/altered_aws_template.yaml"));
    out.insert("assets/tpl_certificate/certificate.yaml", include_str!("assets/tpl_certificate/certificate.yaml"));
    out.insert("assets/valid_aws_template.yaml", include_str!("assets/valid_aws_template.yaml"));
    out
}
