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
//! Generated file from `github.com/kata-containers/agent/pkg/types/types.proto`

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_14_0;

#[derive(PartialEq,Clone,Default)]
pub struct IPAddress {
    // message fields
    pub family: IPFamily,
    pub address: ::std::string::String,
    pub mask: ::std::string::String,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a IPAddress {
    fn default() -> &'a IPAddress {
        <IPAddress as ::protobuf::Message>::default_instance()
    }
}

impl IPAddress {
    pub fn new() -> IPAddress {
        ::std::default::Default::default()
    }

    // .types.IPFamily family = 1;


    pub fn get_family(&self) -> IPFamily {
        self.family
    }
    pub fn clear_family(&mut self) {
        self.family = IPFamily::v4;
    }

    // Param is passed by value, moved
    pub fn set_family(&mut self, v: IPFamily) {
        self.family = v;
    }

    // string address = 2;


    pub fn get_address(&self) -> &str {
        &self.address
    }
    pub fn clear_address(&mut self) {
        self.address.clear();
    }

    // Param is passed by value, moved
    pub fn set_address(&mut self, v: ::std::string::String) {
        self.address = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_address(&mut self) -> &mut ::std::string::String {
        &mut self.address
    }

    // Take field
    pub fn take_address(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.address, ::std::string::String::new())
    }

    // string mask = 3;


    pub fn get_mask(&self) -> &str {
        &self.mask
    }
    pub fn clear_mask(&mut self) {
        self.mask.clear();
    }

    // Param is passed by value, moved
    pub fn set_mask(&mut self, v: ::std::string::String) {
        self.mask = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_mask(&mut self) -> &mut ::std::string::String {
        &mut self.mask
    }

    // Take field
    pub fn take_mask(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.mask, ::std::string::String::new())
    }
}

