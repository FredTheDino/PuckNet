use std::path::PathBuf;
use std::time::SystemTime;

/// A file on disk that has been loaded.
#[derive(Clone, Debug)]
pub struct LoadedFile {
    pub file: PathBuf,
    pub last_modified: SystemTime,
}

impl LoadedFile {
    pub fn new(file: PathBuf) -> (Self, Vec<u8>) {
        let last_modified = std::fs::metadata(&file)
            .expect(&format!("asset file {} not found", file.display()))
            .modified()
            .ok()
            .unwrap_or_else(SystemTime::now);
        let bytes = std::fs::read(&file)
            .expect(&format!("asset file {} not found", file.display()));
        (
            Self {
                file,
                last_modified,
            },
            bytes
        )
    }

    /// Return the file data if it has been modified since it was last read.
    ///
    /// Modification is checked using [std::fs::metadata] and as such might not work on all
    /// operating systems.
    pub fn reload(&mut self) -> Option<Vec<u8>> {
        match std::fs::metadata(&self.file).ok().map(|m| m.modified().ok()).flatten() {
            Some(last_modified) if last_modified != self.last_modified => {
                let bytes = std::fs::read(&self.file).ok();
                if bytes.is_some() {
                    self.last_modified = last_modified;
                }
                bytes
            }
            _ => None,
        }
    }
}

/// A marker type for the unit pixels.
pub type Pixels = usize;

#[derive(Clone, Debug)]
pub struct Image {
    pub width: usize,
    pub height: usize,
    pub texture_data: Vec<u8>,
    pub data: LoadedFile,
}

impl Image {
    pub fn new(file: PathBuf) -> Self {
        let (data, bytes) = LoadedFile::new(file);
        let mut ret = Self {
            width: 0,
            height: 0,
            texture_data: Vec::new(),
            data,
        };
        ret.load_data(bytes);
        ret
    }

    pub fn reload(&mut self) -> bool {
        if let Some(bytes) = self.data.reload() {
            self.load_data(bytes);
            true
        } else {
            false
        }
    }

    fn load_data(&mut self, bytes: Vec<u8>) {
        let mut w: i32 = 0;
        let mut h: i32 = 0;
        let mut comp: i32 = 4;
        // SAFETY: stb_load_from_memory either succeeds or returns a null pointer
        unsafe {
            use stb_image::stb_image::bindgen::*;
            stbi_set_flip_vertically_on_load(1);
            let stb_image = stbi_load_from_memory(
                bytes.as_ptr(),
                bytes.len() as i32,
                &mut w,
                &mut h,
                &mut comp,
                4,
            );
            self.texture_data = Vec::from_raw_parts(stb_image as *mut u8, (w * h * 4) as usize, (w * h * 4) as usize);
        }
        self.width = w as usize;
        self.height = h as usize;
    }
}
