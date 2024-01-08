use std::io::Read;
use std::{fs, io};

use gltf::mesh::Bounds;

#[test]
fn test_accessor_bounds() {
    // file derived from minimal.gltf with changed min/max values
    let file = fs::File::open("tests/minimal_accessor_min_max.gltf").unwrap();
    let mut reader = io::BufReader::new(file);
    let mut buffer = vec![];
    reader.read_to_end(&mut buffer).unwrap();
    let gltf = gltf::Gltf::from_slice(&buffer).unwrap();
    let mesh = &gltf.meshes().next().unwrap();
    let prim = mesh.primitives().next().unwrap();
    let bounds = prim.bounding_box();
    assert_eq!(
        bounds,
        Bounds {
            min: [-0.03, -0.04, -0.05],
            max: [1.0, 1.01, 0.02]
        }
    );
}

/// "SimpleSparseAccessor.gltf" contains positions specified with a sparse accessor.
/// The accessor use a base `bufferView` that contains 14 `Vec3`s and the sparse
/// section overwrites 3 of these with other values when read.
const SIMPLE_SPARSE_ACCESSOR_GLTF: &str =
    "glTF-Sample-Models/2.0/SimpleSparseAccessor/glTF-Embedded/SimpleSparseAccessor.gltf";

#[test]
fn test_sparse_accessor_with_base_buffer_view_yield_exact_size_hints() {
    let (document, buffers, _) = gltf::import(SIMPLE_SPARSE_ACCESSOR_GLTF).unwrap();

    let mesh = document.meshes().next().unwrap();
    let primitive = mesh.primitives().next().unwrap();
    let reader = primitive
        .reader(|buffer: gltf::Buffer| buffers.get(buffer.index()).map(|data| &data.0[..]));
    let mut positions = reader.read_positions().unwrap();

    const EXPECTED_POSITION_COUNT: usize = 14;
    for i in (0..=EXPECTED_POSITION_COUNT).rev() {
        assert_eq!(positions.size_hint(), (i, Some(i)));
        positions.next();
    }
}
