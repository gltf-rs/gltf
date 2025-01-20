use gltf::validation::USize64;
use gltf::Stub;
use std::borrow::Cow;
use std::io::Write;
use std::{fs, mem};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Output {
    /// Output standard glTF.
    Standard,

    /// Output binary glTF.
    Binary,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
}

/// Calculate bounding coordinates of a list of vertices, used for the clipping distance of the model
fn bounding_coords(points: &[Vertex]) -> ([f32; 3], [f32; 3]) {
    let mut min = [f32::MAX, f32::MAX, f32::MAX];
    let mut max = [f32::MIN, f32::MIN, f32::MIN];

    for point in points {
        let p = point.position;
        for i in 0..3 {
            min[i] = f32::min(min[i], p[i]);
            max[i] = f32::max(max[i], p[i]);
        }
    }
    (min, max)
}

fn align_to_multiple_of_four(n: &mut usize) {
    *n = (*n + 3) & !3;
}

fn to_padded_byte_vector<T>(vec: Vec<T>) -> Vec<u8> {
    let byte_length = vec.len() * mem::size_of::<T>();
    let byte_capacity = vec.capacity() * mem::size_of::<T>();
    let alloc = vec.into_boxed_slice();
    let ptr = Box::<[T]>::into_raw(alloc) as *mut u8;
    let mut new_vec = unsafe { Vec::from_raw_parts(ptr, byte_length, byte_capacity) };
    while new_vec.len() % 4 != 0 {
        new_vec.push(0); // pad to multiple of four bytes
    }
    new_vec
}

fn export(output: Output) {
    let triangle_vertices = vec![
        Vertex {
            position: [0.0, 0.5, 0.0],
            color: [1.0, 0.0, 0.0],
        },
        Vertex {
            position: [-0.5, -0.5, 0.0],
            color: [0.0, 1.0, 0.0],
        },
        Vertex {
            position: [0.5, -0.5, 0.0],
            color: [0.0, 0.0, 1.0],
        },
    ];

    let (min, max) = bounding_coords(&triangle_vertices);

    let mut root = gltf::Root::default();

    let buffer_length = triangle_vertices.len() * mem::size_of::<Vertex>();
    let buffer = root.push(gltf::Buffer {
        length: USize64::from(buffer_length),
        uri: if output == Output::Standard {
            Some("buffer0.bin".into())
        } else {
            None
        },
        ..Stub::stub()
    });
    let buffer_view = root.push(gltf::buffer::View {
        buffer,
        length: USize64::from(buffer_length),
        offset: USize64(0),
        stride: Some(mem::size_of::<Vertex>()),
        target: Some(gltf::buffer::Target::ArrayBuffer),
        ..Stub::stub()
    });
    let positions = root.push(gltf::Accessor {
        buffer_view: Some(buffer_view),
        byte_offset: Some(USize64(0)),
        count: USize64::from(triangle_vertices.len()),
        component_type: gltf::accessor::ComponentType::F32,
        attribute_type: gltf::accessor::AttributeType::Vec3,
        min: Some(gltf::Value::from(Vec::from(min))),
        max: Some(gltf::Value::from(Vec::from(max))),
        normalized: false,
        ..Stub::stub()
    });
    let colors = root.push(gltf::Accessor {
        buffer_view: Some(buffer_view),
        byte_offset: Some(USize64::from(3 * mem::size_of::<f32>())),
        count: USize64::from(triangle_vertices.len()),
        component_type: gltf::accessor::ComponentType::F32,
        attribute_type: gltf::accessor::AttributeType::Vec3,
        normalized: false,
        ..Stub::stub()
    });

    let primitive = gltf::mesh::Primitive {
        attributes: [
            (gltf::mesh::Semantic::Positions, positions),
            (gltf::mesh::Semantic::Colors(0), colors),
        ]
        .into(),
        indices: None,
        material: None,
        mode: gltf::mesh::Mode::Triangles,
        ..Stub::stub()
    };

    let mesh = root.push(gltf::Mesh {
        primitives: vec![primitive],
        ..Stub::stub()
    });

    let node = root.push(gltf::Node {
        mesh: Some(mesh),
        ..Default::default()
    });

    root.push(gltf::Scene {
        nodes: vec![node],
        ..Stub::stub()
    });

    match output {
        Output::Standard => {
            let _ = fs::create_dir("triangle");

            let writer = fs::File::create("triangle/triangle.gltf").expect("I/O error");
            serde_json::to_writer_pretty(writer, &root).expect("Serialization error");

            let bin = to_padded_byte_vector(triangle_vertices);
            let mut writer = fs::File::create("triangle/buffer0.bin").expect("I/O error");
            writer.write_all(&bin).expect("I/O error");
        }
        Output::Binary => {
            let json_string = serde_json::to_string(&root).expect("Serialization error");
            let mut json_offset = json_string.len();
            align_to_multiple_of_four(&mut json_offset);
            let glb = gltf::binary::Glb {
                header: gltf::binary::Header {
                    magic: *b"glTF",
                    version: 2,
                    // N.B., the size of binary glTF file is limited to range of `u32`.
                    length: (json_offset + buffer_length)
                        .try_into()
                        .expect("file size exceeds binary glTF limit"),
                },
                bin: Some(Cow::Owned(to_padded_byte_vector(triangle_vertices))),
                json: Cow::Owned(json_string.into_bytes()),
            };
            let writer = std::fs::File::create("triangle.glb").expect("I/O error");
            glb.to_writer(writer).expect("glTF binary output error");
        }
    }
}

fn main() {
    export(Output::Standard);
    export(Output::Binary);
}
