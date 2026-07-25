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

/// `morphed_texcoord_color.gltf` contains a single primitive with one morph
/// target that defines `POSITION`, `TEXCOORD_0`/`TEXCOORD_1` and
/// `COLOR_0`/`COLOR_1` displacements, exercising the numbered morphed attribute
/// support added for gltf-rs/gltf#432.
const MORPHED_TEXCOORD_COLOR_GLTF: &str = "tests/morphed_texcoord_color.gltf";

#[test]
fn test_morph_target_tex_coords_and_colors() {
    let (document, buffers, _) = gltf::import(MORPHED_TEXCOORD_COLOR_GLTF).unwrap();

    let mesh = document.meshes().next().unwrap();
    let primitive = mesh.primitives().next().unwrap();

    // The primitive exposes a single morph target.
    let morph_targets: Vec<_> = primitive.morph_targets().collect();
    assert_eq!(morph_targets.len(), 1);
    let morph_target = &morph_targets[0];

    // POSITION displacement is present.
    assert!(morph_target.positions().is_some());
    // Both TEXCOORD_0 and TEXCOORD_1 displacements are present and accessible
    // via the per-set accessor; a set that isn't defined yields None.
    assert!(morph_target.tex_coords(0).is_some());
    assert!(morph_target.tex_coords(1).is_some());
    assert!(morph_target.tex_coords(2).is_none());
    // Both COLOR_0 and COLOR_1 displacements are present.
    assert!(morph_target.colors(0).is_some());
    assert!(morph_target.colors(1).is_some());
    assert!(morph_target.colors(2).is_none());

    // Set-index iterators report the expected sets, ordered by set index.
    let tex_coords_sets: Vec<u32> = morph_target.tex_coords_sets().collect();
    assert_eq!(tex_coords_sets, [0, 1]);
    let colors_sets: Vec<u32> = morph_target.colors_sets().collect();
    assert_eq!(colors_sets, [0, 1]);

    let reader = primitive
        .reader(|buffer: gltf::Buffer| buffers.get(buffer.index()).map(|data| &data.0[..]));

    // The existing read_morph_targets() iterator still yields the P/N/T tuple.
    {
        let mut morph_targets = reader.read_morph_targets();
        let (positions, normals, tangents) = morph_targets.next().unwrap();
        assert!(positions.is_some());
        assert!(normals.is_none());
        assert!(tangents.is_none());
        assert!(morph_targets.next().is_none());
    }

    // read_morph_target_tex_coords(0) yields one item (one morph target),
    // which is Some(ReadTexCoords::F32(..)) for this asset.
    {
        let mut mt_tex_coords = reader.read_morph_target_tex_coords(0);
        let tex_coords_opt = mt_tex_coords.next().unwrap();
        assert!(tex_coords_opt.is_some(), "TEXCOORD_0 displacement expected");
        let tex_coords = tex_coords_opt.unwrap();
        let uv: Vec<[f32; 2]> = tex_coords.into_f32().collect();
        assert_eq!(
            uv,
            [[0.05, 0.05], [0.1, 0.0], [0.0, 0.1]],
            "morph target TEXCOORD_0 displacements"
        );
        assert!(mt_tex_coords.next().is_none());
    }

    // read_morph_target_tex_coords(1) returns the *distinct* set-1 deltas,
    // proving the reader dispatches by set index and not just set 0.
    {
        let mut mt_tex_coords = reader.read_morph_target_tex_coords(1);
        let tex_coords_opt = mt_tex_coords.next().unwrap();
        assert!(tex_coords_opt.is_some(), "TEXCOORD_1 displacement expected");
        let tex_coords = tex_coords_opt.unwrap();
        let uv: Vec<[f32; 2]> = tex_coords.into_f32().collect();
        assert_eq!(
            uv,
            [[0.02, 0.02], [0.03, 0.0], [0.0, 0.03]],
            "morph target TEXCOORD_1 displacements"
        );
        assert!(mt_tex_coords.next().is_none());
    }

    // Requesting a set that doesn't exist on any morph target still iterates
    // once (one morph target), yielding None for that morph target.
    {
        let mut mt_tex_coords_missing = reader.read_morph_target_tex_coords(7);
        assert!(mt_tex_coords_missing.next().unwrap().is_none());
        assert!(mt_tex_coords_missing.next().is_none());
    }

    // read_morph_target_colors(0) yields one item, which is
    // Some(ReadColors::RgbF32(..)) for this asset.
    {
        let mut mt_colors = reader.read_morph_target_colors(0);
        let colors_opt = mt_colors.next().unwrap();
        assert!(colors_opt.is_some(), "COLOR_0 displacement expected");
        let colors = colors_opt.unwrap();
        let rgb: Vec<[f32; 3]> = colors.into_rgb_f32().collect();
        assert_eq!(
            rgb,
            [[0.1, 0.0, 0.0], [0.0, 0.1, 0.0], [0.0, 0.0, 0.1]],
            "morph target COLOR_0 displacements"
        );
        assert!(mt_colors.next().is_none());
    }

    // read_morph_target_colors(1) returns the distinct set-1 color deltas.
    {
        let mut mt_colors = reader.read_morph_target_colors(1);
        let colors_opt = mt_colors.next().unwrap();
        assert!(colors_opt.is_some(), "COLOR_1 displacement expected");
        let colors = colors_opt.unwrap();
        let rgb: Vec<[f32; 3]> = colors.into_rgb_f32().collect();
        assert_eq!(
            rgb,
            [[0.02, 0.0, 0.0], [0.0, 0.02, 0.0], [0.0, 0.0, 0.02]],
            "morph target COLOR_1 displacements"
        );
        assert!(mt_colors.next().is_none());
    }
}