impl ::protobuf::Message for IPAddress {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_proto3_enum_with_unknown_fields_into(wire_type, is, &mut self.family, 1, &mut self.unknown_fields)?
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.address)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.mask)?;
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
        if self.family != IPFamily::v4 {
            my_size += ::protobuf::rt::enum_size(1, self.family);
        }
        if !self.address.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.address);
        }
        if !self.mask.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.mask);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if self.family != IPFamily::v4 {
            os.write_enum(1, self.family.value())?;
        }
        if !self.address.is_empty() {
            os.write_string(2, &self.address)?;
        }
        if !self.mask.is_empty() {
            os.write_string(3, &self.mask)?;
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

    fn new() -> IPAddress {
        IPAddress::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy::INIT;
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<IPFamily>>(
                    "family",
                    |m: &IPAddress| { &m.family },
                    |m: &mut IPAddress| { &mut m.family },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "address",
                    |m: &IPAddress| { &m.address },
                    |m: &mut IPAddress| { &mut m.address },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "mask",
                    |m: &IPAddress| { &m.mask },
                    |m: &mut IPAddress| { &mut m.mask },
                ));
                ::protobuf::reflect::MessageDescriptor::new_pb_name::<IPAddress>(
                    "IPAddress",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static IPAddress {
        static mut instance: ::protobuf::lazy::Lazy<IPAddress> = ::protobuf::lazy::Lazy::INIT;
        unsafe {
            instance.get(IPAddress::new)
        }
    }
}

impl ::protobuf::Clear for IPAddress {
    fn clear(&mut self) {
        self.family = IPFamily::v4;
        self.address.clear();
        self.mask.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for IPAddress {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for IPAddress {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Interface {
    // message fields
    pub device: ::std::string::String,
    pub name: ::std::string::String,
    pub IPAddresses: ::protobuf::RepeatedField<IPAddress>,
    pub mtu: u64,
    pub hwAddr: ::std::string::String,
    pub pciAddr: ::std::string::String,
    pub field_type: ::std::string::String,
    pub raw_flags: u32,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Interface {
    fn default() -> &'a Interface {
        <Interface as ::protobuf::Message>::default_instance()
    }
}

impl Interface {
    pub fn new() -> Interface {
        ::std::default::Default::default()
    }

    // string device = 1;


    pub fn get_device(&self) -> &str {
        &self.device
    }
    pub fn clear_device(&mut self) {
        self.device.clear();
    }

    // Param is passed by value, moved
    pub fn set_device(&mut self, v: ::std::string::String) {
        self.device = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_device(&mut self) -> &mut ::std::string::String {
        &mut self.device
    }

    // Take field
    pub fn take_device(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.device, ::std::string::String::new())
    }

    // string name = 2;


    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.name, ::std::string::String::new())
    }

    // repeated .types.IPAddress IPAddresses = 3;


    pub fn get_IPAddresses(&self) -> &[IPAddress] {
        &self.IPAddresses
    }
    pub fn clear_IPAddresses(&mut self) {
        self.IPAddresses.clear();
    }

    // Param is passed by value, moved
    pub fn set_IPAddresses(&mut self, v: ::protobuf::RepeatedField<IPAddress>) {
        self.IPAddresses = v;
    }

    // Mutable pointer to the field.
    pub fn mut_IPAddresses(&mut self) -> &mut ::protobuf::RepeatedField<IPAddress> {
        &mut self.IPAddresses
    }

    // Take field
    pub fn take_IPAddresses(&mut self) -> ::protobuf::RepeatedField<IPAddress> {
        ::std::mem::replace(&mut self.IPAddresses, ::protobuf::RepeatedField::new())
    }

    // uint64 mtu = 4;


    pub fn get_mtu(&self) -> u64 {
        self.mtu
    }
    pub fn clear_mtu(&mut self) {
        self.mtu = 0;
    }

    // Param is passed by value, moved
    pub fn set_mtu(&mut self, v: u64) {
        self.mtu = v;
    }

    // string hwAddr = 5;


    pub fn get_hwAddr(&self) -> &str {
        &self.hwAddr
    }
    pub fn clear_hwAddr(&mut self) {
        self.hwAddr.clear();
    }

    // Param is passed by value, moved
    pub fn set_hwAddr(&mut self, v: ::std::string::String) {
        self.hwAddr = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_hwAddr(&mut self) -> &mut ::std::string::String {
        &mut self.hwAddr
    }

    // Take field
    pub fn take_hwAddr(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.hwAddr, ::std::string::String::new())
    }

    // string pciAddr = 6;


    pub fn get_pciAddr(&self) -> &str {
        &self.pciAddr
    }
    pub fn clear_pciAddr(&mut self) {
        self.pciAddr.clear();
    }

    // Param is passed by value, moved
    pub fn set_pciAddr(&mut self, v: ::std::string::String) {
        self.pciAddr = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_pciAddr(&mut self) -> &mut ::std::string::String {
        &mut self.pciAddr
    }

    // Take field
    pub fn take_pciAddr(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.pciAddr, ::std::string::String::new())
    }

    // string type = 7;


    pub fn get_field_type(&self) -> &str {
        &self.field_type
    }
    pub fn clear_field_type(&mut self) {
        self.field_type.clear();
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: ::std::string::String) {
        self.field_type = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_field_type(&mut self) -> &mut ::std::string::String {
        &mut self.field_type
    }

    // Take field
    pub fn take_field_type(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.field_type, ::std::string::String::new())
    }

    // uint32 raw_flags = 8;


    pub fn get_raw_flags(&self) -> u32 {
        self.raw_flags
    }
    pub fn clear_raw_flags(&mut self) {
        self.raw_flags = 0;
    }

    // Param is passed by value, moved
    pub fn set_raw_flags(&mut self, v: u32) {
        self.raw_flags = v;
    }
}

impl ::protobuf::Message for Interface {
    fn is_initialized(&self) -> bool {
        for v in &self.IPAddresses {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.device)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.name)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.IPAddresses)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.mtu = tmp;
                },
                5 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.hwAddr)?;
                },
                6 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.pciAddr)?;
                },
                7 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.field_type)?;
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.raw_flags = tmp;
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
        if !self.device.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.device);
        }
        if !self.name.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.name);
        }
        for value in &self.IPAddresses {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if self.mtu != 0 {
            my_size += ::protobuf::rt::value_size(4, self.mtu, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.hwAddr.is_empty() {
            my_size += ::protobuf::rt::string_size(5, &self.hwAddr);
        }
        if !self.pciAddr.is_empty() {
            my_size += ::protobuf::rt::string_size(6, &self.pciAddr);
        }
        if !self.field_type.is_empty() {
            my_size += ::protobuf::rt::string_size(7, &self.field_type);
        }
        if self.raw_flags != 0 {
            my_size += ::protobuf::rt::value_size(8, self.raw_flags, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.device.is_empty() {
            os.write_string(1, &self.device)?;
        }
        if !self.name.is_empty() {
            os.write_string(2, &self.name)?;
        }
        for v in &self.IPAddresses {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if self.mtu != 0 {
            os.write_uint64(4, self.mtu)?;
        }
        if !self.hwAddr.is_empty() {
            os.write_string(5, &self.hwAddr)?;
        }
        if !self.pciAddr.is_empty() {
            os.write_string(6, &self.pciAddr)?;
        }
        if !self.field_type.is_empty() {
            os.write_string(7, &self.field_type)?;
        }
        if self.raw_flags != 0 {
            os.write_uint32(8, self.raw_flags)?;
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

    fn new() -> Interface {
        Interface::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy::INIT;
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "device",
                    |m: &Interface| { &m.device },
                    |m: &mut Interface| { &mut m.device },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    |m: &Interface| { &m.name },
                    |m: &mut Interface| { &mut m.name },
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<IPAddress>>(
                    "IPAddresses",
                    |m: &Interface| { &m.IPAddresses },
                    |m: &mut Interface| { &mut m.IPAddresses },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "mtu",
                    |m: &Interface| { &m.mtu },
                    |m: &mut Interface| { &mut m.mtu },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "hwAddr",
                    |m: &Interface| { &m.hwAddr },
                    |m: &mut Interface| { &mut m.hwAddr },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "pciAddr",
                    |m: &Interface| { &m.pciAddr },
                    |m: &mut Interface| { &mut m.pciAddr },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "type",
                    |m: &Interface| { &m.field_type },
                    |m: &mut Interface| { &mut m.field_type },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "raw_flags",
                    |m: &Interface| { &m.raw_flags },
                    |m: &mut Interface| { &mut m.raw_flags },
                ));
                ::protobuf::reflect::MessageDescriptor::new_pb_name::<Interface>(
                    "Interface",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static Interface {
        static mut instance: ::protobuf::lazy::Lazy<Interface> = ::protobuf::lazy::Lazy::INIT;
        unsafe {
            instance.get(Interface::new)
        }
    }
}

impl ::protobuf::Clear for Interface {
    fn clear(&mut self) {
        self.device.clear();
        self.name.clear();
        self.IPAddresses.clear();
        self.mtu = 0;
        self.hwAddr.clear();
        self.pciAddr.clear();
        self.field_type.clear();
        self.raw_flags = 0;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Interface {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Interface {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Route {
    // message fields
    pub dest: ::std::string::String,
    pub gateway: ::std::string::String,
    pub device: ::std::string::String,
    pub source: ::std::string::String,
    pub scope: u32,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Route {
    fn default() -> &'a Route {
        <Route as ::protobuf::Message>::default_instance()
    }
}

impl Route {
    pub fn new() -> Route {
        ::std::default::Default::default()
    }

    // string dest = 1;


    pub fn get_dest(&self) -> &str {
        &self.dest
    }
    pub fn clear_dest(&mut self) {
        self.dest.clear();
    }

    // Param is passed by value, moved
    pub fn set_dest(&mut self, v: ::std::string::String) {
        self.dest = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_dest(&mut self) -> &mut ::std::string::String {
        &mut self.dest
    }

    // Take field
    pub fn take_dest(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.dest, ::std::string::String::new())
    }

    // string gateway = 2;


    pub fn get_gateway(&self) -> &str {
        &self.gateway
    }
    pub fn clear_gateway(&mut self) {
        self.gateway.clear();
    }

    // Param is passed by value, moved
    pub fn set_gateway(&mut self, v: ::std::string::String) {
        self.gateway = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_gateway(&mut self) -> &mut ::std::string::String {
        &mut self.gateway
    }

    // Take field
    pub fn take_gateway(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.gateway, ::std::string::String::new())
    }

    // string device = 3;


    pub fn get_device(&self) -> &str {
        &self.device
    }
    pub fn clear_device(&mut self) {
        self.device.clear();
    }

    // Param is passed by value, moved
    pub fn set_device(&mut self, v: ::std::string::String) {
        self.device = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_device(&mut self) -> &mut ::std::string::String {
        &mut self.device
    }

    // Take field
    pub fn take_device(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.device, ::std::string::String::new())
    }

    // string source = 4;


    pub fn get_source(&self) -> &str {
        &self.source
    }
    pub fn clear_source(&mut self) {
        self.source.clear();
    }

    // Param is passed by value, moved
    pub fn set_source(&mut self, v: ::std::string::String) {
        self.source = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_source(&mut self) -> &mut ::std::string::String {
        &mut self.source
    }

    // Take field
    pub fn take_source(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.source, ::std::string::String::new())
    }

    // uint32 scope = 5;


    pub fn get_scope(&self) -> u32 {
        self.scope
    }
    pub fn clear_scope(&mut self) {
        self.scope = 0;
    }

    // Param is passed by value, moved
    pub fn set_scope(&mut self, v: u32) {
        self.scope = v;
    }
}

impl ::protobuf::Message for Route {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.dest)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.gateway)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.device)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.source)?;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.scope = tmp;
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
        if !self.dest.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.dest);
        }
        if !self.gateway.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.gateway);
        }
        if !self.device.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.device);
        }
        if !self.source.is_empty() {
            my_size += ::protobuf::rt::string_size(4, &self.source);
        }
        if self.scope != 0 {
            my_size += ::protobuf::rt::value_size(5, self.scope, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.dest.is_empty() {
            os.write_string(1, &self.dest)?;
        }
        if !self.gateway.is_empty() {
            os.write_string(2, &self.gateway)?;
        }
        if !self.device.is_empty() {
            os.write_string(3, &self.device)?;
        }
        if !self.source.is_empty() {
            os.write_string(4, &self.source)?;
        }
        if self.scope != 0 {
            os.write_uint32(5, self.scope)?;
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

    fn new() -> Route {
        Route::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy::INIT;
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "dest",
                    |m: &Route| { &m.dest },
                    |m: &mut Route| { &mut m.dest },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "gateway",
                    |m: &Route| { &m.gateway },
                    |m: &mut Route| { &mut m.gateway },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "device",
                    |m: &Route| { &m.device },
                    |m: &mut Route| { &mut m.device },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "source",
                    |m: &Route| { &m.source },
                    |m: &mut Route| { &mut m.source },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "scope",
                    |m: &Route| { &m.scope },
                    |m: &mut Route| { &mut m.scope },
                ));
                ::protobuf::reflect::MessageDescriptor::new_pb_name::<Route>(
                    "Route",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static Route {
        static mut instance: ::protobuf::lazy::Lazy<Route> = ::protobuf::lazy::Lazy::INIT;
        unsafe {
            instance.get(Route::new)
        }
    }
}

impl ::protobuf::Clear for Route {
    fn clear(&mut self) {
        self.dest.clear();
        self.gateway.clear();
        self.device.clear();
        self.source.clear();
        self.scope = 0;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Route {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Route {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum IPFamily {
    v4 = 0,
    v6 = 1,
}

impl ::protobuf::ProtobufEnum for IPFamily {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<IPFamily> {
        match value {
            0 => ::std::option::Option::Some(IPFamily::v4),
            1 => ::std::option::Option::Some(IPFamily::v6),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [IPFamily] = &[
            IPFamily::v4,
            IPFamily::v6,
        ];
        values
    }

    fn enum_descriptor_static() -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy::INIT;
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new_pb_name::<IPFamily>("IPFamily", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for IPFamily {
}

impl ::std::default::Default for IPFamily {
    fn default() -> Self {
        IPFamily::v4
    }
}

impl ::protobuf::reflect::ProtobufValue for IPFamily {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n6github.com/kata-containers/agent/pkg/types/types.proto\x12\x05types\"\
    S\n\tIPAddress\x12!\n\x06family\x18\x01\x20\x01(\x0e2\x0f.types.IPFamily\
    B\0\x12\x11\n\x07address\x18\x02\x20\x01(\tB\0\x12\x0e\n\x04mask\x18\x03\
    \x20\x01(\tB\0:\0\"\xb1\x01\n\tInterface\x12\x10\n\x06device\x18\x01\x20\
    \x01(\tB\0\x12\x0e\n\x04name\x18\x02\x20\x01(\tB\0\x12'\n\x0bIPAddresses\
    \x18\x03\x20\x03(\x0b2\x10.types.IPAddressB\0\x12\r\n\x03mtu\x18\x04\x20\
    \x01(\x04B\0\x12\x10\n\x06hwAddr\x18\x05\x20\x01(\tB\0\x12\x11\n\x07pciA\
    ddr\x18\x06\x20\x01(\tB\0\x12\x0e\n\x04type\x18\x07\x20\x01(\tB\0\x12\
    \x13\n\traw_flags\x18\x08\x20\x01(\rB\0:\0\"a\n\x05Route\x12\x0e\n\x04de\
    st\x18\x01\x20\x01(\tB\0\x12\x11\n\x07gateway\x18\x02\x20\x01(\tB\0\x12\
    \x10\n\x06device\x18\x03\x20\x01(\tB\0\x12\x10\n\x06source\x18\x04\x20\
    \x01(\tB\0\x12\x0f\n\x05scope\x18\x05\x20\x01(\rB\0:\0*\x1c\n\x08IPFamil\
    y\x12\x06\n\x02v4\x10\0\x12\x06\n\x02v6\x10\x01\x1a\0B\0b\x06proto3\
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
