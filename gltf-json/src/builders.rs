//! Builders indended to help with gltf export implementation.
//!
//! NOTE: Using consuming builders for simplicity. Change to non-consuming
//! builders if benchmarks prove it necessary.
//!

use byteorder::{ByteOrder, LittleEndian};
use ::accessor::sparse::Sparse;
use ::accessor::{GenericComponentType, Type};
use ::buffer::{Buffer, ByteStride, Target, View};
use ::camera::Camera;
use ::extensions;
use ::material::Material;
use ::mesh::{Mode, MorphTarget, Primitive, Semantic};
use ::scene::UnitQuaternion;
use ::skin::Skin;
use ::validation::Checked;
use ::validation::Checked::Valid;
use ::{Accessor, Extras, Index, Mesh, Node, Scene};
use num_traits::cast::ToPrimitive;
use serde_json;
use std::collections::HashMap;
use std::io;

/// A node in the node hierarchy.  When the node contains `skin`, all
/// `mesh.primitives` must contain `JOINTS_0` and `WEIGHTS_0` attributes.
/// A node can have either a `matrix` or any combination of
/// `translation`/`rotation`/`scale` (TRS) properties. TRS properties are converted
/// to matrices and postmultiplied in the `T * R * S` order to compose the
/// transformation matrix; first the scale is applied to the vertices, then the
/// rotation, and then the translation. If none are provided, the transform is the
/// identity. When a node is targeted for animation (referenced by an
/// animation.channel.target), only TRS properties may be present; `matrix` will not
/// be present.
///
/// Can be removed if Node implements Default.
/// Should we keep it for API consistency?
///
#[derive(Default)]
pub struct NodeBuilder {
    camera: Option<Index<Camera>>,
    children: Option<Vec<Index<Node>>>,
    extensions: Option<extensions::scene::Node>,
    extras: Extras,
    matrix: Option<[f32; 16]>,
    mesh: Option<Index<Mesh>>,

    #[cfg(feature = "names")]
    name: Option<String>,

    rotation: Option<UnitQuaternion>,
    scale: Option<[f32; 3]>,
    translation: Option<[f32; 3]>,
    skin: Option<Index<Skin>>,
    weights: Option<Vec<f32>>,
}

impl NodeBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    /// The index of the camera referenced by this node.
    pub fn camera(mut self, c: Index<Camera>) -> Self {
        self.camera = Some(c);
        self
    }

    /// The indices of this node's children.
    pub fn children(mut self, c: Vec<Index<Node>>) -> Self {
        self.children = Some(c);
        self
    }

    /// Extension specific data.
    pub fn extensions(mut self, e: extensions::scene::Node) -> Self {
        self.extensions = Some(e);
        self
    }

    /// Optional application specific data.
    pub fn extras(mut self, e: Extras) -> Self {
        self.extras = e;
        self
    }

    /// 4x4 column-major transformation matrix.
    ///
    /// TODO: Use session types to divide the control flow for .matrix and
    /// decomposed mode.
    pub fn matrix(mut self, m: [f32; 16]) -> Self {
        self.matrix = Some(m);
        self
    }

    /// The index of the mesh in this node.
    pub fn mesh(mut self, m: Index<Mesh>) -> Self {
        self.mesh = Some(m);
        self
    }

    /// Optional user-defined name for this object.
    #[cfg(feature = "names")]
    pub fn name(mut self, n: String) -> Self {
        self.name = Some(n);
        self
    }

    /// The node's unit quaternion rotation in the order (x, y, z, w), where w
    /// is the scalar.
    pub fn rotation(mut self, r: UnitQuaternion) -> Self {
        self.rotation = Some(r);
        self
    }

    /// The node's non-uniform scale.
    pub fn scale(mut self, s: [f32; 3]) -> Self {
        self.scale = Some(s);
        self
    }

    /// The node's translation.
    pub fn translation(mut self, t: [f32; 3]) -> Self {
        self.translation = Some(t);
        self
    }

    /// The index of the skin referenced by this node.
    pub fn skin(mut self, s: Index<Skin>) -> Self {
        self.skin = Some(s);
        self
    }

    /// The weights of the instantiated Morph Target. Number of elements must
    /// match the number of Morph Targets of used mesh.
    pub fn weights(mut self, w: Vec<f32>) -> Self {
        self.weights = Some(w);
        self
    }

    pub fn build(self) -> Node {
        Node {
            camera: self.camera,
            children: self.children,
            extensions: self.extensions,
            extras: self.extras,
            matrix: self.matrix,
            mesh: self.mesh,

            #[cfg(feature = "names")]
            name: self.name,

            rotation: self.rotation,
            scale: self.scale,
            translation: self.translation,
            skin: self.skin,
            weights: self.weights,
        }
    }
}

