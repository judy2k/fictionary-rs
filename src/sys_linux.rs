use camino::Utf8PathBuf;
use directories::ProjectDirs;

/// Return a (potentially empty) Vec of data directories for the platform.
///
/// Dirs are returned in increasing order of precedence - i.e.: local directories are last,
/// because they take precedence over shared directories.
/// All directories are guaranteed to exist, but may not be writeable.
pub fn data_dirs(qualifier: &str, organization: &str, application: &str) -> Vec<Utf8PathBuf> {
    let mut result = Vec::new();

    // Shared directories go here.
    result.append(&mut vec![
        format!("/usr/share/{application}").into(),
        format!("/usr/local/share/{application}").into(),
    ]);

    // Directory in home dir:
    if let Some(dirs) = ProjectDirs::from(qualifier, organization, application) {
        if let Ok(home_path) = Utf8PathBuf::from_path_buf(dirs.data_dir().into()) {
            result.push(home_path);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linux() {
        let dirs = data_dirs("uk.co", "judy", "fictionary");

        assert_eq!(dirs[0], Utf8PathBuf::from("/usr/share/fictionary"));
        assert_eq!(dirs[1], Utf8PathBuf::from("/usr/local/share/fictionary"));
        // In the 'cross' Linux environment, home is '/' so I'm just testing the suffix here,
        // so that the tests run on host Linux systems as well as cross.
        assert!(dirs[2].to_string().ends_with(".local/share/fictionary"));
    }
}
