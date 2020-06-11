// TODO : In the future registry will be downloaded from this repository.
// https://github.com/vincent-herlemont/short-template-index


use std::collections::HashMap;

pub struct Registry {}

impl Registry {
    fn data() -> HashMap<String, String> {
        let mut data = HashMap::new();
        data.insert(
            "aws-sam".to_string(),
            "https://github.com/vincent-herlemont/aws-sam-short-template.git".to_string(),
        );
        data
    }
}
