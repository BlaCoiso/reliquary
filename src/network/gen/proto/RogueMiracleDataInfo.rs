// This file is generated by rust-protobuf 3.4.0. Do not edit
// .proto file is parsed by pure
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
#![allow(unused_results)]
#![allow(unused_mut)]

//! Generated file from `RogueMiracleDataInfo.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:RogueMiracleDataInfo)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct RogueMiracleDataInfo {
    // message fields
    // @@protoc_insertion_point(field:RogueMiracleDataInfo.DCLABPANBJG)
    pub DCLABPANBJG: u32,
    // @@protoc_insertion_point(field:RogueMiracleDataInfo.KONAGCDHKKH)
    pub KONAGCDHKKH: ::std::collections::HashMap<u32, u32>,
    // @@protoc_insertion_point(field:RogueMiracleDataInfo.MMFONFFPNJM)
    pub MMFONFFPNJM: u32,
    // @@protoc_insertion_point(field:RogueMiracleDataInfo.NKBEHFHLPEF)
    pub NKBEHFHLPEF: u32,
    // @@protoc_insertion_point(field:RogueMiracleDataInfo.miracle_id)
    pub miracle_id: u32,
    // special fields
    // @@protoc_insertion_point(special_field:RogueMiracleDataInfo.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a RogueMiracleDataInfo {
    fn default() -> &'a RogueMiracleDataInfo {
        <RogueMiracleDataInfo as ::protobuf::Message>::default_instance()
    }
}

