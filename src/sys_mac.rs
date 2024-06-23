use std::path::PathBuf;

use camino::Utf8PathBuf;
use directories::ProjectDirs;

/// Return a (potentially empty) Vec of data directories for the platform.
/// 
/// Dirs are returned in increasing order of precedence - i.e.: local directories are last,
/// because they take precedence over shared directories.
/// All directories are guaranteed to exist, but may not be writeable.
pub fn data_dirs(qualifier: &str, organization: &str, application: &str) -> Vec<Utf8PathBuf> {
    // we should replace more characters, according to RFC1034 identifier rules
    let organization = organization.replace(" ", "-");
    let application  = application.replace(" ", "-");
    let mut parts    = vec![qualifier, &organization, &application]; parts.retain(|e| !e.is_empty());
    let bundle_id    = parts.join(".");

    let mut result = Vec::new();

    // Shared directory:
    let shared = Utf8PathBuf::from("/Library/Application Support").join(&bundle_id);
    result.push(shared);

    // TODO: Add Homebrew here?
    // $(brew --prefix)/share/fictionary

    // Directory in home dir:
    if let Some(dirs) = ProjectDirs::from_path(PathBuf::from(&bundle_id)) {
        if let Ok(home_path) = Utf8PathBuf::from_path_buf(dirs.data_dir().into()) {
            result.push(home_path);
        }
    }

    result
}