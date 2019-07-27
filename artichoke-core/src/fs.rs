//! Artichoke virtual filesystem used for storing Ruby sources.

use artichoke_vfs::{FakeFileSystem, FileSystem};
use std::path::Path;

use crate::ArtichokeError;

pub const RUBY_LOAD_PATH: &str = "/src/lib";

pub type RequireFunc<I> = fn(I) -> Result<(), ArtichokeError>;

/// Virtual filesystem that wraps a [`artichoke_vfs`] [`FakeFileSystem`].
pub struct Filesystem<I: Clone> {
    fs: FakeFileSystem<Metadata<I>>,
}

impl<I: Clone> Filesystem<I> {
    /// Create a new in memory virtual filesystem.
    ///
    /// Creates a directory at [`RUBY_LOAD_PATH`] for storing Ruby source files.
    /// This path is searched by
    /// [`Kernel::require`](crate::extn::core::kernel::Kernel::require) and
    /// [`Kernel::require_relative`](crate::extn::core::kernel::Kernel::require_relative).
    pub fn new() -> Result<Self, ArtichokeError> {
        let fs = FakeFileSystem::new();
        fs.create_dir_all(RUBY_LOAD_PATH)
            .map_err(ArtichokeError::Vfs)?;
        Ok(Self { fs })
    }

    pub fn create_dir_all<P: AsRef<Path>>(&self, path: P) -> Result<(), ArtichokeError> {
        self.fs
            .create_dir_all(path.as_ref())
            .map_err(ArtichokeError::Vfs)
    }

    pub fn is_file<P: AsRef<Path>>(&self, path: P) -> bool {
        self.fs.is_file(path.as_ref())
    }

    pub fn read_file<P: AsRef<Path>>(&self, path: P) -> Result<Vec<u8>, ArtichokeError> {
        self.fs
            .read_file(path.as_ref())
            .map_err(ArtichokeError::Vfs)
    }

    pub fn write_file<P, B>(&self, path: P, buf: B) -> Result<(), ArtichokeError>
    where
        P: AsRef<Path>,
        B: AsRef<[u8]>,
    {
        self.fs
            .write_file(path.as_ref(), buf.as_ref())
            .map_err(ArtichokeError::Vfs)
    }

    pub fn set_metadata<P: AsRef<Path>>(
        &self,
        path: P,
        metadata: Metadata<I>,
    ) -> Result<(), ArtichokeError> {
        self.fs
            .set_metadata(path.as_ref(), metadata)
            .map_err(ArtichokeError::Vfs)
    }

    pub fn metadata<P: AsRef<Path>>(&self, path: P) -> Option<Metadata<I>> {
        self.fs.metadata(path.as_ref())
    }
}

#[derive(Clone, Debug)]
pub struct Metadata<I: Clone> {
    pub require: Option<RequireFunc<I>>,
    already_required: bool,
}

impl<I: Clone> Metadata<I> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn mark_required(self) -> Self {
        Self {
            require: self.require,
            already_required: true,
        }
    }

    pub fn is_already_required(&self) -> bool {
        self.already_required
    }
}

impl<I: Clone> Default for Metadata<I> {
    fn default() -> Self {
        Self {
            require: None,
            already_required: false,
        }
    }
}