/// The root `Node`s of a scene.
///
/// Can be removed if Scene implements Default.
/// Should we keep it for API consistency?
///
#[derive(Default)]
pub struct SceneBuilder {
    extensions: Option<extensions::scene::Scene>,
    extras: Extras,

    #[cfg(feature = "names")]
    name: Option<String>,

    nodes: Vec<Index<Node>>,
}

impl SceneBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    /// Extension specific data.
    pub fn extensions(mut self, e: extensions::scene::Scene) -> Self {
        self.extensions = Some(e);
        self
    }

    /// Optional application specific data.
    pub fn extras(mut self, e: Extras) -> Self {
        self.extras = e;
        self
    }

    /// Optional user-defined name for this object.
    #[cfg(feature = "names")]
    pub fn name(mut self, n: String) -> Self {
        self.name = Some(n);
        self
    }

    /// The indices of each root node.
    pub fn nodes(mut self, n: Vec<Index<Node>>) -> Self {
        self.nodes = n;
        self
    }

    pub fn build(self) -> Scene {
        Scene {
            extensions: self.extensions,
            extras: self.extras,

            #[cfg(feature = "names")]
            name: self.name,

            nodes: self.nodes,
        }
    }
}

/// A set of primitives to be rendered.
///
/// A node can contain one or more meshes and its transform places the meshes in
/// the scene.
///
/// Can be removed if Mesh implements Default.
/// Should we keep it for API consistency?
///
#[derive(Default)]
pub struct MeshBuilder {
    pub extensions: Option<extensions::mesh::Mesh>,
    pub extras: Extras,

    #[cfg(feature = "names")]
    pub name: Option<String>,

    pub primitives: Vec<Primitive>,
    pub weights: Option<Vec<f32>>,
}

impl MeshBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    /// Extension specific data.
    pub fn extensions(mut self, e: extensions::mesh::Mesh) -> Self {
        self.extensions = Some(e);
        self
    }

    /// Optional application specific data.
    pub fn extras(mut self, e: Extras) -> Self {
        self.extras = e;
        self
    }

    /// Optional user-defined name for this object.
    #[cfg(feature = "names")]
    pub fn name(mut self, n: String) -> Self {
        self.name = Some(n);
        self
    }

    /// Defines the geometry to be renderered with a material.
    pub fn primitives(mut self, p: Vec<Primitive>) -> Self {
        self.primitives = p;
        self
    }

    /// Defines the weights to be applied to the morph targets.
    pub fn weights(mut self, w: Vec<f32>) -> Self {
        self.weights = Some(w);
        self
    }

    pub fn build(self) -> Mesh {
        Mesh {
            extensions: self.extensions,
            extras: self.extras,

            #[cfg(feature = "names")]
            name: self.name,

            primitives: self.primitives,
            weights: self.weights,
        }
    }
}

/// Geometry to be rendered with the given material.
///
/// Can be removed if Primitive implements Default.
/// Should we keep it for API consistency?
///
#[derive(Default)]
pub struct PrimitiveBuilder {
    attributes: HashMap<Checked<Semantic>, Index<Accessor>>,
    extensions: Option<extensions::mesh::Primitive>,
    extras: Extras,
    indices: Option<Index<Accessor>>,
    material: Option<Index<Material>>,
    mode: Checked<Mode>,
    targets: Option<Vec<MorphTarget>>,
}

