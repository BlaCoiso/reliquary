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

//! Generated file from `SceneEntityGroupInfo.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:SceneEntityGroupInfo)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct SceneEntityGroupInfo {
    // message fields
    // @@protoc_insertion_point(field:SceneEntityGroupInfo.state)
    pub state: u32,
    // @@protoc_insertion_point(field:SceneEntityGroupInfo.group_id)
    pub group_id: u32,
    // @@protoc_insertion_point(field:SceneEntityGroupInfo.entity_list)
    pub entity_list: ::std::vec::Vec<super::SceneEntityInfo::SceneEntityInfo>,
    // special fields
    // @@protoc_insertion_point(special_field:SceneEntityGroupInfo.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a SceneEntityGroupInfo {
    fn default() -> &'a SceneEntityGroupInfo {
        <SceneEntityGroupInfo as ::protobuf::Message>::default_instance()
    }
}

impl SceneEntityGroupInfo {
    pub fn new() -> SceneEntityGroupInfo {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "state",
            |m: &SceneEntityGroupInfo| { &m.state },
            |m: &mut SceneEntityGroupInfo| { &mut m.state },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "group_id",
            |m: &SceneEntityGroupInfo| { &m.group_id },
            |m: &mut SceneEntityGroupInfo| { &mut m.group_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "entity_list",
            |m: &SceneEntityGroupInfo| { &m.entity_list },
            |m: &mut SceneEntityGroupInfo| { &mut m.entity_list },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<SceneEntityGroupInfo>(
            "SceneEntityGroupInfo",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for SceneEntityGroupInfo {
    const NAME: &'static str = "SceneEntityGroupInfo";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                48 => {
                    self.state = is.read_uint32()?;
                },
                32 => {
                    self.group_id = is.read_uint32()?;
                },
                58 => {
                    self.entity_list.push(is.read_message()?);
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
        if self.state != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.state);
        }
        if self.group_id != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.group_id);
        }
        for value in &self.entity_list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.state != 0 {
            os.write_uint32(6, self.state)?;
        }
        if self.group_id != 0 {
            os.write_uint32(4, self.group_id)?;
        }
        for v in &self.entity_list {
            ::protobuf::rt::write_message_field_with_cached_size(7, v, os)?;
        };
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> SceneEntityGroupInfo {
        SceneEntityGroupInfo::new()
    }

    fn clear(&mut self) {
        self.state = 0;
        self.group_id = 0;
        self.entity_list.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static SceneEntityGroupInfo {
        static instance: SceneEntityGroupInfo = SceneEntityGroupInfo {
            state: 0,
            group_id: 0,
            entity_list: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for SceneEntityGroupInfo {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("SceneEntityGroupInfo").unwrap()).clone()
    }
}

impl ::std::fmt::Display for SceneEntityGroupInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SceneEntityGroupInfo {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1aSceneEntityGroupInfo.proto\x1a\x15SceneEntityInfo.proto\"z\n\x14Sc\
    eneEntityGroupInfo\x12\x14\n\x05state\x18\x06\x20\x01(\rR\x05state\x12\
    \x19\n\x08group_id\x18\x04\x20\x01(\rR\x07groupId\x121\n\x0bentity_list\
    \x18\x07\x20\x03(\x0b2\x10.SceneEntityInfoR\nentityListB\x15\n\x13emu.lu\
    narcore.protob\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(1);
            deps.push(super::SceneEntityInfo::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(SceneEntityGroupInfo::generated_message_descriptor_data());
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
