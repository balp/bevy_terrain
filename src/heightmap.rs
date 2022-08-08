use image::io::Reader as ImageReader;

#[derive(Debug)]
pub struct HeightMap {
    pub width: u16,
    pub max_height: f64,
    pub min_height: f64,
    pub map: Vec<f64>,
}

impl HeightMap {
    /// Make a new map from a vector
    pub fn from_vec(height: Vec<f64>) -> HeightMap {
        let width = (height.len() as f64).sqrt() as u16;
        let max = *height.iter()
            .max_by(|a, b|a.total_cmp(b)).unwrap();
        let min = *height.iter()
            .min_by(|a, b|a.total_cmp(b)).unwrap();
        let map = HeightMap {
            width,
            max_height: max,
            min_height: min,
            map: height,
        };
        map
    }

    /// Make a new HeightMap with a linear translation of height
    ///
    /// The new heightmap will have it's height translated between the
    /// new max and min heights.
    ///
    /// # Arguments
    ///
    /// * `max` - The new maps maximal height
    /// * `min` - The new maps minmal height
    ///
    /// # Examples
    ///
    /// Should be a test here
    /// ```
    /// use bevy_terrain::heightmap::HeightMap;
    ///
    /// let map = HeightMap { width: 2, max_height: 1., min_height: -1.,
    ///                        map: vec![ -1., 0., 0., 1., ], };
    /// let new_map = map.transform(1., 0.);
    /// assert_eq!(new_map.width, 2);
    /// ```
    pub fn transform(&self, max: f64, min: f64) -> HeightMap {
        let a = (max - min) / (self.max_height - self.min_height);
        let b = min;
        println!("x = {} * x + {}", a, b);
        let new_heights = self
            .map
            .iter()
            .map(|&x| a * (x - self.min_height) + b)
            .collect();
        let map = HeightMap {
            width: self.width,
            max_height: max,
            min_height: min,
            map: new_heights,
        };
        map
    }
}

pub fn load_height_map(path: &str) -> Result<HeightMap, &str> {
    return if let Ok(reader) = ImageReader::open(path) {
        if let Ok(img) = reader.decode() {
            //dbg!("{}", &img);
            if img.width() == img.height() {
                // Only handle square height maps for now
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
    };
}

#[cfg(test)]
mod tests {
    use crate::heightmap::{load_height_map, HeightMap};
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn terrain_from_file_402() {
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

    #[test]
    fn terrain_from_file_svanesund() {
        if let Ok(height_map) = load_height_map("resources/svanesund_heightmap_1700x1700.png") {
            assert_eq!(height_map.width, 1700);
            assert_eq!(height_map.max_height, 65535.);
            assert_eq!(height_map.min_height, 0.);
            // Sample two random height to make sure something loaded
            assert_eq!(height_map.map[0], 15420.);
            assert_eq!(height_map.map[1000], 4626.);
        } else {
            panic!("Unable to load test image map");
        }
    }

    #[test]
    fn transform_height_3x3_0_8_to_0_1() {
        let _map_3x3 = HeightMap {
            width: 3,
            max_height: 8.,
            min_height: 0.,
            map: vec![0., 1., 2., 3., 4., 5., 6., 7., 8.],
        };
        let expected = vec![0.0, 0.125, 0.25, 0.375, 0.5, 0.625, 0.75, 0.875, 1.0];
        let map = _map_3x3.transform(1., 0.);
        assert_eq!(map.width, 3);
        assert_approx_eq!(map.max_height, 1.);
        assert_approx_eq!(map.min_height, 0.);
        assert_eq!(map.map, expected);
    }

    #[test]
    fn transform_height_3x3_1_9_to_2_4() {
        let _map_3x3 = HeightMap {
            width: 3,
            max_height: 9.,
            min_height: 1.,
            map: vec![1., 2., 3., 4., 5., 6., 7., 8., 9.],
        };
        let expected = vec![2.0, 2.25, 2.5, 2.75, 3.0, 3.25, 3.5, 3.75, 4.0];
        let map = _map_3x3.transform(4., 2.);
        assert_eq!(map.width, 3);
        assert_eq!(map.max_height, 4.);
        assert_eq!(map.min_height, 2.);
        assert_eq!(map.map, expected);
    }

    #[test]
    fn scale_file_terrain() {
        if let Ok(height_map) = load_height_map("resources/test/402.png") {
            let map = height_map.transform(1., 0.);
            assert_eq!(map.width, 256);
            assert_eq!(map.max_height, 1.);
            assert_eq!(map.min_height, 0.);
            // Sample two random height to make sure something loaded
            assert_approx_eq!(map.map[0], 0.4915254237288135);
            assert_approx_eq!(map.map[200], 0.7966101694915254);
        } else {
            panic!("Unable to load test image map");
        }
    }

    #[test]
    fn small_map_2x2() {
        let map = HeightMap::from_vec(vec![1., 2., 3., 4.,]);
        let expected = vec![1., 2., 3., 4.,];
        assert_eq!(map.width, 2);
        assert_eq!(map.max_height, 4.);
        assert_eq!(map.min_height, 1.);
        assert_eq!(map.map, expected);
    }
}
