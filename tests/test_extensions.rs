//! Tests for glTF extensions: KHR_mesh_quantization, EXT_meshopt_compression, KHR_texture_transform

const SAMPLE_MODELS_DIR: &str = "";

/// Test that KHR_mesh_quantization reads texture coordinates correctly.
/// Duck/glTF-Quantized uses U16 texture coordinates with normalized=false.
/// The library should return raw integer values, not normalized [0,1] values.
#[test]
#[cfg(feature = "KHR_mesh_quantization")]
fn test_quantized_texture_coordinates() {
    // Reference values extracted from Duck/glTF-Quantized via manual binary parsing
    const TEST_INDICES: [usize; 5] = [0, 100, 500, 1000, 2000];
    const EXPECTED: [[f32; 2]; 5] = [
        [3595.0, 2479.0],
        [3509.0, 2385.0],
        [3871.0, 2382.0],
        [3962.0, 2001.0],
        [3635.0, 2875.0],
    ];

    let path = "glTF-Sample-Assets/Models/Duck/glTF-Quantized/Duck.gltf";
    let (document, buffers, _) = gltf::import(&path).expect("Failed to import Duck quantized");

    let mesh = document.meshes().next().expect("No meshes found");
    let primitive = mesh.primitives().next().expect("No primitives found");
    let reader =
        primitive.reader(|buffer: gltf::Buffer| buffers.get(buffer.index()).map(|d| &d.0[..]));

    let tex_coords: Vec<[f32; 2]> = reader
        .read_tex_coords(0)
        .expect("Failed to read tex coords")
        .into_f32()
        .collect();

    for (i, &idx) in TEST_INDICES.iter().enumerate() {
        let actual = tex_coords[idx];
        let expected = EXPECTED[i];
        assert!(
            (actual[0] - expected[0]).abs() < 0.01 && (actual[1] - expected[1]).abs() < 0.01,
            "Vertex {} mismatch: got {:?}, expected {:?}",
            idx,
            actual,
            expected
        );
    }
}

/// Test that applying KHR_texture_transform to raw quantized texcoords produces correct final UVs.
#[test]
#[cfg(all(feature = "KHR_mesh_quantization", feature = "KHR_texture_transform"))]
fn test_quantized_texture_coordinates_with_transform() {
    // Expected final UVs after applying texture transform: offset + raw * scale
    const TEST_INDICES: [usize; 5] = [0, 100, 500, 1000, 2000];
    const EXPECTED: [[f32; 2]; 5] = [
        [0.866_503_86, 0.601_165_3],
        [0.846_407_0, 0.579_127_0],
        [0.931_000_7, 0.578_423_6],
        [0.952_266_0, 0.489_098_05],
        [0.875_851_2, 0.694_007_6],
    ];

    let path = "glTF-Sample-Assets/Models/Duck/glTF-Quantized/Duck.gltf";
    let (document, buffers, _) = gltf::import(&path).expect("Failed to import Duck quantized");

    let mesh = document.meshes().next().expect("No meshes found");
    let primitive = mesh.primitives().next().expect("No primitives found");

    let material = primitive.material();
    let pbr = material.pbr_metallic_roughness();
    let base_color_texture = pbr
        .base_color_texture()
        .expect("Expected base color texture");
    let transform = base_color_texture
        .texture_transform()
        .expect("Expected texture transform");
    let offset = transform.offset();
    let scale = transform.scale();

    let reader =
        primitive.reader(|buffer: gltf::Buffer| buffers.get(buffer.index()).map(|d| &d.0[..]));
    let tex_coords: Vec<[f32; 2]> = reader
        .read_tex_coords(0)
        .expect("Failed to read tex coords")
        .into_f32()
        .collect();

    for (i, &idx) in TEST_INDICES.iter().enumerate() {
        let raw = tex_coords[idx];
        let final_uv = [offset[0] + raw[0] * scale[0], offset[1] + raw[1] * scale[1]];
        let expected = EXPECTED[i];
        assert!(
            (final_uv[0] - expected[0]).abs() < 1e-5 && (final_uv[1] - expected[1]).abs() < 1e-5,
            "Vertex {} mismatch: got {:?}, expected {:?}",
            idx,
            final_uv,
            expected
        );
    }
}

/// Test that quantized normals are correctly decoded.
/// Duck/glTF-Quantized uses I8 normalized normals.
#[test]
#[cfg(feature = "KHR_mesh_quantization")]
fn test_quantized_normals() {
    // Reference values extracted via gltf-transform dequantize
    const TEST_INDICES: [usize; 5] = [0, 100, 500, 1000, 2000];
    const EXPECTED: [[f32; 3]; 5] = [
        [-0.188_976_38, -0.937_007_87, 0.299_212_6],
        [-0.590_551_2, -0.291_338_58, -0.755_905_5],
        [0.314_960_63, -0.236_220_47, -0.921_259_8],
        [0.212_598_43, 0.527_559_06, 0.818_897_64],
        [1.0, 0.015_748_032, 0.023_622_047],
    ];

    let path = "glTF-Sample-Assets/Models/Duck/glTF-Quantized/Duck.gltf";
    let (document, buffers, _) = gltf::import(&path).expect("Failed to import Duck quantized");

    let mesh = document.meshes().next().expect("No meshes found");
    let primitive = mesh.primitives().next().expect("No primitives found");
    let reader =
        primitive.reader(|buffer: gltf::Buffer| buffers.get(buffer.index()).map(|d| &d.0[..]));

    let normals: Vec<[f32; 3]> = reader
        .read_normals()
        .expect("Failed to read normals")
        .collect();

    for (i, &idx) in TEST_INDICES.iter().enumerate() {
        let actual = normals[idx];
        let expected = EXPECTED[i];
        for c in 0..3 {
            assert!(
                (actual[c] - expected[c]).abs() < 1e-5,
                "Vertex {} normal[{}] mismatch: got {}, expected {}",
                idx,
                c,
                actual[c],
                expected[c]
            );
        }
    }
}

/// Test that EXT_meshopt_compression extension files can be parsed.
#[test]
#[cfg(all(feature = "EXT_meshopt_compression", feature = "KHR_mesh_quantization"))]
fn test_ext_meshopt_compression_parsing() {
    let path = "glTF-Sample-Assets/Models/DragonAttenuation/glTF-Meshopt/DragonAttenuation.gltf";
    gltf::Gltf::open(&path).expect("Failed to parse DragonAttenuation meshopt file");
}
