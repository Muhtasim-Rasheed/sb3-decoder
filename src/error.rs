/// Represents errors that can occur during the decoding process.
///
/// This enum encapsulates errors from ZIP handling, JSON parsing, and I/O operations. Each variant
/// context about the specific error encountered.
type BoxError = Box<dyn std::error::Error + Send + Sync>;

#[derive(thiserror::Error, Debug)]
#[non_exhaustive]
pub enum Error {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    // Archive
    #[error("corrupt .sb3 archive")]
    ZipCorrupt(#[source] BoxError),

    #[error("unsupported .sb3 archive")]
    ZipUnsupported(#[source] BoxError),

    #[error("missing archive entry: {name}")]
    ZipMissingEntry { name: String },

    #[error("zip entry too large: {name} exceeds {limit} bytes")]
    ZipEntryTooLarge { name: String, limit: u64 },

    // project.json
    #[error("invalid JSON syntax at line {line}, column {column}")]
    JsonSyntax {
        line: usize,
        column: usize,
        #[source]
        source: BoxError,
    },

    #[error("project.json has an invalid structure at line {line}, column {column}: {source}")]
    JsonData {
        line: usize,
        column: usize,
        #[source]
        source: BoxError,
    },

    #[error("project.json ended unexpectedly")]
    JsonUnexpectedEof(#[source] BoxError),

    // Costumes
    #[cfg(feature = "costumes")]
    #[error("corrupt costume data")]
    CostumeCorrupt(#[source] BoxError),

    #[cfg(feature = "costumes")]
    #[error("unsupported costume data")]
    CostumeUnsupported(#[source] BoxError),

    #[cfg(feature = "costumes")]
    #[error("costume exceeds size limits")]
    CostumeTooLarge(#[source] BoxError),

    #[cfg(feature = "costumes")]
    #[error("invalid costume size: {w}x{h}")]
    InvalidCostumeSize { w: u32, h: u32 },

    #[error("unknown costume format: {0}")]
    UnknownCostumeFormat(String),

    // Sounds
    #[cfg(feature = "sounds")]
    #[error("corrupt sound data")]
    SoundCorrupt(#[source] BoxError),

    #[cfg(feature = "sounds")]
    #[error("unsupported sound data")]
    SoundUnsupported(#[source] BoxError),

    #[cfg(feature = "sounds")]
    #[error("sound contains no audio")]
    SoundEmpty,

    #[error("unknown sound format: {0}")]
    UnknownSoundFormat(String),

    // Model
    #[error("invalid rotation style: {0}")]
    InvalidRotationStyle(String),

    #[error("invalid loudness/timer selection: {0}")]
    InvalidLoudnessTimer(String),

    #[error("invalid graphic effect: {0}")]
    InvalidGraphicEffect(String),

    #[error("invalid front/back selection: {0}")]
    InvalidFrontBack(String),

    #[error("invalid forward/backward selection: {0}")]
    InvalidForwardBackward(String),

    #[error("invalid sound effect: {0}")]
    InvalidSoundEffect(String),

    #[error("invalid stop option: {0}")]
    InvalidStop(String),

    #[error("invalid draggable/not draggable selection: {0}")]
    InvalidDraggable(String),

    #[error("invalid number/name selection: {0}")]
    InvalidNumberName(String),

    #[error("invalid current time selection: {0}")]
    InvalidCurrent(String),

    #[error("invalid math function selection: {0}")]
    InvalidMathFunc(String),

    #[error("invalid block mutation")]
    InvalidMutation,

    #[error("more than one stage found")]
    MultipleStages,

    #[error("no stage found")]
    NoStage,

    #[error("missing block: {id}")]
    MissingBlock { id: String },

    #[error("block depth exceeded: block depth went past {limit}")]
    DepthExceeded { limit: usize },

    #[error("block cycle: cycle at {id}")]
    BlockCycle { id: String },

    #[error("unknown opcode: {0}")]
    UnknownOpcode(String),

    #[error("invalid primitive: {0}")]
    InvalidPrimitive(&'static str),

    #[error("missing rotation style: no rotation style in target {0}")]
    MissingRotationStyle(String),

    #[error("missing field {field}")]
    MissingField { field: String },

    #[error("missing id part of field {field}")]
    MissingIDOfField { field: String },

    #[error("missing block mutation")]
    MissingMutation,
}

impl Error {
    pub(crate) fn json(e: serde_json::Error) -> Self {
        use serde_json::error::Category;
        let (line, column) = (e.line(), e.column());
        match e.classify() {
            Category::Syntax => Error::JsonSyntax {
                line,
                column,
                source: Box::new(e),
            },
            Category::Eof => Error::JsonUnexpectedEof(Box::new(e)),
            _ => Error::JsonData {
                line,
                column,
                source: Box::new(e),
            },
        }
    }

    pub(crate) fn zip(e: zip::result::ZipError, entry: &str) -> Self {
        use zip::result::ZipError as Z;
        match e {
            Z::Io(e) => Error::Io(e),
            Z::FileNotFound => Error::ZipMissingEntry {
                name: entry.to_owned(),
            },
            Z::UnsupportedArchive(_) => Error::ZipUnsupported(Box::new(e)),
            e => Error::ZipCorrupt(Box::new(e)),
        }
    }
}

#[cfg(feature = "costumes")]
impl Error {
    pub(crate) fn image(e: image::ImageError) -> Self {
        use image::ImageError as I;
        match e {
            I::Unsupported(_) => Error::CostumeUnsupported(Box::new(e)),
            I::Limits(_) => Error::CostumeTooLarge(Box::new(e)),
            _ => Error::CostumeCorrupt(Box::new(e)),
        }
    }

    pub(crate) fn usvg(e: resvg::usvg::Error) -> Self {
        Error::CostumeCorrupt(Box::new(e))
    }
}

#[cfg(feature = "sounds")]
impl Error {
    pub(crate) fn hound(e: hound::Error) -> Self {
        use hound::Error as H;
        match e {
            H::Unsupported => Error::SoundUnsupported(Box::new(e)),
            H::IoError(io_err) => Error::Io(io_err),
            _ => Error::SoundCorrupt(Box::new(e)),
        }
    }

    pub(crate) fn nanomp3(e: nanomp3::Error) -> Self {
        use nanomp3::Error as N;
        match e {
            N::Io(io_err) => Error::Io(io_err),
            _ => Error::SoundCorrupt(Box::new(e)),
        }
    }
}