impl PrimitiveBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a vertex attribute and its accessor index to this primitive.
    pub fn attribute(mut self, attr: Semantic, acc: Index<Accessor>) -> Self {
        self.attributes.insert(Valid(attr), acc);
        self
    }

    /// Extension specific data.
    pub fn extensions(mut self, e: extensions::mesh::Primitive) -> Self {
        self.extensions = Some(e);
        self
    }

    /// Optional application specific data.
    pub fn extras(mut self, e: Extras) -> Self {
        self.extras = e;
        self
    }

    /// The index of the accessor that contains the indices.
    pub fn indices(mut self, i: Index<Accessor>) -> Self {
        self.indices = Some(i);
        self
    }

    /// The index of the material to apply to this primitive when rendering
    pub fn material(mut self, m: Index<Material>) -> Self {
        self.material = Some(m);
        self
    }

    /// The type of primitives to render.
    pub fn mode(mut self, m: Mode) -> Self {
        self.mode = Valid(m);
        self
    }

    /// An array of Morph Targets, each  Morph Target is a dictionary mapping
    /// attributes (only `POSITION`, `NORMAL`, and `TANGENT` supported) to their
    /// deviations in the Morph Target.
    pub fn targets(mut self, t: Vec<MorphTarget>) -> Self {
        self.targets = Some(t);
        self
    }

    pub fn build(self) -> Primitive {
        Primitive {
            attributes: self.attributes,
            extensions: self.extensions,
            extras: self.extras,
            indices: self.indices,
            material: self.material,
            mode: self.mode,
            targets: self.targets,
        }
    }
}

/// A typed view into a buffer view.
///
/// This builder has both required and optional members. The required members
/// are handled by struct initialization, while optional members can be
/// specified after calling the method .with_options().
///
/// There are two dependently optional members, max and min.
/// glTF 2.0 specification:
///   POSITION accessor must have min and max properties defined.
///
/// .min and .max aren't validated here. Implement validation later as a pass
/// over the entire gltf structure including buffers and use it for both import
/// and export.
///
pub struct AccessorBuilder {
    /// The parent buffer view this accessor reads from.
    pub buffer_view: Index<View>,

    /// The offset relative to the start of the parent `BufferView` in bytes.
    ///
    /// glTF 2.0 specification:
    ///     The offset of an accessor into a bufferView (i.e.,
    ///     accessor.byteOffset) and the offset of an accessor into a buffer
    ///     (i.e., accessor.byteOffset + bufferView.byteOffset) must be a
    ///     multiple of the size of the accessor's component type.
    ///
    pub byte_offset: usize,

    /// The number of components within the buffer view - not to be confused
    /// with the number of bytes in the buffer view.
    pub count: usize,

    /// The data type of components in the attribute.
    pub component_type: GenericComponentType,

    /// Specifies if the attribute is a scalar, vector, or matrix.
    pub type_: Type,

    /// Minimum value of each component in this attribute.
    /// POSITION accessor must have min and max properties defined.
    pub min: Option<serde_json::Value>,

    /// Maximum value of each component in this attribute.
    /// POSITION accessor must have min and max properties defined.
    pub max: Option<serde_json::Value>,
}

impl AccessorBuilder {
    /// Advance to the optional stage of this builder.
    pub fn with_options(self) -> AccessorBuilderWithOptions {
        AccessorBuilderWithOptions::new(self)
    }

    pub fn build(self) -> Accessor {
        self.with_options().build()
    }
}

pub struct AccessorBuilderWithOptions {
    base: AccessorBuilder,

    // Optionals
    extensions: Option<extensions::accessor::Accessor>,
    extras: Extras,

    #[cfg(feature = "names")]
    name: Option<String>,

    normalized: bool,
    sparse: Option<Sparse>,
}

impl AccessorBuilderWithOptions {
    fn new(base: AccessorBuilder) -> Self {
        Self {
            base,
            extensions: Default::default(),
            extras: Default::default(),

            #[cfg(feature = "names")]
            name: Default::default(),

            normalized: Default::default(),
            sparse: Default::default(),
        }
    }

    /// Extension specific data.
    pub fn extensions(mut self, e: extensions::accessor::Accessor) -> Self {
        self.extensions = Some(e);
        self
    }

    /// Optional application specific data.
    pub fn extras(mut self, e: Extras) -> Self {
        self.extras = e;
        self
    }

    /// Optional user-defined name for this object.
    #[cfg(feature = "names")]
    pub fn name(mut self, n: String) -> Self {
        self.name = Some(n);
        self
    }

    /// Specifies whether integer data values should be normalized.
    pub fn normalized(mut self, n: bool) -> Self {
        self.normalized = n;
        self
    }

    /// Sparse storage of attributes that deviate from their initialization
    /// value.
    pub fn sparse(mut self, s: Sparse) -> Self {
        self.sparse = Some(s);
        self
    }

