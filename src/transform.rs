extern crate image;
use image::io::Reader as ImageReader;
use std::io;
use std::error::Error;
use std::option::Option;
use std::path::{Path, PathBuf};
use std::fmt::{Display, Formatter};
use image::ImageError;

#[derive(Debug)]
enum TransformError {
    Open(io::Error),
    Decode(ImageError),
    Save(ImageError),
    NoFileName,
    NoParentDir,
}

type TransformResult<T> = Result<T, TransformError>;

impl Display for TransformError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match *self {
            TransformError::Open(ref e) => e.fmt(f),
            TransformError::Decode(ref e) => e.fmt(f),
            TransformError::Save(ref e) => e.fmt(f),
            TransformError::NoFileName => write!(f, "supplied path has no file_name"),
            TransformError::NoParentDir => write!(f, "supplied path has no parent directory"),
        }
    }
}

impl Error for TransformError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            TransformError::Open(ref e) => Some(e),
            TransformError::Decode(ref e) => Some(e),
            TransformError::Save(ref e) => Some(e),
            TransformError::NoFileName => None,
            TransformError::NoParentDir => None,
        }
    }
}

/// Takes an image at the provided path and creates a thumbnail of it. The
/// thumbnail is saved in same path with the same name as the original file,
/// but in a new folder named .hc_thumbs.
fn make_thumbnail(path: &Path) -> TransformResult<()> {
    let filename = path.file_name().ok_or(TransformError::NoFileName)?;
    let parent = path.parent().ok_or(TransformError::NoParentDir)?;
    let thumbnail_path = parent.join(".hc_thumbs").with_file_name(filename).with_extension("jpg");
    ImageReader::open(path)
        .map_err(TransformError::Open)?
        .decode()
        .map_err(TransformError::Decode)?
        .thumbnail(100, 100)
        .save(thumbnail_path)
        .map_err(TransformError::Save)
}
