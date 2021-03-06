// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

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

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(PartialEq,Clone,Default)]
pub struct Propose {
    // message fields
    rand: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    pubkey: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    exchanges: ::protobuf::SingularField<::std::string::String>,
    ciphers: ::protobuf::SingularField<::std::string::String>,
    hashes: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Propose {}

impl Propose {
    pub fn new() -> Propose {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Propose {
        static mut instance: ::protobuf::lazy::Lazy<Propose> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Propose,
        };
        unsafe {
            instance.get(Propose::new)
        }
    }

    // optional bytes rand = 1;

    pub fn clear_rand(&mut self) {
        self.rand.clear();
    }

    pub fn has_rand(&self) -> bool {
        self.rand.is_some()
    }

    // Param is passed by value, moved
    pub fn set_rand(&mut self, v: ::std::vec::Vec<u8>) {
        self.rand = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_rand(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.rand.is_none() {
            self.rand.set_default();
        }
        self.rand.as_mut().unwrap()
    }

    // Take field
    pub fn take_rand(&mut self) -> ::std::vec::Vec<u8> {
        self.rand.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_rand(&self) -> &[u8] {
        match self.rand.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_rand_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.rand
    }

    fn mut_rand_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.rand
    }

    // optional bytes pubkey = 2;

    pub fn clear_pubkey(&mut self) {
        self.pubkey.clear();
    }

    pub fn has_pubkey(&self) -> bool {
        self.pubkey.is_some()
    }

    // Param is passed by value, moved
    pub fn set_pubkey(&mut self, v: ::std::vec::Vec<u8>) {
        self.pubkey = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_pubkey(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.pubkey.is_none() {
            self.pubkey.set_default();
        }
        self.pubkey.as_mut().unwrap()
    }

    // Take field
    pub fn take_pubkey(&mut self) -> ::std::vec::Vec<u8> {
        self.pubkey.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_pubkey(&self) -> &[u8] {
        match self.pubkey.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_pubkey_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.pubkey
    }

    fn mut_pubkey_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.pubkey
    }

    // optional string exchanges = 3;

    pub fn clear_exchanges(&mut self) {
        self.exchanges.clear();
    }

    pub fn has_exchanges(&self) -> bool {
        self.exchanges.is_some()
    }

    // Param is passed by value, moved
    pub fn set_exchanges(&mut self, v: ::std::string::String) {
        self.exchanges = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_exchanges(&mut self) -> &mut ::std::string::String {
        if self.exchanges.is_none() {
            self.exchanges.set_default();
        }
        self.exchanges.as_mut().unwrap()
    }

    // Take field
    pub fn take_exchanges(&mut self) -> ::std::string::String {
        self.exchanges.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_exchanges(&self) -> &str {
        match self.exchanges.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_exchanges_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.exchanges
    }

    fn mut_exchanges_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.exchanges
    }

    // optional string ciphers = 4;

    pub fn clear_ciphers(&mut self) {
        self.ciphers.clear();
    }

    pub fn has_ciphers(&self) -> bool {
        self.ciphers.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ciphers(&mut self, v: ::std::string::String) {
        self.ciphers = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ciphers(&mut self) -> &mut ::std::string::String {
        if self.ciphers.is_none() {
            self.ciphers.set_default();
        }
        self.ciphers.as_mut().unwrap()
    }

    // Take field
    pub fn take_ciphers(&mut self) -> ::std::string::String {
        self.ciphers.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_ciphers(&self) -> &str {
        match self.ciphers.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_ciphers_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.ciphers
    }

    fn mut_ciphers_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.ciphers
    }

    // optional string hashes = 5;

    pub fn clear_hashes(&mut self) {
        self.hashes.clear();
    }

    pub fn has_hashes(&self) -> bool {
        self.hashes.is_some()
    }

    // Param is passed by value, moved
    pub fn set_hashes(&mut self, v: ::std::string::String) {
        self.hashes = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_hashes(&mut self) -> &mut ::std::string::String {
        if self.hashes.is_none() {
            self.hashes.set_default();
        }
        self.hashes.as_mut().unwrap()
    }

    // Take field
    pub fn take_hashes(&mut self) -> ::std::string::String {
        self.hashes.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_hashes(&self) -> &str {
        match self.hashes.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_hashes_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.hashes
    }

    fn mut_hashes_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.hashes
    }
}

impl ::protobuf::Message for Propose {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.rand)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.pubkey)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.exchanges)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.ciphers)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.hashes)?;
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
        if let Some(ref v) = self.rand.as_ref() {
            my_size += ::protobuf::rt::bytes_size(1, &v);
        }
        if let Some(ref v) = self.pubkey.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        }
        if let Some(ref v) = self.exchanges.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        }
        if let Some(ref v) = self.ciphers.as_ref() {
            my_size += ::protobuf::rt::string_size(4, &v);
        }
        if let Some(ref v) = self.hashes.as_ref() {
            my_size += ::protobuf::rt::string_size(5, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.rand.as_ref() {
            os.write_bytes(1, &v)?;
        }
        if let Some(ref v) = self.pubkey.as_ref() {
            os.write_bytes(2, &v)?;
        }
        if let Some(ref v) = self.exchanges.as_ref() {
            os.write_string(3, &v)?;
        }
        if let Some(ref v) = self.ciphers.as_ref() {
            os.write_string(4, &v)?;
        }
        if let Some(ref v) = self.hashes.as_ref() {
            os.write_string(5, &v)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Propose {
    fn new() -> Propose {
        Propose::new()
    }

    fn descriptor_static(_: ::std::option::Option<Propose>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "rand",
                    Propose::get_rand_for_reflect,
                    Propose::mut_rand_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "pubkey",
                    Propose::get_pubkey_for_reflect,
                    Propose::mut_pubkey_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "exchanges",
                    Propose::get_exchanges_for_reflect,
                    Propose::mut_exchanges_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "ciphers",
                    Propose::get_ciphers_for_reflect,
                    Propose::mut_ciphers_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "hashes",
                    Propose::get_hashes_for_reflect,
                    Propose::mut_hashes_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Propose>(
                    "Propose",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Propose {
    fn clear(&mut self) {
        self.clear_rand();
        self.clear_pubkey();
        self.clear_exchanges();
        self.clear_ciphers();
        self.clear_hashes();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Propose {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Propose {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Exchange {
    // message fields
    epubkey: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    signature: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Exchange {}

impl Exchange {
    pub fn new() -> Exchange {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Exchange {
        static mut instance: ::protobuf::lazy::Lazy<Exchange> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Exchange,
        };
        unsafe {
            instance.get(Exchange::new)
        }
    }

    // optional bytes epubkey = 1;

    pub fn clear_epubkey(&mut self) {
        self.epubkey.clear();
    }

    pub fn has_epubkey(&self) -> bool {
        self.epubkey.is_some()
    }

    // Param is passed by value, moved
    pub fn set_epubkey(&mut self, v: ::std::vec::Vec<u8>) {
        self.epubkey = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_epubkey(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.epubkey.is_none() {
            self.epubkey.set_default();
        }
        self.epubkey.as_mut().unwrap()
    }

    // Take field
    pub fn take_epubkey(&mut self) -> ::std::vec::Vec<u8> {
        self.epubkey.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_epubkey(&self) -> &[u8] {
        match self.epubkey.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_epubkey_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.epubkey
    }

    fn mut_epubkey_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.epubkey
    }

    // optional bytes signature = 2;

    pub fn clear_signature(&mut self) {
        self.signature.clear();
    }

    pub fn has_signature(&self) -> bool {
        self.signature.is_some()
    }

    // Param is passed by value, moved
    pub fn set_signature(&mut self, v: ::std::vec::Vec<u8>) {
        self.signature = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_signature(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.signature.is_none() {
            self.signature.set_default();
        }
        self.signature.as_mut().unwrap()
    }

    // Take field
    pub fn take_signature(&mut self) -> ::std::vec::Vec<u8> {
        self.signature.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_signature(&self) -> &[u8] {
        match self.signature.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_signature_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.signature
    }

    fn mut_signature_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.signature
    }
}

impl ::protobuf::Message for Exchange {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.epubkey)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.signature)?;
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
        if let Some(ref v) = self.epubkey.as_ref() {
            my_size += ::protobuf::rt::bytes_size(1, &v);
        }
        if let Some(ref v) = self.signature.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.epubkey.as_ref() {
            os.write_bytes(1, &v)?;
        }
        if let Some(ref v) = self.signature.as_ref() {
            os.write_bytes(2, &v)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Exchange {
    fn new() -> Exchange {
        Exchange::new()
    }

    fn descriptor_static(_: ::std::option::Option<Exchange>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "epubkey",
                    Exchange::get_epubkey_for_reflect,
                    Exchange::mut_epubkey_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "signature",
                    Exchange::get_signature_for_reflect,
                    Exchange::mut_signature_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Exchange>(
                    "Exchange",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Exchange {
    fn clear(&mut self) {
        self.clear_epubkey();
        self.clear_signature();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Exchange {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Exchange {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0esrc/data.proto\x12\x08spipe.pb\"\x85\x01\n\x07Propose\x12\x12\n\
    \x04rand\x18\x01\x20\x01(\x0cR\x04rand\x12\x16\n\x06pubkey\x18\x02\x20\
    \x01(\x0cR\x06pubkey\x12\x1c\n\texchanges\x18\x03\x20\x01(\tR\texchanges\
    \x12\x18\n\x07ciphers\x18\x04\x20\x01(\tR\x07ciphers\x12\x16\n\x06hashes\
    \x18\x05\x20\x01(\tR\x06hashes\"B\n\x08Exchange\x12\x18\n\x07epubkey\x18\
    \x01\x20\x01(\x0cR\x07epubkey\x12\x1c\n\tsignature\x18\x02\x20\x01(\x0cR\
    \tsignatureJ\xa5\x04\n\x06\x12\x04\0\0\r\x01\n\x08\n\x01\x02\x12\x03\0\
    \x08\x10\n\n\n\x02\x04\0\x12\x04\x02\0\x08\x01\n\n\n\x03\x04\0\x01\x12\
    \x03\x02\x08\x0f\n\x0b\n\x04\x04\0\x02\0\x12\x03\x03\x02\x1a\n\x0c\n\x05\
    \x04\0\x02\0\x04\x12\x03\x03\x02\n\n\x0c\n\x05\x04\0\x02\0\x05\x12\x03\
    \x03\x0b\x10\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\x03\x11\x15\n\x0c\n\x05\
    \x04\0\x02\0\x03\x12\x03\x03\x18\x19\n\x0b\n\x04\x04\0\x02\x01\x12\x03\
    \x04\x02\x1c\n\x0c\n\x05\x04\0\x02\x01\x04\x12\x03\x04\x02\n\n\x0c\n\x05\
    \x04\0\x02\x01\x05\x12\x03\x04\x0b\x10\n\x0c\n\x05\x04\0\x02\x01\x01\x12\
    \x03\x04\x11\x17\n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03\x04\x1a\x1b\n\x0b\
    \n\x04\x04\0\x02\x02\x12\x03\x05\x02\x20\n\x0c\n\x05\x04\0\x02\x02\x04\
    \x12\x03\x05\x02\n\n\x0c\n\x05\x04\0\x02\x02\x05\x12\x03\x05\x0b\x11\n\
    \x0c\n\x05\x04\0\x02\x02\x01\x12\x03\x05\x12\x1b\n\x0c\n\x05\x04\0\x02\
    \x02\x03\x12\x03\x05\x1e\x1f\n\x0b\n\x04\x04\0\x02\x03\x12\x03\x06\x02\
    \x1e\n\x0c\n\x05\x04\0\x02\x03\x04\x12\x03\x06\x02\n\n\x0c\n\x05\x04\0\
    \x02\x03\x05\x12\x03\x06\x0b\x11\n\x0c\n\x05\x04\0\x02\x03\x01\x12\x03\
    \x06\x12\x19\n\x0c\n\x05\x04\0\x02\x03\x03\x12\x03\x06\x1c\x1d\n\x0b\n\
    \x04\x04\0\x02\x04\x12\x03\x07\x02\x1d\n\x0c\n\x05\x04\0\x02\x04\x04\x12\
    \x03\x07\x02\n\n\x0c\n\x05\x04\0\x02\x04\x05\x12\x03\x07\x0b\x11\n\x0c\n\
    \x05\x04\0\x02\x04\x01\x12\x03\x07\x12\x18\n\x0c\n\x05\x04\0\x02\x04\x03\
    \x12\x03\x07\x1b\x1c\n\n\n\x02\x04\x01\x12\x04\n\0\r\x01\n\n\n\x03\x04\
    \x01\x01\x12\x03\n\x08\x10\n\x0b\n\x04\x04\x01\x02\0\x12\x03\x0b\x02\x1d\
    \n\x0c\n\x05\x04\x01\x02\0\x04\x12\x03\x0b\x02\n\n\x0c\n\x05\x04\x01\x02\
    \0\x05\x12\x03\x0b\x0b\x10\n\x0c\n\x05\x04\x01\x02\0\x01\x12\x03\x0b\x11\
    \x18\n\x0c\n\x05\x04\x01\x02\0\x03\x12\x03\x0b\x1b\x1c\n\x0b\n\x04\x04\
    \x01\x02\x01\x12\x03\x0c\x02\x1f\n\x0c\n\x05\x04\x01\x02\x01\x04\x12\x03\
    \x0c\x02\n\n\x0c\n\x05\x04\x01\x02\x01\x05\x12\x03\x0c\x0b\x10\n\x0c\n\
    \x05\x04\x01\x02\x01\x01\x12\x03\x0c\x11\x1a\n\x0c\n\x05\x04\x01\x02\x01\
    \x03\x12\x03\x0c\x1d\x1e\
";

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

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
