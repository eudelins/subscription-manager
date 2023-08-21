use std::{
    io::{self, ErrorKind},
    str::FromStr,
};

pub enum UploadType {
    Brand = 0,
    Category = 1,
}

impl FromStr for UploadType {
    type Err = io::Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "brands" => Ok(Self::Brand),
            "categories" => Ok(Self::Category),
            _ => Err(io::Error::new(
                ErrorKind::InvalidInput,
                "Invalid upload directory",
            )),
        }
    }
}

impl UploadType {
    pub fn into_upload_dir(&self) -> &'static str {
        match self {
            Self::Brand => {
                if cfg!(test) {
                    "test/brands"
                } else {
                    "brands"
                }
            }
            Self::Category => {
                if cfg!(test) {
                    "test/categories"
                } else {
                    "categories"
                }
            }
        }
    }
}
