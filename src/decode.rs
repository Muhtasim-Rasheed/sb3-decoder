use std::{
    fs::File,
    io::{BufReader, Cursor, Read, Seek},
    path::Path,
};

use crate::{Error, model::Project, raw};

mod block;
mod project;
mod target;

const DEFAULT_MAX_PROJECT_JSON: u64 = 5 * 1024 * 1024;
const DEFAULT_MAX_ASSET: u64 = 10 * 1024 * 1024;
const DEFAULT_MAX_DEPTH: usize = 256;
const DEFAULT_ALLOW_UNKNOWN_BLOCKS: bool = false;

struct DecodeCtx<'a> {
    max_depth: usize,
    allow_unknown_blocks: bool,
    asset_loader: &'a mut dyn FnMut(&str) -> Result<Vec<u8>, Error>,
}

impl<'a> DecodeCtx<'a> {
    fn load_asset(&mut self, name: &str) -> Result<Vec<u8>, Error> {
        (self.asset_loader)(name)
    }
}

/// Decodes a project from a ZIP archive containing a `project.json` entry.
///
/// The decoder supports reading from files or in-memory byte slices and limits the size of the
/// project JSON to protect against oversized entries. Use [`Decoder::max_project_json`] to
/// configure the limit.
///
/// # Examples
///
/// ```no_run
/// # use sb3_decoder::prelude::*;
/// let bytes = std::fs::read("project.zip")?;
/// let mut decoder = Decoder::from_bytes(&bytes)?;
/// let project = decoder.decode()?;
/// # Ok::<(), sb3_decoder::Error>(())
/// ```
pub struct Decoder<R> {
    /// Maximum number of bytes allowed when reading `project.json`.
    ///
    /// Defaults to 5 MiB. If the entry exceeds this limit, decoding returns an error.
    pub max_project_json: u64,

    /// Maximum number of bytes allowed when reading an asset.
    ///
    /// Defaults to 10 MiB. If the entry exceeds this limit, decoding returns an error.
    pub max_asset: u64,

    /// Maximum nesting of C-blocks and reporter inputs.
    ///
    /// Defaults to 256. If the block nesting exceeds this limit, decoding returns an error.
    pub max_depth: usize,

    /// Decides if the decoder returns an error or outputs an [`crate::model::code::Action::Unknown`]
    /// or an [`crate::model::code::Reporter::Unknown`] when met with an unknown block opcode.
    ///
    /// Defaults to false.
    pub allow_unknown_blocks: bool,

    archive: zip::ZipArchive<R>,
}

impl<R: Read + Seek> Decoder<R> {
    /// Creates a decoder from a readable and seekable ZIP archive.
    ///
    /// The JSON size limit defaults to 5 MiB. Returns an error if the archive cannot be opened.
    ///
    /// # Examples
    ///
    /// ```
    /// # use sb3_decoder::prelude::*;
    /// # use std::io::Cursor;
    /// # let zip_bytes: &[u8] = &[];
    /// let decoder = Decoder::new(Cursor::new(zip_bytes));
    /// ```
    pub fn new(reader: R) -> Result<Self, Error> {
        Ok(Self {
            max_project_json: DEFAULT_MAX_PROJECT_JSON,
            max_asset: DEFAULT_MAX_ASSET,
            max_depth: DEFAULT_MAX_DEPTH,
            allow_unknown_blocks: DEFAULT_ALLOW_UNKNOWN_BLOCKS,

            archive: zip::ZipArchive::new(reader).map_err(|e| Error::zip(e, ""))?,
        })
    }

    /// Sets the maximum size of the `project.json` entry in bytes. Defaults to 5 MiB.
    pub fn max_project_json(mut self, bytes: u64) -> Self {
        self.max_project_json = bytes;
        self
    }

    /// Sets the maximum size of the asset entries in bytes. Defaults to 10 MiB.
    pub fn max_asset(mut self, bytes: u64) -> Self {
        self.max_asset = bytes;
        self
    }

    /// Sets the maximum nesting of C-blocks and reporter inputs. Defaults to 256.
    pub fn max_depth(mut self, depth: usize) -> Self {
        self.max_depth = depth;
        self
    }

    /// Sets whether to allow unknown blocks. Defaults to false.
    pub fn allow_unknown_blocks(mut self, allow: bool) -> Self {
        self.allow_unknown_blocks = allow;
        self
    }

    /// Decodes the Scratch project.
    pub fn decode(mut self) -> Result<Project, Error> {
        let json = self.read_entry("project.json", self.max_project_json)?;
        let raw: raw::Project = serde_json::from_slice(&json).map_err(Error::json)?;
        project::decode_project(
            raw,
            DecodeCtx {
                max_depth: self.max_depth,
                allow_unknown_blocks: self.allow_unknown_blocks,
                asset_loader: &mut |n| self.read_entry(n, self.max_asset),
            },
        )
    }

    fn read_entry(&mut self, name: &str, limit: u64) -> Result<Vec<u8>, Error> {
        let file = self
            .archive
            .by_name(name)
            .map_err(|e| Error::zip(e, name))?;
        let mut buf = Vec::new();
        file.take(limit + 1).read_to_end(&mut buf)?;
        if buf.len() as u64 > limit {
            return Err(Error::ZipEntryTooLarge {
                name: name.to_string(),
                limit,
            });
        }
        Ok(buf)
    }
}

impl Decoder<BufReader<File>> {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        Self::new(BufReader::new(File::open(path)?))
    }
}

impl<'a> Decoder<Cursor<&'a [u8]>> {
    pub fn from_bytes(bytes: &'a [u8]) -> Result<Self, Error> {
        Self::new(Cursor::new(bytes))
    }
}