impl RogueMiracleDataInfo {
    pub fn new() -> RogueMiracleDataInfo {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "DCLABPANBJG",
            |m: &RogueMiracleDataInfo| { &m.DCLABPANBJG },
            |m: &mut RogueMiracleDataInfo| { &mut m.DCLABPANBJG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_map_simpler_accessor::<_, _, _>(
            "KONAGCDHKKH",
            |m: &RogueMiracleDataInfo| { &m.KONAGCDHKKH },
            |m: &mut RogueMiracleDataInfo| { &mut m.KONAGCDHKKH },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "MMFONFFPNJM",
            |m: &RogueMiracleDataInfo| { &m.MMFONFFPNJM },
            |m: &mut RogueMiracleDataInfo| { &mut m.MMFONFFPNJM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "NKBEHFHLPEF",
            |m: &RogueMiracleDataInfo| { &m.NKBEHFHLPEF },
            |m: &mut RogueMiracleDataInfo| { &mut m.NKBEHFHLPEF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "miracle_id",
            |m: &RogueMiracleDataInfo| { &m.miracle_id },
            |m: &mut RogueMiracleDataInfo| { &mut m.miracle_id },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<RogueMiracleDataInfo>(
            "RogueMiracleDataInfo",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for RogueMiracleDataInfo {
    const NAME: &'static str = "RogueMiracleDataInfo";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                96 => {
                    self.DCLABPANBJG = is.read_uint32()?;
                },
                122 => {
                    let len = is.read_raw_varint32()?;
                    let old_limit = is.push_limit(len as u64)?;
                    let mut key = ::std::default::Default::default();
                    let mut value = ::std::default::Default::default();
                    while let Some(tag) = is.read_raw_tag_or_eof()? {
                        match tag {
                            8 => key = is.read_uint32()?,
                            16 => value = is.read_uint32()?,
                            _ => ::protobuf::rt::skip_field_for_tag(tag, is)?,
                        };
                    }
                    is.pop_limit(old_limit);
                    self.KONAGCDHKKH.insert(key, value);
                },
                64 => {
                    self.MMFONFFPNJM = is.read_uint32()?;
                },
                88 => {
                    self.NKBEHFHLPEF = is.read_uint32()?;
                },
                56 => {
                    self.miracle_id = is.read_uint32()?;
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if self.DCLABPANBJG != 0 {
            my_size += ::protobuf::rt::uint32_size(12, self.DCLABPANBJG);
        }
        for (k, v) in &self.KONAGCDHKKH {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::uint32_size(1, *k);
            entry_size += ::protobuf::rt::uint32_size(2, *v);
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(entry_size) + entry_size
        };
        if self.MMFONFFPNJM != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.MMFONFFPNJM);
        }
        if self.NKBEHFHLPEF != 0 {
            my_size += ::protobuf::rt::uint32_size(11, self.NKBEHFHLPEF);
        }
        if self.miracle_id != 0 {
            my_size += ::protobuf::rt::uint32_size(7, self.miracle_id);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.DCLABPANBJG != 0 {
            os.write_uint32(12, self.DCLABPANBJG)?;
        }
        for (k, v) in &self.KONAGCDHKKH {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::uint32_size(1, *k);
            entry_size += ::protobuf::rt::uint32_size(2, *v);
            os.write_raw_varint32(122)?; // Tag.
            os.write_raw_varint32(entry_size as u32)?;
            os.write_uint32(1, *k)?;
            os.write_uint32(2, *v)?;
        };
        if self.MMFONFFPNJM != 0 {
            os.write_uint32(8, self.MMFONFFPNJM)?;
        }
        if self.NKBEHFHLPEF != 0 {
            os.write_uint32(11, self.NKBEHFHLPEF)?;
        }
        if self.miracle_id != 0 {
            os.write_uint32(7, self.miracle_id)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> RogueMiracleDataInfo {
        RogueMiracleDataInfo::new()
    }

    fn clear(&mut self) {
        self.DCLABPANBJG = 0;
        self.KONAGCDHKKH.clear();
        self.MMFONFFPNJM = 0;
        self.NKBEHFHLPEF = 0;
        self.miracle_id = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static RogueMiracleDataInfo {
        static instance: ::protobuf::rt::Lazy<RogueMiracleDataInfo> = ::protobuf::rt::Lazy::new();
        instance.get(RogueMiracleDataInfo::new)
    }
}

impl ::protobuf::MessageFull for RogueMiracleDataInfo {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("RogueMiracleDataInfo").unwrap()).clone()
    }
}

impl ::std::fmt::Display for RogueMiracleDataInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RogueMiracleDataInfo {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1aRogueMiracleDataInfo.proto\"\xa5\x02\n\x14RogueMiracleDataInfo\x12\
    \x20\n\x0bDCLABPANBJG\x18\x0c\x20\x01(\rR\x0bDCLABPANBJG\x12H\n\x0bKONAG\
    CDHKKH\x18\x0f\x20\x03(\x0b2&.RogueMiracleDataInfo.KONAGCDHKKHEntryR\x0b\
    KONAGCDHKKH\x12\x20\n\x0bMMFONFFPNJM\x18\x08\x20\x01(\rR\x0bMMFONFFPNJM\
    \x12\x20\n\x0bNKBEHFHLPEF\x18\x0b\x20\x01(\rR\x0bNKBEHFHLPEF\x12\x1d\n\n\
    miracle_id\x18\x07\x20\x01(\rR\tmiracleId\x1a>\n\x10KONAGCDHKKHEntry\x12\
    \x10\n\x03key\x18\x01\x20\x01(\rR\x03key\x12\x14\n\x05value\x18\x02\x20\
    \x01(\rR\x05value:\x028\x01B\x15\n\x13emu.lunarcore.protob\x06proto3\
";

/// `FileDescriptorProto` object which was a source for this generated file
fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    static file_descriptor_proto_lazy: ::protobuf::rt::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::Lazy::new();
    file_descriptor_proto_lazy.get(|| {
        ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
    })
}

/// `FileDescriptor` object which allows dynamic access to files
pub fn file_descriptor() -> &'static ::protobuf::reflect::FileDescriptor {
    static generated_file_descriptor_lazy: ::protobuf::rt::Lazy<::protobuf::reflect::GeneratedFileDescriptor> = ::protobuf::rt::Lazy::new();
    static file_descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::FileDescriptor> = ::protobuf::rt::Lazy::new();
    file_descriptor.get(|| {
        let generated_file_descriptor = generated_file_descriptor_lazy.get(|| {
            let mut deps = ::std::vec::Vec::with_capacity(0);
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(RogueMiracleDataInfo::generated_message_descriptor_data());
            let mut enums = ::std::vec::Vec::with_capacity(0);
            ::protobuf::reflect::GeneratedFileDescriptor::new_generated(
                file_descriptor_proto(),
                deps,
                messages,
                enums,
            )
        });
        ::protobuf::reflect::FileDescriptor::new_generated_2(generated_file_descriptor)
    })
}
