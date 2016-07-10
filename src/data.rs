// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(Clone,Default)]
pub struct Propose {
    // message fields
    rand: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    pubkey: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    exchanges: ::protobuf::SingularField<::std::string::String>,
    ciphers: ::protobuf::SingularField<::std::string::String>,
    hashes: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
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
            instance.get(|| {
                Propose {
                    rand: ::protobuf::SingularField::none(),
                    pubkey: ::protobuf::SingularField::none(),
                    exchanges: ::protobuf::SingularField::none(),
                    ciphers: ::protobuf::SingularField::none(),
                    hashes: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
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
        };
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
        };
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
        };
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
        };
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
        };
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
}

impl ::protobuf::Message for Propose {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.rand));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.pubkey));
                },
                3 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.exchanges));
                },
                4 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.ciphers));
                },
                5 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.hashes));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.rand.iter() {
            my_size += ::protobuf::rt::bytes_size(1, &value);
        };
        for value in self.pubkey.iter() {
            my_size += ::protobuf::rt::bytes_size(2, &value);
        };
        for value in self.exchanges.iter() {
            my_size += ::protobuf::rt::string_size(3, &value);
        };
        for value in self.ciphers.iter() {
            my_size += ::protobuf::rt::string_size(4, &value);
        };
        for value in self.hashes.iter() {
            my_size += ::protobuf::rt::string_size(5, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.rand.as_ref() {
            try!(os.write_bytes(1, &v));
        };
        if let Some(v) = self.pubkey.as_ref() {
            try!(os.write_bytes(2, &v));
        };
        if let Some(v) = self.exchanges.as_ref() {
            try!(os.write_string(3, &v));
        };
        if let Some(v) = self.ciphers.as_ref() {
            try!(os.write_string(4, &v));
        };
        if let Some(v) = self.hashes.as_ref() {
            try!(os.write_string(5, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<Propose>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
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
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "rand",
                    Propose::has_rand,
                    Propose::get_rand,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "pubkey",
                    Propose::has_pubkey,
                    Propose::get_pubkey,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "exchanges",
                    Propose::has_exchanges,
                    Propose::get_exchanges,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "ciphers",
                    Propose::has_ciphers,
                    Propose::get_ciphers,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "hashes",
                    Propose::has_hashes,
                    Propose::get_hashes,
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

impl ::std::cmp::PartialEq for Propose {
    fn eq(&self, other: &Propose) -> bool {
        self.rand == other.rand &&
        self.pubkey == other.pubkey &&
        self.exchanges == other.exchanges &&
        self.ciphers == other.ciphers &&
        self.hashes == other.hashes &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Propose {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Exchange {
    // message fields
    epubkey: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    signature: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
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
            instance.get(|| {
                Exchange {
                    epubkey: ::protobuf::SingularField::none(),
                    signature: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
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
        };
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
        };
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
}

impl ::protobuf::Message for Exchange {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.epubkey));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.signature));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.epubkey.iter() {
            my_size += ::protobuf::rt::bytes_size(1, &value);
        };
        for value in self.signature.iter() {
            my_size += ::protobuf::rt::bytes_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.epubkey.as_ref() {
            try!(os.write_bytes(1, &v));
        };
        if let Some(v) = self.signature.as_ref() {
            try!(os.write_bytes(2, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<Exchange>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
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
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "epubkey",
                    Exchange::has_epubkey,
                    Exchange::get_epubkey,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "signature",
                    Exchange::has_signature,
                    Exchange::get_signature,
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

impl ::std::cmp::PartialEq for Exchange {
    fn eq(&self, other: &Exchange) -> bool {
        self.epubkey == other.epubkey &&
        self.signature == other.signature &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Exchange {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x0b, 0x73, 0x70, 0x69, 0x70, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x08, 0x73,
    0x70, 0x69, 0x70, 0x65, 0x2e, 0x70, 0x62, 0x22, 0x5b, 0x0a, 0x07, 0x50, 0x72, 0x6f, 0x70, 0x6f,
    0x73, 0x65, 0x12, 0x0c, 0x0a, 0x04, 0x72, 0x61, 0x6e, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0c,
    0x12, 0x0e, 0x0a, 0x06, 0x70, 0x75, 0x62, 0x6b, 0x65, 0x79, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0c,
    0x12, 0x11, 0x0a, 0x09, 0x65, 0x78, 0x63, 0x68, 0x61, 0x6e, 0x67, 0x65, 0x73, 0x18, 0x03, 0x20,
    0x01, 0x28, 0x09, 0x12, 0x0f, 0x0a, 0x07, 0x63, 0x69, 0x70, 0x68, 0x65, 0x72, 0x73, 0x18, 0x04,
    0x20, 0x01, 0x28, 0x09, 0x12, 0x0e, 0x0a, 0x06, 0x68, 0x61, 0x73, 0x68, 0x65, 0x73, 0x18, 0x05,
    0x20, 0x01, 0x28, 0x09, 0x22, 0x2e, 0x0a, 0x08, 0x45, 0x78, 0x63, 0x68, 0x61, 0x6e, 0x67, 0x65,
    0x12, 0x0f, 0x0a, 0x07, 0x65, 0x70, 0x75, 0x62, 0x6b, 0x65, 0x79, 0x18, 0x01, 0x20, 0x01, 0x28,
    0x0c, 0x12, 0x11, 0x0a, 0x09, 0x73, 0x69, 0x67, 0x6e, 0x61, 0x74, 0x75, 0x72, 0x65, 0x18, 0x02,
    0x20, 0x01, 0x28, 0x0c, 0x4a, 0xa5, 0x04, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x0d, 0x01, 0x0a,
    0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x00, 0x08, 0x10, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12,
    0x04, 0x02, 0x00, 0x08, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x02, 0x08,
    0x0f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x03, 0x02, 0x1a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x03, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x03, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x03, 0x11, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00,
    0x03, 0x12, 0x03, 0x03, 0x18, 0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03,
    0x04, 0x02, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x04, 0x02,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x04, 0x0b, 0x10, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x04, 0x11, 0x17, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x04, 0x1a, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x00, 0x02, 0x02, 0x12, 0x03, 0x05, 0x02, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02,
    0x04, 0x12, 0x03, 0x05, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x05, 0x12,
    0x03, 0x05, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x05,
    0x12, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x05, 0x1e, 0x1f,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x03, 0x12, 0x03, 0x06, 0x02, 0x1e, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x03, 0x04, 0x12, 0x03, 0x06, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x03, 0x05, 0x12, 0x03, 0x06, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x03, 0x01, 0x12, 0x03, 0x06, 0x12, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x03,
    0x12, 0x03, 0x06, 0x1c, 0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x04, 0x12, 0x03, 0x07,
    0x02, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x04, 0x12, 0x03, 0x07, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x05, 0x12, 0x03, 0x07, 0x0b, 0x11, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x01, 0x12, 0x03, 0x07, 0x12, 0x18, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x04, 0x03, 0x12, 0x03, 0x07, 0x1b, 0x1c, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01,
    0x12, 0x04, 0x0a, 0x00, 0x0d, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x0a,
    0x08, 0x10, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x0b, 0x02, 0x1d, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x04, 0x12, 0x03, 0x0b, 0x02, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x00, 0x05, 0x12, 0x03, 0x0b, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0b, 0x11, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x00, 0x03, 0x12, 0x03, 0x0b, 0x1b, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x01, 0x12,
    0x03, 0x0c, 0x02, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x04, 0x12, 0x03, 0x0c,
    0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x05, 0x12, 0x03, 0x0c, 0x0b, 0x10,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x0c, 0x11, 0x1a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x03, 0x12, 0x03, 0x0c, 0x1d, 0x1e,
];

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
