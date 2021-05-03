// This file is generated by rust-protobuf 2.23.0. Do not edit
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_imports)]
#![allow(unused_results)]
//! Generated file from `fileformat.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_23_0;

#[derive(PartialEq,Clone,Default)]
pub struct Blob {
    // message fields
    raw: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    raw_size: ::std::option::Option<i32>,
    zlib_data: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    lzma_data: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    bzip2_data: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Blob {
    fn default() -> &'a Blob {
        <Blob as ::protobuf::Message>::default_instance()
    }
}

impl Blob {
    pub fn new() -> Blob {
        ::std::default::Default::default()
    }

    // optional bytes raw = 1;


    pub fn get_raw(&self) -> &[u8] {
        match self.raw.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }
    pub fn clear_raw(&mut self) {
        self.raw.clear();
    }

    pub fn has_raw(&self) -> bool {
        self.raw.is_some()
    }

    // Param is passed by value, moved
    pub fn set_raw(&mut self, v: ::std::vec::Vec<u8>) {
        self.raw = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_raw(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.raw.is_none() {
            self.raw.set_default();
        }
        self.raw.as_mut().unwrap()
    }

    // Take field
    pub fn take_raw(&mut self) -> ::std::vec::Vec<u8> {
        self.raw.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    // optional int32 raw_size = 2;


    pub fn get_raw_size(&self) -> i32 {
        self.raw_size.unwrap_or(0)
    }
    pub fn clear_raw_size(&mut self) {
        self.raw_size = ::std::option::Option::None;
    }

    pub fn has_raw_size(&self) -> bool {
        self.raw_size.is_some()
    }

    // Param is passed by value, moved
    pub fn set_raw_size(&mut self, v: i32) {
        self.raw_size = ::std::option::Option::Some(v);
    }

    // optional bytes zlib_data = 3;


    pub fn get_zlib_data(&self) -> &[u8] {
        match self.zlib_data.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }
    pub fn clear_zlib_data(&mut self) {
        self.zlib_data.clear();
    }

    pub fn has_zlib_data(&self) -> bool {
        self.zlib_data.is_some()
    }

    // Param is passed by value, moved
    pub fn set_zlib_data(&mut self, v: ::std::vec::Vec<u8>) {
        self.zlib_data = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_zlib_data(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.zlib_data.is_none() {
            self.zlib_data.set_default();
        }
        self.zlib_data.as_mut().unwrap()
    }

    // Take field
    pub fn take_zlib_data(&mut self) -> ::std::vec::Vec<u8> {
        self.zlib_data.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    // optional bytes lzma_data = 4;


    pub fn get_lzma_data(&self) -> &[u8] {
        match self.lzma_data.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }
    pub fn clear_lzma_data(&mut self) {
        self.lzma_data.clear();
    }

    pub fn has_lzma_data(&self) -> bool {
        self.lzma_data.is_some()
    }

    // Param is passed by value, moved
    pub fn set_lzma_data(&mut self, v: ::std::vec::Vec<u8>) {
        self.lzma_data = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_lzma_data(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.lzma_data.is_none() {
            self.lzma_data.set_default();
        }
        self.lzma_data.as_mut().unwrap()
    }

    // Take field
    pub fn take_lzma_data(&mut self) -> ::std::vec::Vec<u8> {
        self.lzma_data.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    // optional bytes bzip2_data = 5;


    pub fn get_bzip2_data(&self) -> &[u8] {
        match self.bzip2_data.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }
    pub fn clear_bzip2_data(&mut self) {
        self.bzip2_data.clear();
    }

    pub fn has_bzip2_data(&self) -> bool {
        self.bzip2_data.is_some()
    }

    // Param is passed by value, moved
    pub fn set_bzip2_data(&mut self, v: ::std::vec::Vec<u8>) {
        self.bzip2_data = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_bzip2_data(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.bzip2_data.is_none() {
            self.bzip2_data.set_default();
        }
        self.bzip2_data.as_mut().unwrap()
    }

    // Take field
    pub fn take_bzip2_data(&mut self) -> ::std::vec::Vec<u8> {
        self.bzip2_data.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }
}

impl ::protobuf::Message for Blob {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.raw)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.raw_size = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.zlib_data)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.lzma_data)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.bzip2_data)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.raw.as_ref() {
            my_size += ::protobuf::rt::bytes_size(1, &v);
        }
        if let Some(v) = self.raw_size {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.zlib_data.as_ref() {
            my_size += ::protobuf::rt::bytes_size(3, &v);
        }
        if let Some(ref v) = self.lzma_data.as_ref() {
            my_size += ::protobuf::rt::bytes_size(4, &v);
        }
        if let Some(ref v) = self.bzip2_data.as_ref() {
            my_size += ::protobuf::rt::bytes_size(5, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.raw.as_ref() {
            os.write_bytes(1, &v)?;
        }
        if let Some(v) = self.raw_size {
            os.write_int32(2, v)?;
        }
        if let Some(ref v) = self.zlib_data.as_ref() {
            os.write_bytes(3, &v)?;
        }
        if let Some(ref v) = self.lzma_data.as_ref() {
            os.write_bytes(4, &v)?;
        }
        if let Some(ref v) = self.bzip2_data.as_ref() {
            os.write_bytes(5, &v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> Blob {
        Blob::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                "raw",
                |m: &Blob| { &m.raw },
                |m: &mut Blob| { &mut m.raw },
            ));
            fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                "raw_size",
                |m: &Blob| { &m.raw_size },
                |m: &mut Blob| { &mut m.raw_size },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                "zlib_data",
                |m: &Blob| { &m.zlib_data },
                |m: &mut Blob| { &mut m.zlib_data },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                "lzma_data",
                |m: &Blob| { &m.lzma_data },
                |m: &mut Blob| { &mut m.lzma_data },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                "bzip2_data",
                |m: &Blob| { &m.bzip2_data },
                |m: &mut Blob| { &mut m.bzip2_data },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<Blob>(
                "Blob",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static Blob {
        static instance: ::protobuf::rt::LazyV2<Blob> = ::protobuf::rt::LazyV2::INIT;
        instance.get(Blob::new)
    }
}

impl ::protobuf::Clear for Blob {
    fn clear(&mut self) {
        self.raw.clear();
        self.raw_size = ::std::option::Option::None;
        self.zlib_data.clear();
        self.lzma_data.clear();
        self.bzip2_data.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Blob {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Blob {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct BlockHeader {
    // message fields
    field_type: ::protobuf::SingularField<::std::string::String>,
    indexdata: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    datasize: ::std::option::Option<i32>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a BlockHeader {
    fn default() -> &'a BlockHeader {
        <BlockHeader as ::protobuf::Message>::default_instance()
    }
}

impl BlockHeader {
    pub fn new() -> BlockHeader {
        ::std::default::Default::default()
    }

    // required string type = 1;


    pub fn get_field_type(&self) -> &str {
        match self.field_type.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
    pub fn clear_field_type(&mut self) {
        self.field_type.clear();
    }

    pub fn has_field_type(&self) -> bool {
        self.field_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: ::std::string::String) {
        self.field_type = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_field_type(&mut self) -> &mut ::std::string::String {
        if self.field_type.is_none() {
            self.field_type.set_default();
        }
        self.field_type.as_mut().unwrap()
    }

    // Take field
    pub fn take_field_type(&mut self) -> ::std::string::String {
        self.field_type.take().unwrap_or_else(|| ::std::string::String::new())
    }

    // optional bytes indexdata = 2;


    pub fn get_indexdata(&self) -> &[u8] {
        match self.indexdata.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }
    pub fn clear_indexdata(&mut self) {
        self.indexdata.clear();
    }

    pub fn has_indexdata(&self) -> bool {
        self.indexdata.is_some()
    }

    // Param is passed by value, moved
    pub fn set_indexdata(&mut self, v: ::std::vec::Vec<u8>) {
        self.indexdata = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_indexdata(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.indexdata.is_none() {
            self.indexdata.set_default();
        }
        self.indexdata.as_mut().unwrap()
    }

    // Take field
    pub fn take_indexdata(&mut self) -> ::std::vec::Vec<u8> {
        self.indexdata.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    // required int32 datasize = 3;


    pub fn get_datasize(&self) -> i32 {
        self.datasize.unwrap_or(0)
    }
    pub fn clear_datasize(&mut self) {
        self.datasize = ::std::option::Option::None;
    }

    pub fn has_datasize(&self) -> bool {
        self.datasize.is_some()
    }

    // Param is passed by value, moved
    pub fn set_datasize(&mut self, v: i32) {
        self.datasize = ::std::option::Option::Some(v);
    }
}

impl ::protobuf::Message for BlockHeader {
    fn is_initialized(&self) -> bool {
        if self.field_type.is_none() {
            return false;
        }
        if self.datasize.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.field_type)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.indexdata)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.datasize = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.field_type.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(ref v) = self.indexdata.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        }
        if let Some(v) = self.datasize {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.field_type.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(ref v) = self.indexdata.as_ref() {
            os.write_bytes(2, &v)?;
        }
        if let Some(v) = self.datasize {
            os.write_int32(3, v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> BlockHeader {
        BlockHeader::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "type",
                |m: &BlockHeader| { &m.field_type },
                |m: &mut BlockHeader| { &mut m.field_type },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                "indexdata",
                |m: &BlockHeader| { &m.indexdata },
                |m: &mut BlockHeader| { &mut m.indexdata },
            ));
            fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                "datasize",
                |m: &BlockHeader| { &m.datasize },
                |m: &mut BlockHeader| { &mut m.datasize },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<BlockHeader>(
                "BlockHeader",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static BlockHeader {
        static instance: ::protobuf::rt::LazyV2<BlockHeader> = ::protobuf::rt::LazyV2::INIT;
        instance.get(BlockHeader::new)
    }
}

impl ::protobuf::Clear for BlockHeader {
    fn clear(&mut self) {
        self.field_type.clear();
        self.indexdata.clear();
        self.datasize = ::std::option::Option::None;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for BlockHeader {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for BlockHeader {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x10fileformat.proto\"\x98\x01\n\x04Blob\x12\x12\n\x03raw\x18\x01\x20\
    \x01(\x0cR\x03rawB\0\x12\x1b\n\x08raw_size\x18\x02\x20\x01(\x05R\x07rawS\
    izeB\0\x12\x1d\n\tzlib_data\x18\x03\x20\x01(\x0cR\x08zlibDataB\0\x12\x1d\
    \n\tlzma_data\x18\x04\x20\x01(\x0cR\x08lzmaDataB\0\x12\x1f\n\nbzip2_data\
    \x18\x05\x20\x01(\x0cR\tbzip2DataB\0:\0\"c\n\x0bBlockHeader\x12\x14\n\
    \x04type\x18\x01\x20\x02(\tR\x04typeB\0\x12\x1e\n\tindexdata\x18\x02\x20\
    \x01(\x0cR\tindexdataB\0\x12\x1c\n\x08datasize\x18\x03\x20\x02(\x05R\x08\
    datasizeB\0:\0B\0b\x06proto2\
";

static file_descriptor_proto_lazy: ::protobuf::rt::LazyV2<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::LazyV2::INIT;

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    file_descriptor_proto_lazy.get(|| {
        parse_descriptor_proto()
    })
}
