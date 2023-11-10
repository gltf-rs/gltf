use byteorder::{LittleEndian, WriteBytesExt};
use gltf::json::validation::Checked::Valid;
use std::convert::TryInto;
use std::io::Write;
use uuid::Uuid;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
type UnitResult = Result<()>;

fn join(a: &str, b: &str) -> String {
    String::from(a) + b
}

/// Append padding bytes to the end of the input vector until
/// the glTF binary alignment requirement is met.
///
/// The vector will not increase in size if it already meets the alignment requirement.
///
/// See [`GLTF_BINARY_ALIGNMENT`] for more information.
pub fn pad_to_align(v: &mut Vec<u8>, padding: u8) {
    while v.len() as u32 % 4 != 0 {
        v.push(padding);
    }
}

/// Packages JSON and binary data into an exportable binary glTF blob.
fn package(mut json: Vec<u8>, mut bin: Vec<u8>) -> Result<Vec<u8>> {
    pad_to_align(&mut json, b' ');
    pad_to_align(&mut bin, b'\0');

    let header_size = 12;
    let chunk_header_size = 8;
    let json_chunk_data_size: u32 = json.len().try_into()?;
    let bin_chunk_data_size: u32 = bin.len().try_into()?;
    let total_size: usize = header_size
        + 2 * chunk_header_size
        + json_chunk_data_size as usize
        + bin_chunk_data_size as usize;
    let length: u32 = total_size.try_into()?;

    let mut data = Vec::with_capacity(total_size);

    // Header
    data.write_all(b"glTF")?; // header magic
    data.write_u32::<LittleEndian>(2)?; // version
    data.write_u32::<LittleEndian>(length)?; // whole GLB length

    // JSON chunk
    data.write_u32::<LittleEndian>(json_chunk_data_size)?; // chunk length
    data.write_all(b"JSON")?; // chunk magic
    data.write_all(&json[..])?; // JSON payload

    // Binary chunk
    data.write_u32::<LittleEndian>(bin_chunk_data_size)?; // chunk length
    data.write_all(b"BIN\0")?; // chunk magic
    data.write_all(&bin[..])?; // binary payload

    Ok(data)
}

const VERTEX_SIZE: u32 = 44;

// 3-2
// | |
// 0-1
const QUAD_POSITIONS: &[[f32; 3]] = &[
    [-0.5, 0.0, -0.5], // 0
    [0.5, 0.0, -0.5],  // 1
    [0.5, 0.0, 0.5],   // 2
    [-0.5, 0.0, 0.5],  // 3
];

// 3-2   3-2   2
// |/| = |/ + /|
// 0-1   0   0-1
const QUAD_INDICES: &[u32] = &[0, 1, 2, 0, 2, 3];

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
enum Output {
    #[default]
    Standard,
    Binary,
}

