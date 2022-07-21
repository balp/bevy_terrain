use crate::{HeightMap, Indices, Mesh, PrimitiveTopology};

pub fn create_simple_terrain(height_map: &HeightMap) -> Mesh {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    let heights = height_map;
    let size = height_map.width;
    let mut vertices: Vec<[f32; 3]> = Vec::with_capacity(heights.map.len());
    let mut normals: Vec<[f32; 3]> = Vec::with_capacity(heights.map.len());
    let mut uvs: Vec<[f32; 2]> = Vec::with_capacity(heights.map.len());
    let mut indices_vector: Vec<u16> = Vec::new();

    let mut x = 0;
    let mut z = 0;

    for height in &heights.map {
        vertices.push([x as f32, *height as f32, z as f32]);
        normals.push([0f32, 0f32, 1f32]);
        uvs.push([x as f32, z as f32]);
        x += 1;
        if x == size {
            x = 0;
            z += 1;
        }
    }

    let l = (heights.map.len() - size as usize - 1) as u16;
    for a in 0..l {
        if !(a % size as u16 == size as u16 - 1) {
            indices_vector.push(a);
            indices_vector.push(a + 1u16 + (size as u16));
            indices_vector.push(a + 1u16);

            indices_vector.push(a);
            indices_vector.push(a + 0u16 + (size as u16));
            indices_vector.push(a + 1u16 + (size as u16));
        }
    }
    let indices = Indices::U16(indices_vector);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.set_indices(Some(indices));

    mesh
}


#[cfg(test)]
mod tests {
    use crate::terrain::create_simple_terrain;
    use bevy::prelude::*;
    use bevy::render::mesh::Indices::U16;
    use bevy::render::mesh::VertexAttributeValues;
    use crate::heightmap::{HeightMap};


    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }


    #[test]
    /// Make sure a simple 2x2 height map generates correct
    ///
    /// The test sets up a height map like this:
    ///      0  1
    ///      -  -
    ///  0|  0  1
    ///  1|  2  3
    ///
    /// the verifies the resulting positions and indices for the mesh
    fn make_simple_terrain_2x2() {
        let _map_2x2 = HeightMap {
            width: 2,
            max_height: 3.,
            min_height: 0.,
            map: vec![
                0.,
                1.,
                2.,
                3.,
            ],
        };
        let mesh = create_simple_terrain(&_map_2x2);
        assert_eq!(mesh.count_vertices(), 4);
        let positions = mesh.attribute(Mesh::ATTRIBUTE_POSITION).unwrap();
        dbg!("{}", positions);
        let expected_pos = VertexAttributeValues::from(vec![
            [0., 0., 0.],
            [1., 1., 0.],
            [0., 2., 1.],
            [1., 3., 1.],
        ]);
        assert_eq!(positions.get_bytes(), expected_pos.get_bytes());

        if let U16(bytes) = mesh.indices().unwrap() {
            dbg!("{}", bytes);
            assert_eq!(*bytes, vec![
                0u16, 3u16, 1u16,
                0u16, 2u16, 3u16,
            ]);
        }
    }

    #[test]
    /// Make sure a simple 3x3 height map generates correct
    ///
    /// The test sets up a height map like this:
    ///      0  1  2
    ///      -  -  -
    ///  0|  0  1  2
    ///  1|  3  4  5
    ///  2|  6  7  8
    ///
    /// the verifies the resulting positions and indices for the mesh
    fn make_simple_terrain_3x3() {
        let _map_3x3 = HeightMap {
            width: 3,
            max_height: 8.,
            min_height: 0.,
            map: vec![
                0., 1., 2.,
                3., 4., 5.,
                6., 7., 8.,
            ],
        };
        let mesh = create_simple_terrain(&_map_3x3);
        assert_eq!(mesh.count_vertices(), 9);
        let positions = mesh.attribute(Mesh::ATTRIBUTE_POSITION).unwrap();
        dbg!("{}", positions);
        let expected_pos = VertexAttributeValues::from(vec![
            [0., 0., 0.],
            [1., 1., 0.],
            [2., 2., 0.],
            [0., 3., 1.],
            [1., 4., 1.],
            [2., 5., 1.],
            [0., 6., 2.],
            [1., 7., 2.],
            [2., 8., 2.],
        ]);
        assert_eq!(positions.get_bytes(), expected_pos.get_bytes());

        if let U16(bytes) = mesh.indices().unwrap() {
            dbg!("{}", bytes);
            assert_eq!(*bytes, vec![0u16, 4u16, 1u16,
                                    0u16, 3u16, 4u16,
                                    1u16, 5u16, 2u16,
                                    1u16, 4u16, 5u16,
                                    3u16, 7u16, 4u16,
                                    3u16, 6u16, 7u16,
                                    4u16, 8u16, 5u16,
                                    4u16, 7u16, 8u16,
            ]);
        }
    }
}