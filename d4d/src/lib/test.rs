//! Helper for test related of d4d domain.
use crate::lib::resource;
use crate::lib::resource::Resource;
use std::fs;
use std::path::PathBuf;
use tempdir::TempDir;

pub struct Config {
    pub tmp_dir: PathBuf,
}

impl Drop for Config {
    fn drop(&mut self) {
        fs::remove_dir_all(self.tmp_dir.clone()).expect("can not clean tmp directory");
    }
}

/// Return [`InspectorConfig`], create temporary directory and copy resource on it.
///
/// The temporary directory is owned by [`InspectorConfig.path`].
///
/// # Recommendation
///
/// Need to call [`after`] at the end of test.
pub fn before(test_name: &str) -> Config {
    let test_name = format!("{}.{}", "d4d", test_name);

    // Create temporary directory.
    let path = TempDir::new(test_name.as_str())
        .expect("fail to create temporary directory")
        .into_path();

    // Copy resources to it.
    resource::to_dir(&path).expect("fail to copy resources");

    Config { tmp_dir: path }
}

/// Assert that patter value or|and expression is present on an vector.
///
/// # Notice
/// The macro don't take the ownership of a vector.
///
/// You have to add [`#[allow(unreachable_patterns)]`] to avoid warning
///
/// # Example
/// ```
/// let v = vec![1,2,3,4];
///assert_find!(v, 2);
///assert_find!(v, 8); // assertion failed: can not found {8}  in {v}
///assert_find!(v, el, el < &&5)
///assert_find!(v, el, el > &&5); // assertion failed: can not found {el} with expresion {el > &&5} in {v}
///
/// ```
#[macro_export]
macro_rules! assert_find {
    ($v:ident,$arms:pat) => {
        assert_find!($v, $arms, true, true)
    };
    ($v:ident,$arms:pat,$e:expr) => {
        assert_find!($v, $arms, $e, true)
    };
    ($v:ident,$arms:pat,$e:expr,$b:expr) => {
        if (&$v)
            .iter()
            .find(|el| match el {
                $arms => $e,
                _ => false,
            })
            .is_some()
            != $b
        {
            panic!(format!(
                "assertion failed: {} {{{}}} {} in {{{}}}",
                if ($b) {
                    String::from("can not found")
                } else {
                    String::from("found")
                },
                String::from(stringify!($arms)),
                if (stringify!($e) == "true" || stringify!($e) == "false") {
                    String::from("")
                } else {
                    String::from(format!(
                        "with expresion {{{}}}",
                        String::from(stringify!($e))
                    ))
                },
                String::from(stringify!($v))
            ));
        };
    };
}

/// Assert that patter value or|and expression is not present on an vector.
///
/// # Notice
/// The macro don't take the ownership of a vector.
///
/// You have to add [`#[allow(unreachable_patterns)]`] to avoid warning
///
/// # Example
/// ```
/// let v = vec![1,2,3,4];
///assert_not_find!(v, 8);
///assert_not_find!(v, 2); // assertion failed: found {2}  in {v}
///assert_not_find!(v, el, el > &&5);
///assert_not_find!(v, el, el < &&5); // assertion failed: found {el} with expresion {el < &&5} in {v}
/// ```
#[macro_export]
macro_rules! assert_not_find {
    ($v:ident,$arms:pat) => {
        assert_find!($v, $arms, true, false)
    };
    ($v:ident,$arms:pat,$e:expr) => {
        assert_find!($v, $arms, $e, false)
    };
}

#[cfg(test)]
mod tests {

    #[allow(unreachable_patterns)]
    #[test]
    fn assert_find_macro_test() {
        let v = vec![1, 2, 3, 4];
        assert_find!(v, 2);
        // assert_find!(v, 8); // assertion failed: can not found {8}  in {v}
        assert_find!(v, el, el < &&5);
        // assert_find!(v, el, el > &&5); // assertion failed: can not found {el} with expresion {el > &&5} in {v}

        assert_not_find!(v, 8);
        // assert_not_find!(v, 2); // assertion failed: found {2}  in {v}
        assert_not_find!(v, el, el > &&5);
        //assert_not_find!(v, el, el < &&5); // assertion failed: found {el} with expresion {el < &&5} in {v}
    }
}

pub const TEST_RESOURCE_DIRECTORY: &'static str = "./init_tpl";

/// Get all [`Resource`]
pub fn get_resource() -> Vec<Resource> {
    vec![
        Resource::new(
            "./init_tpl/other_conf.yaml",
            include_str!("./init_tpl/0_other_conf.yaml"),
        ),
        Resource::new(
            "./init_tpl/1_certificate.yaml",
            include_str!("./init_tpl/1_certificate.yaml"),
        ),
        Resource::new(
            "./init_tpl/1_certificate_altered.yaml",
            include_str!("./init_tpl/2_certificate_altered.yaml"),
        ),
        Resource::new(
            "./init_tpl/3_test/0_test.js",
            include_str!("./init_tpl/3_test/0_test.js"),
        ),
        Resource::new(
            "./init_tpl/4_tpl_certificate/certificate.yaml",
            include_str!("./init_tpl/4_tpl_certificate/certificate.yaml"),
        ),
    ]
}