    pub fn build(self) -> Accessor {
        Accessor {
            buffer_view: self.base.buffer_view,
            byte_offset: self.base.byte_offset.to_u32().unwrap(),
            component_type: Valid(self.base.component_type),
            count: self.base.count.to_u32().unwrap(),
            type_: Valid(self.base.type_),
            max: self.base.max,
            min: self.base.min,
            extensions: self.extensions,
            extras: self.extras,

            #[cfg(feature = "names")]
            name: self.name,

            normalized: self.normalized,
            sparse: self.sparse,
        }
    }
}

/// A view into a buffer generally representing a subset of the buffer.
///
/// <https://github.com/KhronosGroup/glTF/tree/master/specification/2.0#reference-bufferview>
///
/// This builder has both required and optional members. The required members
/// are handled by struct initialization, while optional members can be
/// specified after calling the method .with_options().
///
pub struct ViewBuilder {
    /// The parent `Buffer`.
    pub buffer: Index<Buffer>,

    /// The length of the `BufferView` in bytes.
    pub byte_length: usize,
}

impl ViewBuilder {
    /// Advance to the optional stage of this builder.
    pub fn with_options(self) -> ViewBuilderWithOptions {
        ViewBuilderWithOptions::new(self)
    }

    pub fn build(self) -> View {
        self.with_options().build()
    }
}

pub struct ViewBuilderWithOptions {
    base: ViewBuilder,
    byte_offset: Option<usize>,
    byte_stride: Option<ByteStride>,

    #[cfg(feature = "names")]
    name: Option<String>,

    target: Option<Checked<Target>>,
    extensions: Option<extensions::buffer::View>,
    extras: Extras,
}

impl ViewBuilderWithOptions {
    fn new(base: ViewBuilder) -> Self {
        Self {
            base,
            byte_offset: Default::default(),
            byte_stride: Default::default(),

            #[cfg(feature = "names")]
            name: Default::default(),

            target: Default::default(),
            extensions: Default::default(),
            extras: Default::default(),
        }
    }

    /// Offset into the parent buffer in bytes.
    pub fn byte_offset(mut self, b: usize) -> Self {
        self.byte_offset = Some(b);
        self
    }

    /// The stride in bytes between vertex attributes or other interleavable data.
    ///
    /// When zero, data is assumed to be tightly packed.
    pub fn byte_stride(mut self, s: ByteStride) -> Self {
        self.byte_stride = Some(s);
        self
    }

    /// Optional user-defined name for this object.
    #[cfg(feature = "names")]
    pub fn name(mut self, n: String) -> Self {
        self.name = Some(n);
        self
    }

    /// Optional target the buffer should be bound to.
    pub fn target(mut self, t: Target) -> Self {
        self.target = Some(Valid(t));
        self
    }

    /// Extension specific data.
    pub fn extensions(mut self, e: extensions::buffer::View) -> Self {
        self.extensions = Some(e);
        self
    }

    /// Optional application specific data.
    pub fn extras(mut self, e: Extras) -> Self {
        self.extras = e;
        self
    }

    pub fn build(self) -> View {
        View {
            buffer: self.base.buffer,
            byte_length: self.base.byte_length.to_u32().unwrap(),
            byte_offset: self.byte_offset.map(|o| o.to_u32().unwrap()),
            byte_stride: self.byte_stride,

            #[cfg(feature = "names")]
            name: self.name,

            target: self.target,
            extensions: self.extensions,
            extras: self.extras,
        }
    }
}

/// A buffer points to binary data representing geometry, animations, or skins.
///
/// This builder has both required and optional members. The required members
/// are handled by struct initialization, while optional members can be
/// specified after calling the method .with_options().
///
/// TODO:
///  - Figure out how to help exporter code build correct buffers of all kinds
///    and in combination with all container formats, glb, gltf, gltf-embedded
///    (data URIs). Both `.glb` and `.gltf` files can reference external files
///    and both can use data-URIs. However, using data-URIs in a `.glb`
///    container would be a poor choice since that would only cause extra
///    export/import time and extra file size compared to using the special
///    embedded binary GLB buffer with index 0. Using external binary buffers
///    on the other hand makes perfect sense both with `.glb` and `.gltf`, eg.
///    when sharing textures and meshes between different assets.
///  - Evaluate if the two builders should be merged.
///  - Evaluate if its practical to have a single builder collect information
///    for and output buffer(s), bufferView(s), accessor(s) and mesh(es)?
///
pub struct BufferBuilder {
    /// The length of the buffer in bytes.
    pub byte_length: usize,
}

