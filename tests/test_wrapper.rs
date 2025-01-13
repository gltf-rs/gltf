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
    "glTF-Sample-Assets/Models/SimpleSparseAccessor/glTF-Embedded/SimpleSparseAccessor.gltf";

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

#[test]
fn test_sparse_accessor_with_base_buffer_view_yield_all_values() {
    let (document, buffers, _) = gltf::import(SIMPLE_SPARSE_ACCESSOR_GLTF).unwrap();

    let mesh = document.meshes().next().unwrap();
    let primitive = mesh.primitives().next().unwrap();
    let reader = primitive
        .reader(|buffer: gltf::Buffer| buffers.get(buffer.index()).map(|data| &data.0[..]));
    let positions: Vec<[f32; 3]> = reader.read_positions().unwrap().collect::<Vec<_>>();

    const EXPECTED_POSITIONS: [[f32; 3]; 14] = [
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [2.0, 0.0, 0.0],
        [3.0, 0.0, 0.0],
        [4.0, 0.0, 0.0],
        [5.0, 0.0, 0.0],
        [6.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        [1.0, 2.0, 0.0], // Sparse value #1
        [2.0, 1.0, 0.0],
        [3.0, 3.0, 0.0], // Sparse value #2
        [4.0, 1.0, 0.0],
        [5.0, 4.0, 0.0], // Sparse value #3
        [6.0, 1.0, 0.0],
    ];
    assert_eq!(positions.len(), EXPECTED_POSITIONS.len());
    for (i, p) in positions.iter().enumerate() {
        for (j, q) in p.iter().enumerate() {
            assert_eq!(q - EXPECTED_POSITIONS[i][j], 0.0);
        }
    }
}

/// "box_sparse.gltf" contains an animation with a sampler with output of two values.
/// The values are specified with a sparse accessor that is missing a base `bufferView` field.
/// Which means that each value in it will be 0.0, except the values contained in the sparse
/// buffer view itself. In this case the second value is read from the sparse accessor (1.0),
/// while the first is left at the default zero.
const BOX_SPARSE_GLTF: &str = "tests/box_sparse.gltf";

#[test]
fn test_sparse_accessor_without_base_buffer_view_yield_exact_size_hints() {
    let (document, buffers, _) = gltf::import(BOX_SPARSE_GLTF).unwrap();

    let animation = document.animations().next().unwrap();
    let sampler = animation.samplers().next().unwrap();
    let output_accessor = sampler.output();
    let mut outputs_iter =
        gltf::accessor::Iter::<f32>::new(output_accessor, |buffer: gltf::Buffer| {
            buffers.get(buffer.index()).map(|data| &data.0[..])
        })
        .unwrap();

    const EXPECTED_OUTPUT_COUNT: usize = 2;
    for i in (0..=EXPECTED_OUTPUT_COUNT).rev() {
        assert_eq!(outputs_iter.size_hint(), (i, Some(i)));
        outputs_iter.next();
    }
}

#[test]
fn test_sparse_accessor_without_base_buffer_view_yield_all_values() {
    let (document, buffers, _) = gltf::import(BOX_SPARSE_GLTF).unwrap();

    let animation = document.animations().next().unwrap();
    let sampler = animation.samplers().next().unwrap();
    let output_accessor = sampler.output();
    let output_iter = gltf::accessor::Iter::<f32>::new(output_accessor, |buffer: gltf::Buffer| {
        buffers.get(buffer.index()).map(|data| &data.0[..])
    })
    .unwrap();
    let outputs = output_iter.collect::<Vec<_>>();

    const EXPECTED_OUTPUTS: [f32; 2] = [0.0, 1.0];
    assert_eq!(outputs.len(), EXPECTED_OUTPUTS.len());
    for (i, o) in outputs.iter().enumerate() {
        assert_eq!(o - EXPECTED_OUTPUTS[i], 0.0);
    }
}
