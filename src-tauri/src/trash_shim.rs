// trash_shim.rs — Provides a `trash`-compatible API across all platforms.
//
// On desktop (Windows, macOS, Linux), this re-exports the real `trash` crate.
// On Android there is no system trash, so we fall back to permanent deletion
// and expose a matching error type so the call sites compile unchanged.

#[cfg(not(target_os = "android"))]
pub use trash::{delete, delete_all};

#[cfg(target_os = "android")]
mod android {
    use std::{fs, path::Path};

    #[derive(Debug)]
    pub struct Error(String);

    impl std::fmt::Display for Error {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    impl std::error::Error for Error {}

    /// On Android: permanently delete the path (no system trash available).
    pub fn delete<P: AsRef<Path>>(path: P) -> Result<(), Error> {
        let path = path.as_ref();
        if path.is_dir() {
            fs::remove_dir_all(path).map_err(|e| Error(e.to_string()))
        } else {
            fs::remove_file(path).map_err(|e| Error(e.to_string()))
        }
    }

    /// On Android: permanently delete all given paths.
    pub fn delete_all<I, P>(paths: I) -> Result<(), Error>
    where
        I: IntoIterator<Item = P>,
        P: AsRef<Path>,
    {
        for path in paths {
            delete(path)?;
        }
        Ok(())
    }
}

#[cfg(target_os = "android")]
pub use android::{delete, delete_all};
