//!
//! A minimal example of generating and exporting a simple triangle with an
//! external binary buffer.
//!
//! The exported binary buffer should be identical to the official sample model
//! "Triangle" (glTF-Sample-Models/2.0/Triangle/glTF).
//!
//! The exported `.gltf` file should be semantically identical to the sample
//! model, but with some minor syntactic differences, like json property order
//! and array formatting.
//!

extern crate byteorder;
extern crate gltf_json;
extern crate num_traits;
extern crate serde_json;

use GltfIoError::{Io, Json};
use gltf_json::accessor::ComponentType::{F32, U16};
use gltf_json::accessor::{GenericComponentType, Type};
use gltf_json::buffer::Target::{ArrayBuffer, ElementArrayBuffer};
use gltf_json::builders::AccessorBuilder;
use gltf_json::builders::BufferBuilder;
use gltf_json::builders::BufferDataBuilder;
use gltf_json::builders::MeshBuilder;
use gltf_json::builders::NodeBuilder;
use gltf_json::builders::PrimitiveBuilder;
use gltf_json::builders::SceneBuilder;
use gltf_json::builders::ViewBuilder;
use gltf_json::mesh::Semantic::Positions;
use gltf_json::serialize::to_writer_pretty;
use gltf_json::{Index, Root};
use std::fs::File;
use std::io::BufWriter;
use std::io;
use std::path::PathBuf;
use std::vec::Vec;

fn main() {
    export_simple_triangle().unwrap();
}

/// Errors that can occur during import, export, serialization, deserialization
/// and related IO.
#[derive(Debug)]
enum GltfIoError {
    Json(gltf_json::Error, PathBuf),
    Io(std::io::Error, PathBuf),
}

type GltfIoResult<T> = Result<T, GltfIoError>;

/// A minimal subset of glTF, a single primitive, always indexed triangles.
struct TriPrim {
    /// glTF FLOAT, 4 bytes, 5126.
    pub positions: Vec<f32>,

    /// glTF UNSIGNED_SHORT, 2 bytes, 5123
    pub indices: Vec<u16>,
}

fn build_test_triangle() -> TriPrim {
    let mut positions = Vec::new();
    positions.extend_from_slice(&[0.0f32, 0.0f32, 0.0f32]);
    positions.extend_from_slice(&[1.0f32, 0.0f32, 0.0f32]);
    positions.extend_from_slice(&[0.0f32, 1.0f32, 0.0f32]);
    let indices = vec![0, 1, 2];
    TriPrim { positions, indices }
}

fn export_simple_triangle() -> GltfIoResult<()> {
    let gltf_path = "Triangle.gltf";
    let bin_path = "simpleTriangle.bin";
    let triangle = build_test_triangle();
    let mut buffer_data_reader = BufferDataBuilder::new()
        .push_u16(&triangle.indices)
        .push_f32(&triangle.positions)
        .into_reader();
    {
        let file = File::create(bin_path).map_err(|e| Io(e, bin_path.into()))?;
        let mut writer = BufWriter::new(file);
        io::copy(&mut buffer_data_reader, &mut writer)
            .map_err(|e| Io(e, bin_path.into()))?;
    }
    let buffers = vec![
        BufferBuilder {
            byte_length: buffer_data_reader.byte_length(),
        }.with_options()
            .uri(bin_path.to_owned())
            .build(),
    ];
    let sub_buf_info_0 = buffer_data_reader.info(0).unwrap();
    let sub_buf_info_1 = buffer_data_reader.info(1).unwrap();
    let buffer_views = vec![
        ViewBuilder {
            buffer: Index::new(0),
            byte_length: sub_buf_info_0.byte_length,
        }.with_options()
            .byte_offset(sub_buf_info_0.byte_offset)
            .target(ElementArrayBuffer)
            .build(),
        ViewBuilder {
            buffer: Index::new(0),
            byte_length: sub_buf_info_1.byte_length,
        }.with_options()
            .byte_offset(sub_buf_info_1.byte_offset)
            .target(ArrayBuffer)
            .build(),
    ];
    let mut accessors = Vec::new();
    let indices_accessor_index = {
        accessors.push(
            AccessorBuilder {
                buffer_view: Index::new(0),
                byte_offset: 0,
                component_type: GenericComponentType(U16),
                count: 3,
                type_: Type::Scalar,
                max: Some(vec![2].into()),
                min: Some(vec![0].into()),
            }.build(),
        );
        Index::new((accessors.len() - 1) as u32)
    };
    let position_accessor_index = {
        accessors.push(
            AccessorBuilder {
                buffer_view: Index::new(1),
                byte_offset: 0,
                component_type: GenericComponentType(F32),
                count: 3,
                type_: Type::Vec3,
                max: Some(vec![1.0, 1.0, 0.0].into()),
                min: Some(vec![0.0, 0.0, 0.0].into()),
            }.build(),
        );
        Index::new((accessors.len() - 1) as u32)
    };
    let primitive = PrimitiveBuilder::new()
        .attribute(Positions, position_accessor_index)
        .indices(indices_accessor_index)
        .build();
    let meshes = vec![MeshBuilder::new().primitives(vec![primitive]).build()];
    let mesh_index = Index::new((meshes.len() - 1) as u32);
    let nodes = vec![NodeBuilder::new().mesh(mesh_index).build()];
    let node_index = Index::new((nodes.len() - 1) as u32);
    let scenes = vec![SceneBuilder::new().nodes(vec![node_index]).build()];
    let root = Root {
        accessors,
        buffers,
        buffer_views,
        meshes,
        nodes,
        scenes,
        ..Default::default()
    };
    File::create(gltf_path)
        .map_err(|e| Io(e, gltf_path.into()))
        .map(BufWriter::new)
        .and_then(|w| {
            to_writer_pretty(w, &root).map_err(|e| Json(e, gltf_path.into()))
        })
}
