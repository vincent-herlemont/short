


use crate::env_file::Env;


pub fn sync(_envs: &Vec<Env>) {}

#[cfg(test)]
mod tests {
    use crate::env_file::Env;
    
    use std::path::PathBuf;

    #[test]
    fn test_sync() {
        let mut e1 = Env::new(PathBuf::new());
        e1.add("var1", "value1");
        let mut e2 = e1.copy(PathBuf::new());
        e2.add("var2", "value2");

        let _v = vec![e1, e2];
    }
}