fn main() -> UnitResult {
    let basename = "quad";
    let arg = std::env::args().nth(1);
    let output = match arg.as_ref().map(String::as_str) {
        Some("glb") => Output::Binary,
        _ => Output::Standard,
    };

    let mut bin = Vec::new();

    // Write vertices
    let vertex_data_offset = 0;
    let mut vertex_data_length = 0;
    for position in QUAD_POSITIONS {
        let mut bytes_written = 0u32;
        for x in position {
            bin.write_f32::<LittleEndian>(*x).unwrap();
            bytes_written += std::mem::size_of::<f32>() as u32;
        }
        while bytes_written < VERTEX_SIZE {
            bin.push(0);
            bytes_written += 1;
        }
        vertex_data_length += bytes_written;
    }

    // Write indices
    let index_data_offset = bin.len() as u32;
    let mut index_data_length = 0;
    for index in QUAD_INDICES {
        bin.write_u32::<LittleEndian>(*index)?;
        index_data_length += std::mem::size_of::<u32>() as u32;
    }

    let mut root = gltf::json::Root {
        asset: gltf::json::Asset {
            version: "2.0".to_owned(),
            generator: Some("kittycad.io".to_owned()),
            ..Default::default()
        },
        ..Default::default()
    };

    root.buffers.push(gltf::json::Buffer {
        byte_length: bin.len() as u32,
        extensions: None,
        extras: Default::default(),
        name: None,
        uri: if output == Output::Standard {
            Some(join(basename, ".bin"))
        } else {
            // binary payload
            None
        },
    });
    root.buffer_views.push(gltf::json::buffer::View {
        buffer: gltf::json::Index::new(0),
        byte_length: vertex_data_length,
        byte_offset: Some(vertex_data_offset),
        byte_stride: Some(VERTEX_SIZE),
        extensions: None,
        extras: Default::default(),
        name: Some("vertex-buffer".to_string()),
        target: Some(Valid(gltf::json::buffer::Target::ArrayBuffer)),
    });
    root.buffer_views.push(gltf::json::buffer::View {
        buffer: gltf::json::Index::new(0),
        byte_length: index_data_length,
        byte_offset: Some(index_data_offset),
        byte_stride: None,
        extensions: None,
        extras: Default::default(),
        name: Some("index-buffer".to_string()),
        target: Some(Valid(gltf::json::buffer::Target::ElementArrayBuffer)),
    });
    root.accessors.push(gltf::json::Accessor {
        buffer_view: Some(gltf::json::Index::new(0)),
        byte_offset: 0,
        count: QUAD_POSITIONS.len() as u32,
        component_type: Valid(gltf::json::accessor::GenericComponentType(
            gltf::json::accessor::ComponentType::F32,
        )),
        extensions: None,
        extras: Default::default(),
        type_: Valid(gltf::json::accessor::Type::Vec3),
        min: Some(serde_json::json!([-0.5, 0.0, -0.5])),
        max: Some(serde_json::json!([0.5, 0.0, 0.5])),
        name: Some("positions".to_string()),
        normalized: false,
        sparse: None,
    });
    root.accessors.push(gltf::json::Accessor {
        buffer_view: Some(gltf::json::Index::new(0)),
        byte_offset: 0,
        count: QUAD_INDICES.len() as u32,
        component_type: Valid(gltf::json::accessor::GenericComponentType(
            gltf::json::accessor::ComponentType::U32,
        )),
        extensions: None,
        extras: Default::default(),
        type_: Valid(gltf::json::accessor::Type::Scalar),
        min: None,
        max: None,
        name: Some("indices".to_string()),
        normalized: false,
        sparse: None,
    });
    root.meshes.push(gltf::json::Mesh {
        extensions: Some(gltf::json::extensions::mesh::Mesh {
            kittycad_uuid: Some(gltf::json::extensions::kittycad_uuid::Uuid {
                uuid: Uuid::new_v4().to_string(),
            }),
            ..Default::default()
        }),
        extras: Default::default(),
        name: None,
        primitives: vec![gltf::json::mesh::Primitive {
            attributes: std::collections::BTreeMap::from([(
                Valid(gltf::json::mesh::Semantic::Positions),
                gltf::json::Index::new(0),
            )]),
            extensions: None,
            extras: Default::default(),
            indices: Some(gltf::json::Index::new(1)),
            material: None,
            mode: Valid(gltf::json::mesh::Mode::Triangles),
            targets: None,
        }],
        weights: None,
    });
    root.nodes.push(gltf::json::Node {
        camera: None,
        children: None,
        extensions: Some(gltf::json::extensions::scene::Node {
            kittycad_uuid: Some(gltf::json::extensions::kittycad_uuid::Uuid {
                uuid: Uuid::new_v4().to_string(),
            }),
            ..Default::default()
        }),
        extras: Default::default(),
        matrix: None,
        mesh: Some(gltf::json::Index::new(0)),
        name: None,
        rotation: None,
        scale: None,
        translation: None,
        skin: None,
        weights: None,
    });
    root.scenes.push(gltf::json::Scene {
        extensions: None,
        extras: Default::default(),
        name: None,
        nodes: vec![gltf::json::Index::new(0)],
    });
    root.scene = Some(gltf::json::Index::new(0));

    match output {
        Output::Standard => {
            let json = root.to_string_pretty()?;
            std::fs::write(join(basename, ".gltf"), &json)?;
            std::fs::write(join(basename, ".bin"), &bin)?;
        }
        Output::Binary => {
            let glb = package(root.to_vec()?, bin)?;
            std::fs::write(join(basename, ".glb"), &glb)?;
        }
    }

    Ok(())
}
