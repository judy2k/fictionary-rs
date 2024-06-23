use std::path::PathBuf;

use camino::Utf8PathBuf;
use directories::ProjectDirs;

/// Return a (potentially empty) Vec of data directories for the platform.
/// 
/// Dirs are returned in increasing order of precedence - i.e.: local directories are last,
/// because they take precedence over shared directories.
/// All directories are guaranteed to exist, but may not be writeable.
fn data_dirs(qualifier: &str, organization: &str, application: &str) -> Vec<Utf8PathBuf> {
    let mut result = Vec::new();

    // Shared directory goes here.

    // Directory in home dir:
    if let Some(dirs) = ProjectDirs::from(qualifier, organization, application) {
        if let Ok(home_path) = Utf8PathBuf::from_path_buf(dirs.data_dir().into()) {
            result.push(home_path);
        }
    }

    result
}