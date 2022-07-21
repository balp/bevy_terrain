use image::io::Reader as ImageReader;

#[derive(Debug)]
pub struct HeightMap {
    pub width: u16,
    pub max_height: f64,
    pub min_height: f64,
    pub map: Vec<f64>,
}

pub fn load_height_map(path: &str) -> Result<HeightMap, &str> {
    return if let Ok(reader) = ImageReader::open(path) {
        if let Ok(img) = reader.decode() {
            //dbg!("{}", &img);
            if img.width() == img.height() { // Only handle square height maps for now
                let mut result = HeightMap {
                    width: img.width() as u16,
                    max_height: f64::MIN,
                    min_height: f64::MAX,
                    map: vec![],
                };
                let mut buffer: Vec<f64> = Vec::new();

                let grey_image = img.into_luma16();
                for pixel in grey_image.pixels() {
                    let height = pixel[0] as f64;
                    buffer.push(height);
                    if height > result.max_height {
                        result.max_height = height;
                    }
                    if height < result.min_height {
                        result.min_height = height;
                    }
                }
                result.map = buffer;
                Ok(result)
            } else {
                Err("Not a square image")
            }
        } else {
            Err("Can't read image")
        }
    } else {
        Err("Can't open file")
    }
}

pub fn make_height_map() -> HeightMap {
    // Terrain mesh
    let _map_6x6 = HeightMap {
        width: 6,
        max_height: 0.3,
        min_height: -0.1,
        map: vec![
            -0.1, 0.1, 0.1, 0.1, 0.1, 0.1,
            -0.1, 0.2, 0.2, 0.2, 0.2, 0.1,
            -0.1, 0.2, 0.1, 0.1, 0.3, 0.1,
            -0.1, 0.2, -0.1, 0.1, 0.3, 0.1,
            -0.1, 0.2, 0.2, 0.2, 0.3, 0.1,
            -0.1, 0.1, 0.1, 0.1, 0.1, 0.1,
        ],
    };
    let _map_3x3 = HeightMap {
        width: 3,
        max_height: 8.,
        min_height: 0.,
        map: vec![
            0., 1., 0.,
            3., 4., 5.,
            6., 5., 8.,
        ]
    };
    let _map_2x2 = HeightMap {
        width: 2,
        max_height: 0.2,
        min_height: -0.1,
        map: vec![
            -0.1, 0.1,
            -0.2, 0.2,
        ]
    };
    let _map_4x4 = HeightMap {
        width: 4,
        max_height: 1.0,
        min_height: 0.,
        map: vec![
            0.0, 1.0, 0.0, 1.0,
            1.0, 0.0, 1.0, 0.0,
            0.0, 1.0, 0.0, 1.0,
            1.0, 0.0, 1.0, 0.0,
        ]
    };
    let _map_5x5 = HeightMap {
        width: 5,
        max_height: 0.1,
        min_height: 0.1,
        map: vec![
            0.1, 0.1, 0.1, 0.1, 0.1,
            0.1, 0.1, 0.1, 0.1, 0.1,
            0.1, 0.1, 0.1, 0.1, 0.1,
            0.1, 0.1, 0.1, 0.1, 0.1,
            0.1, 0.1, 0.1, 0.1, 0.1,
        ]
    };
    _map_6x6
}
#[cfg(test)]
mod tests {
    use crate::heightmap::{load_height_map};

    #[test]
    fn terrain_from_file() {
        // let map = load_height_map("resources/test/402.png");
        // assert!(map.is_ok());
        if let Ok(height_map) = load_height_map("resources/test/402.png") {
            assert_eq!(height_map.width, 256);
            assert_eq!(height_map.max_height, 42405.);
            assert_eq!(height_map.min_height, 27242.);
            // Sample two random height to make sure something loaded
            assert_eq!(height_map.map[0], 34695.);
            assert_eq!(height_map.map[200], 39321.);
        } else {
            panic!("Unable to load test image map");

        }
    }

}