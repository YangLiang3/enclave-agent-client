// This file is generated by rust-protobuf 2.14.0. Do not edit
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]
//! Generated file from `image.proto`

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_14_0;

#[derive(PartialEq,Clone,Default)]
pub struct PullImageRequest {
    // message fields
    pub image: ::std::string::String,
    pub container_id: ::std::string::String,
    pub source_creds: ::std::string::String,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a PullImageRequest {
    fn default() -> &'a PullImageRequest {
        <PullImageRequest as ::protobuf::Message>::default_instance()
    }
}

impl PullImageRequest {
    pub fn new() -> PullImageRequest {
        ::std::default::Default::default()
    }

    // string image = 1;


    pub fn get_image(&self) -> &str {
        &self.image
    }
    pub fn clear_image(&mut self) {
        self.image.clear();
    }

    // Param is passed by value, moved
    pub fn set_image(&mut self, v: ::std::string::String) {
        self.image = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_image(&mut self) -> &mut ::std::string::String {
        &mut self.image
    }

    // Take field
    pub fn take_image(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.image, ::std::string::String::new())
    }

    // string container_id = 2;


    pub fn get_container_id(&self) -> &str {
        &self.container_id
    }
    pub fn clear_container_id(&mut self) {
        self.container_id.clear();
    }

    // Param is passed by value, moved
    pub fn set_container_id(&mut self, v: ::std::string::String) {
        self.container_id = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_container_id(&mut self) -> &mut ::std::string::String {
        &mut self.container_id
    }

    // Take field
    pub fn take_container_id(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.container_id, ::std::string::String::new())
    }

    // string source_creds = 3;


    pub fn get_source_creds(&self) -> &str {
        &self.source_creds
    }
    pub fn clear_source_creds(&mut self) {
        self.source_creds.clear();
    }

    // Param is passed by value, moved
    pub fn set_source_creds(&mut self, v: ::std::string::String) {
        self.source_creds = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_source_creds(&mut self) -> &mut ::std::string::String {
        &mut self.source_creds
    }

    // Take field
    pub fn take_source_creds(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.source_creds, ::std::string::String::new())
    }
}

impl ::protobuf::Message for PullImageRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.image)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.container_id)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.source_creds)?;
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
        if !self.image.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.image);
        }
        if !self.container_id.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.container_id);
        }
        if !self.source_creds.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.source_creds);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.image.is_empty() {
            os.write_string(1, &self.image)?;
        }
        if !self.container_id.is_empty() {
            os.write_string(2, &self.container_id)?;
        }
        if !self.source_creds.is_empty() {
            os.write_string(3, &self.source_creds)?;
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
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> PullImageRequest {
        PullImageRequest::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy::INIT;
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "image",
                    |m: &PullImageRequest| { &m.image },
                    |m: &mut PullImageRequest| { &mut m.image },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "container_id",
                    |m: &PullImageRequest| { &m.container_id },
                    |m: &mut PullImageRequest| { &mut m.container_id },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "source_creds",
                    |m: &PullImageRequest| { &m.source_creds },
                    |m: &mut PullImageRequest| { &mut m.source_creds },
                ));
                ::protobuf::reflect::MessageDescriptor::new_pb_name::<PullImageRequest>(
                    "PullImageRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static PullImageRequest {
        static mut instance: ::protobuf::lazy::Lazy<PullImageRequest> = ::protobuf::lazy::Lazy::INIT;
        unsafe {
            instance.get(PullImageRequest::new)
        }
    }
}

impl ::protobuf::Clear for PullImageRequest {
    fn clear(&mut self) {
        self.image.clear();
        self.container_id.clear();
        self.source_creds.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PullImageRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PullImageRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct PullImageResponse {
    // message fields
    pub image_ref: ::std::string::String,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a PullImageResponse {
    fn default() -> &'a PullImageResponse {
        <PullImageResponse as ::protobuf::Message>::default_instance()
    }
}

impl PullImageResponse {
    pub fn new() -> PullImageResponse {
        ::std::default::Default::default()
    }

    // string image_ref = 1;


    pub fn get_image_ref(&self) -> &str {
        &self.image_ref
    }
    pub fn clear_image_ref(&mut self) {
        self.image_ref.clear();
    }

    // Param is passed by value, moved
    pub fn set_image_ref(&mut self, v: ::std::string::String) {
        self.image_ref = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_image_ref(&mut self) -> &mut ::std::string::String {
        &mut self.image_ref
    }

    // Take field
    pub fn take_image_ref(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.image_ref, ::std::string::String::new())
    }
}

impl ::protobuf::Message for PullImageResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.image_ref)?;
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
        if !self.image_ref.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.image_ref);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.image_ref.is_empty() {
            os.write_string(1, &self.image_ref)?;
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
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> PullImageResponse {
        PullImageResponse::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy::INIT;
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "image_ref",
                    |m: &PullImageResponse| { &m.image_ref },
                    |m: &mut PullImageResponse| { &mut m.image_ref },
                ));
                ::protobuf::reflect::MessageDescriptor::new_pb_name::<PullImageResponse>(
                    "PullImageResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static PullImageResponse {
        static mut instance: ::protobuf::lazy::Lazy<PullImageResponse> = ::protobuf::lazy::Lazy::INIT;
        unsafe {
            instance.get(PullImageResponse::new)
        }
    }
}

impl ::protobuf::Clear for PullImageResponse {
    fn clear(&mut self) {
        self.image_ref.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PullImageResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PullImageResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0bimage.proto\x12\x04grpc\"U\n\x10PullImageRequest\x12\x0f\n\x05imag\
    e\x18\x01\x20\x01(\tB\0\x12\x16\n\x0ccontainer_id\x18\x02\x20\x01(\tB\0\
    \x12\x16\n\x0csource_creds\x18\x03\x20\x01(\tB\0:\0\"*\n\x11PullImageRes\
    ponse\x12\x13\n\timage_ref\x18\x01\x20\x01(\tB\0:\0B\0b\x06proto3\
";

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy::INIT;

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}