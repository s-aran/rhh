use std::path::PathBuf;

use glob::glob;

pub fn glob_with_recursive<F>(pattern: &str, handler: &mut F)
where
    F: FnMut(&PathBuf) -> (),
{
    glob(pattern)
        .expect("Failed to read glob pattern")
        .for_each(|entry| match entry {
            Ok(path) => {
                if path.is_dir() {
                    glob_with_recursive(&format!("{}/*", path.display()), handler);
                } else {
                    handler(&path);
                }
            }
            Err(e) => println!("{:?}", e),
        });
}
