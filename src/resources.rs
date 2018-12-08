use std::ffi;
use std::fs;
use std::io::{self, Read};
use std::path::{Path, PathBuf};

use image;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "I/O error")]
    IO(#[cause] io::Error),

    #[fail(display = "Failed to read CString from file that contains 0")]
    FileContainsNil,

    #[fail(display = "Unable to locate executable")]
    UnableToLocateExe,

    #[fail(display = "Unable to load image")]
    UnableToLoadImage(image::ImageError),
}

impl From<io::Error> for Error {
    fn from(other: io::Error) -> Self {
        Error::IO(other)
    }
}

pub struct Resources {
    root_path: PathBuf,
}

impl Resources {

    pub fn from_relative_exe_path(rel_path: &Path) -> Result<Resources, Error> {
        let exe_file_name = ::std::env::current_exe()
            .map_err(|_| Error::UnableToLocateExe)?;

        let exe_path = exe_file_name.parent()
            .ok_or(Error::UnableToLocateExe)?;

        Ok(Resources {
            root_path: exe_path.join(rel_path)
        })
    }

    pub fn load_cstring(&self, resource_name: &str) -> Result<ffi::CString, Error> {
        let mut file = fs::File::open(
            resource_name_to_path(&self.root_path,resource_name)
        )?;

        let mut buffer: Vec<u8> = Vec::with_capacity(
            file.metadata()?.len() as usize + 1
        );
        file.read_to_end(&mut buffer)?;

        // check for null byte
        if buffer.iter().find(|i| **i == 0).is_some() {
            return Err(Error::FileContainsNil);
        }
        
        Ok(unsafe { ffi::CString::from_vec_unchecked(buffer) })
    }

    pub fn load_image(&self, resource_name: &str) -> Result<image::DynamicImage, image::ImageError> {
        let path = resource_name_to_path(&self.root_path,resource_name);
        image::open(path)
    }
}

fn resource_name_to_path(root_dir: &Path, location: &str) -> PathBuf {
    let mut path: PathBuf = root_dir.into();

    for part in location.split("/") {
        path = path.join(part);
    }

    path
}