impl BufferBuilder {
    /// Advance to the optional stage of this builder.
    pub fn with_options(self) -> BufferBuilderWithOptions {
        BufferBuilderWithOptions::new(self)
    }

    pub fn build(self) -> Buffer {
        self.with_options().build()
    }
}

pub struct BufferBuilderWithOptions {
    base: BufferBuilder,

    #[cfg(feature = "names")]
    name: Option<String>,

    uri: Option<String>,
    extensions: Option<extensions::buffer::Buffer>,
    extras: Extras,
}

impl BufferBuilderWithOptions {
    fn new(base: BufferBuilder) -> Self {
        Self {
            base,

            #[cfg(feature = "names")]
            name: Default::default(),

            uri: Default::default(),
            extensions: Default::default(),
            extras: Default::default(),
        }
    }

    /// Optional user-defined name for this object.
    #[cfg(feature = "names")]
    pub fn name(mut self, n: String) -> Self {
        self.name = Some(n);
        self
    }

    /// The uri of the buffer. Relative paths are relative to the .gltf file.
    /// Instead of referencing an external file, the uri can also be a data-uri.
    ///
    /// From the glTF 2.0 specification:
    ///
    ///   glTF asset could use GLB file container to pack all resources into one
    ///   file. glTF Buffer referring to GLB-stored BIN chunk, must have
    ///   buffer.uri property undefined, and it must be the first element of
    ///   buffers array; byte length of BIN chunk could be up to 3 bytes bigger
    ///   than JSON-defined buffer.byteLength to satisfy GLB padding
    ///   requirements.
    ///
    pub fn uri(mut self, u: String) -> Self {
        self.uri = Some(u);
        self
    }

    /// Extension specific data.
    pub fn extensions(mut self, e: extensions::buffer::Buffer) -> Self {
        self.extensions = Some(e);
        self
    }

    /// Optional application specific data.
    pub fn extras(mut self, e: Extras) -> Self {
        self.extras = e;
        self
    }

    pub fn build(self) -> Buffer {
        Buffer {
            byte_length: self.base.byte_length.to_u32().unwrap(), // TODO: Fix this in `gltf`.

            #[cfg(feature = "names")]
            name: self.name,

            uri: self.uri,
            extensions: self.extensions,
            extras: self.extras,
        }
    }
}

