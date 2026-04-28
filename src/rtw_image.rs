use std::path::PathBuf;

pub struct RtwImage {
    width: u32,
    height: u32,
    data: Vec<u8>,
}

impl RtwImage {
    pub fn new(filename: &str) -> Self {
        let path = find_image_file(filename);

        match path.and_then(|p| image::open(p).ok()) {
            Some(img) => {
                let rgb = img.to_rgb8();
                let (width, height) = rgb.dimensions();

                Self {
                    width,
                    height,
                    data: rgb.into_raw(),
                }
            }
            None => {
                eprintln!("ERROR: Could not load image file '{filename}'.");

                Self {
                    width: 0,
                    height: 0,
                    data: Vec::new(),
                }
            }
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn pixel_data(&self, x: u32, y: u32) -> [u8; 3] {
        if self.data.is_empty() || self.width == 0 || self.height == 0 {
            return [255, 0, 255];
        }

        let x = x.min(self.width - 1);
        let y = y.min(self.height - 1);

        let index = ((y * self.width + x) * 3) as usize;

        [self.data[index], self.data[index + 1], self.data[index + 2]]
    }
}

fn find_image_file(filename: &str) -> Option<PathBuf> {
    if let Ok(dir) = std::env::var("RTW_IMAGES") {
        let path = PathBuf::from(dir).join(filename);
        if path.exists() {
            return Some(path);
        }
    }

    let candidates = [
        filename.to_string(),
        format!("images/{filename}"),
        format!("../images/{filename}"),
        format!("../../images/{filename}"),
        format!("../../../images/{filename}"),
        format!("../../../../images/{filename}"),
        format!("../../../../../images/{filename}"),
        format!("../../../../../../images/{filename}"),
    ];

    candidates
        .into_iter()
        .map(PathBuf::from)
        .find(|path| path.exists())
}
