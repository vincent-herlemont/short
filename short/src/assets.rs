use std::collections::HashMap;

#[allow(dead_code)]
pub fn get_all() -> HashMap<&'static str, &'static str> {
    let mut out = HashMap::new();
    out.insert("assets/other_conf.yaml", include_str!("assets/other_conf.yaml"));
    out.insert("assets/tpl_bucket_template/steps.sh", include_str!("assets/tpl_bucket_template/steps.sh"));
    out.insert("assets/tpl_bucket_template/bucket_template.yaml", include_str!("assets/tpl_bucket_template/bucket_template.yaml"));
    out.insert("assets/sam_test/events/event.json", include_str!("assets/sam_test/events/event.json"));
    out.insert("assets/sam_test/.gitignore", include_str!("assets/sam_test/.gitignore"));
    out.insert("assets/sam_test/steps.sh", include_str!("assets/sam_test/steps.sh"));
    out.insert("assets/sam_test/README.md", include_str!("assets/sam_test/README.md"));
    out.insert("assets/sam_test/template.yaml", include_str!("assets/sam_test/template.yaml"));
    out.insert("assets/sam_test/template.pkg.cf.yaml", include_str!("assets/sam_test/template.pkg.cf.yaml"));
    out.insert("assets/sam_test/hello-world/app.js", include_str!("assets/sam_test/hello-world/app.js"));
    out.insert("assets/sam_test/hello-world/package.json", include_str!("assets/sam_test/hello-world/package.json"));
    out.insert("assets/sam_test/hello-world/tests/unit/test-handler.js", include_str!("assets/sam_test/hello-world/tests/unit/test-handler.js"));
    out.insert("assets/sam_test/hello-world/.npmignore", include_str!("assets/sam_test/hello-world/.npmignore"));
    out.insert("assets/sam_test/template.pkg.sam.yaml", include_str!("assets/sam_test/template.pkg.sam.yaml"));
    out.insert("assets/test/test.js", include_str!("assets/test/test.js"));
    out.insert("assets/altered_aws_template.yaml", include_str!("assets/altered_aws_template.yaml"));
    out.insert("assets/validate_aws_template.yaml", include_str!("assets/validate_aws_template.yaml"));
    out.insert("assets/tpl_certificate/steps.sh", include_str!("assets/tpl_certificate/steps.sh"));
    out.insert("assets/tpl_certificate/certificate.pkg.yaml", include_str!("assets/tpl_certificate/certificate.pkg.yaml"));
    out.insert("assets/tpl_certificate/certificate.yaml", include_str!("assets/tpl_certificate/certificate.yaml"));
    out.insert("assets/tpl_nested/child/child.yaml", include_str!("assets/tpl_nested/child/child.yaml"));
    out.insert("assets/tpl_nested/child/example_child_lambda/handler.js", include_str!("assets/tpl_nested/child/example_child_lambda/handler.js"));
    out.insert("assets/tpl_nested/steps.sh", include_str!("assets/tpl_nested/steps.sh"));
    out.insert("assets/tpl_nested/main.pkg.yaml", include_str!("assets/tpl_nested/main.pkg.yaml"));
    out.insert("assets/tpl_nested/example_parent_lambda/handler.js", include_str!("assets/tpl_nested/example_parent_lambda/handler.js"));
    out.insert("assets/tpl_nested/main.yaml", include_str!("assets/tpl_nested/main.yaml"));
    out.insert("assets/tpl_nested/parent.yaml", include_str!("assets/tpl_nested/parent.yaml"));
    out
}