/// SubBuffer is not a name from glTF 2.0 standard, it is our name for a
/// homogenous subsequence of a buffer, excluding its possible padding.
///
/// This type allows storing subsequences of different types in the same vector
/// inside BufferDataBuilder.
///
/// TODO: Investigate if there's a point of storing Iterator(s) instead and if
/// it would impact performance. It could reduce memory usage in some
/// situations(?).
///
enum SubBuffer<'a> {
    /// 5121 UNSIGNED_BYTE
    U8(&'a [u8]),

    /// 5153 UNSIGNED_SHORT
    U16(&'a [u16]),

    /// 5125 UNSIGNED_INT
    U32(&'a [u32]),

    /// 5120 BYTE
    I8(&'a [i8]),

    /// 5122 SHORT
    I16(&'a [i16]),

    /// 5126 FLOAT
    F32(&'a [f32]),
}

/// A sub-buffer with pre-calculated padding, to help simplify the
/// implementation of BufferDataReader.
///
struct SubBufferWithPad<'a> {
    /// Byte offset from the end of the previous sub-buffer.
    padding: usize,

    /// Information needed by the caller code when building the gltf json
    /// document during export.
    info: SubBufferInfo,

    /// A handle to the source data.
    ///
    sub_buf: SubBuffer<'a>,
}

/// Helps with alignment, endianness etc. Provides a lazy conversion by
/// returning a `std::io::Read` to allow the caller code to controll buffering
/// and memory usage.
///
/// TODO: Figure out how to handle interleaved formats. Interleaved buffers
///       should have its own dedicated builder and reader type?
///
pub struct BufferDataBuilder<'a> {
    /// Internal counter to help with pre-calculation of padding and alignment
    /// when pushing a new subslice to the buffer builder. The padding and
    /// alignment information will be used by the BufferDataReader.
    ///
    /// This does not include the padding for the next sub-buffer since that
    /// cannot be known before the type of the next sub-buffer is.
    ///
    byte_length: usize,

    /// Sequencial subslices to be used as source data when the BufferDataReader
    /// produces the resulting byte stream.
    sources: Vec<SubBufferWithPad<'a>>,
}

impl<'a> BufferDataBuilder<'a> {
    pub fn new() -> Self {
        Self {
            byte_length: 0,
            sources: Default::default(),
        }
    }

    /// Store a reference to some source data for deferred streaming.
    pub fn push_u8(mut self, data: &'a [u8]) -> Self {
        self.push_internal(SubBuffer::U8(data), data.len(), 1);
        self
    }

    /// Store a reference to some source data for deferred streaming.
    pub fn push_u16(mut self, data: &'a [u16]) -> Self {
        self.push_internal(SubBuffer::U16(data), data.len(), 2);
        self
    }

    /// Store a reference to some source data for deferred streaming.
    pub fn push_u32(mut self, data: &'a [u32]) -> Self {
        self.push_internal(SubBuffer::U32(data), data.len(), 4);
        self
    }

    /// Store a reference to some source data for deferred streaming.
    pub fn push_f32(mut self, data: &'a [f32]) -> Self {
        self.push_internal(SubBuffer::F32(data), data.len(), 4);
        self
    }

    /// Store a reference to some source data for deferred streaming.
    pub fn push_i8(mut self, data: &'a [i8]) -> Self {
        self.push_internal(SubBuffer::I8(data), data.len(), 1);
        self
    }

    /// Store a reference to some source data for deferred streaming.
    pub fn push_i16(mut self, data: &'a [i16]) -> Self {
        self.push_internal(SubBuffer::I16(data), data.len(), 2);
        self
    }

    /// Internal shared code.
    fn push_internal(
        &mut self,
        sub_buf: SubBuffer<'a>,
        component_count: usize,
        component_byte_size: usize,
    ) {
        let padding = self.byte_length % component_byte_size;
        let byte_offset = self.byte_length + padding;
        let byte_length = component_count * component_byte_size;
        let info = SubBufferInfo {
            byte_length,
            byte_offset,
        };
        self.sources.push(SubBufferWithPad {
            padding,
            info,
            sub_buf,
        });
        self.byte_length += padding + byte_length;
    }

    pub fn into_reader(self) -> BufferDataReader<'a> {
        BufferDataReader::new(self)
    }
}

/// Intended to be used with `std::io:copy` and `std::io::BufWriter` to give the
/// caller code some control over memory usage.
///
/// See BufferDataBuilder.
///
pub struct BufferDataReader<'a> {
    /// Total length of the buffer, in bytes.
    byte_length: usize,

    /// Internal index for the sub-buffer currently being streamed by
    /// BufferDataReader::read.
    i: usize,

    /// Internal index for the next sub-buffer component to be streamed by
    /// BufferDataReader::read. This is needed since the buffer passed to the
    /// read method may not be large enough to convert all of the current
    /// sub-buffer.
    j: usize,

    sources: Vec<SubBufferWithPad<'a>>,
}

/// Byte offset and length of a sub-buffer.
///
/// This information is needed by the caller code when building the gltf json
/// document during export.
///
pub struct SubBufferInfo {
    /// The address of the first byte of the first element in this sub-buffer
    /// relative to the first byte of the buffer. In other words this is the
    /// offset from the start of the buffer, including the padding for this
    /// sub-buffer.
    pub byte_offset: usize,

    /// The byte length of this sub-buffer.
    pub byte_length: usize,
}

impl<'a> BufferDataReader<'a> {
    fn new(base: BufferDataBuilder<'a>) -> Self {
        Self {
            byte_length: base.byte_length,
            i: 0,
            j: 0,
            sources: base.sources,
        }
    }

    /// Total length of the buffer, in bytes.
    pub fn byte_length(&self) -> usize {
        self.byte_length
    }

    /// Get the sub-buffer info by its index.
    pub fn info(&self, index: usize) -> Option<&SubBufferInfo> {
        self.sources.get(index).map(|s| &s.info)
    }
}

struct InternalSubStreamStatusData {
    bytes_written: usize,
    components_written: usize,
}

enum InternalSubStreamStatus {
    EndOfStream(InternalSubStreamStatusData),
    MoreData(InternalSubStreamStatusData),
}

/// TODO: This one really needs tests and benchmarks.
///
impl<'a> io::Read for BufferDataReader<'a> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        use self::InternalSubStreamStatus::*;
        let mut bytes_written = 0;
        loop {
            let (padding, sub_buf) = match self.sources.get(self.i) {
                Some(s) => (s.padding, &s.sub_buf),
                None => return Ok(bytes_written), // 0 == done.
            };
            if self.j == 0 && padding > 0 {
                if bytes_written + padding >= buf.len() {
                    break;
                }
                for _ in 0..padding {
                    buf[bytes_written] = 0u8; // Zero the padding bytes.
                    bytes_written += 1;
                }
            }
            let padded_target = &mut buf[bytes_written..];
            let substream_status = match sub_buf {
                SubBuffer::U8(s) => convert_sub_buffer_chunk(
                    &s[self.j..],
                    padded_target,
                    copy_u8_into,
                ),
                SubBuffer::U16(s) => convert_sub_buffer_chunk(
                    &s[self.j..],
                    padded_target,
                    LittleEndian::write_u16_into,
                ),
                SubBuffer::U32(s) => convert_sub_buffer_chunk(
                    &s[self.j..],
                    padded_target,
                    LittleEndian::write_u32_into,
                ),
                SubBuffer::I8(s) => convert_sub_buffer_chunk(
                    &s[self.j..],
                    padded_target,
                    copy_i8_as_u8_into,
                ),
                SubBuffer::I16(s) => convert_sub_buffer_chunk(
                    &s[self.j..],
                    padded_target,
                    LittleEndian::write_i16_into,
                ),
                SubBuffer::F32(s) => convert_sub_buffer_chunk(
                    &s[self.j..],
                    padded_target,
                    LittleEndian::write_f32_into,
                ),
            };
            match substream_status {
                EndOfStream(d) => {
                    bytes_written += d.bytes_written;
                    self.j = 0;
                    self.i += 1;
                }
                MoreData(d) => {
                    bytes_written += d.bytes_written;
                    self.j += d.components_written;
                    if d.components_written == 0 {
                        break;
                    }
                }
            }
        }
        if bytes_written > 0 {
            Ok(bytes_written)
        } else {
            Err(io::Error::new(
                io::ErrorKind::Other,
                "BufferDataReader, target buffer is too small.",
            ))
        }
    }
}

