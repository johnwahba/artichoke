use std::path::PathBuf;

use crate::file::File;
use crate::fs::RUBY_LOAD_PATH;
use crate::ArtichokeError;

#[allow(clippy::module_name_repetitions)]
pub trait LoadSources
where
    Self: Sized,
{
    /// Add a Rust-backed Ruby source file to the virtual filesystem. A stub
    /// Ruby file is added to the filesystem and `require` will dynamically
    /// define Ruby items when invoked via `Kernel#require`.
    ///
    /// If filename is a relative path, the Ruby source is added to the
    /// filesystem relative to [`RUBY_LOAD_PATH`]. If the path is absolute, the
    /// file is placed directly on the filesystem. Anscestor directories are
    /// created automatically.
    fn def_file<T>(
        &self,
        filename: T,
        require: fn(Self) -> Result<(), ArtichokeError>,
    ) -> Result<(), ArtichokeError>
    where
        T: AsRef<str>;

    /// Add a Rust-backed Ruby source file to the virtual filesystem. A stub
    /// Ruby file is added to the filesystem and [`File::require`] will
    /// dynamically define Ruby items when invoked via `Kernel#require`.
    ///
    /// If filename is a relative path, the Ruby source is added to the
    /// filesystem relative to [`RUBY_LOAD_PATH`]. If the path is absolute, the
    /// file is placed directly on the filesystem. Anscestor directories are
    /// created automatically.
    fn def_file_for_type<T, F>(&self, filename: T) -> Result<(), ArtichokeError>
    where
        T: AsRef<str>,
        F: File<Self>;

    /// Add a pure Ruby source file to the virtual filesystem.
    ///
    /// If filename is a relative path, the Ruby source is added to the
    /// filesystem relative to [`RUBY_LOAD_PATH`]. If the path is absolute, the
    /// file is placed directly on the filesystem. Anscestor directories are
    /// created automatically.
    fn def_rb_source_file<T, F>(&self, filename: T, contents: F) -> Result<(), ArtichokeError>
    where
        T: AsRef<str>,
        F: AsRef<[u8]>;

    /// Normalize path of a Ruby source to be relative to `RUBY_LOAD_PATH`
    /// unless the path is absolute.
    fn normalize_source_path<T>(&self, filename: T) -> PathBuf
    where
        T: AsRef<str>,
    {
        let mut path = PathBuf::from(filename.as_ref());
        if path.is_relative() {
            path = PathBuf::from(RUBY_LOAD_PATH).join(filename.as_ref());
        }
        path
    }
}