/// For single byte component types, since they don't need to care about
/// endianness.
///
/// Will panic if `src` and `dst` have different lengths.
///
fn copy_u8_into(src: &[u8], dst: &mut [u8]) {
    dst.copy_from_slice(src);
}

/// For single byte component types, since they don't need to care about
/// endianness.
///
/// Will panic if `src` and `dst` have different lengths.
///
/// TODO: Benchmark this and change the loop for `ptr::copy_nonoverlapping` if
///       needed.
///
fn copy_i8_as_u8_into(src: &[i8], dst: &mut [u8]) {
    if src.len() != dst.len() {
        panic!("copy_i8_as_u8_into requires same length for src and dst.");
    }
    for i in 0..src.len() {
        dst[i] = src[i] as u8;
    }
}

fn convert_sub_buffer_chunk<T, F>(
    src: &[T],
    dst: &mut [u8],
    write_as_little_endian: F,
) -> InternalSubStreamStatus
where
    T: Sized,
    F: Fn(&[T], &mut [u8]),
{
    use self::InternalSubStreamStatus::*;
    use std::cmp::min;
    use std::mem::size_of;
    let component_byte_size = size_of::<T>();
    let capacity = dst.len() / component_byte_size;
    let components = min(src.len(), capacity);
    let bytes = components * component_byte_size;
    let mut chunk_dst = &mut dst[..bytes];
    let chunk_src = &src[..components];
    write_as_little_endian(&chunk_src, &mut chunk_dst);
    let status_data = InternalSubStreamStatusData {
        bytes_written: bytes,
        components_written: components,
    };
    if components == src.len() {
        EndOfStream(status_data)
    } else {
        MoreData(status_data)
    }
}
