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

//! Generated file from `ScenePropInfo.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:ScenePropInfo)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct ScenePropInfo {
    // message fields
    // @@protoc_insertion_point(field:ScenePropInfo.life_time_ms)
    pub life_time_ms: u32,
    // @@protoc_insertion_point(field:ScenePropInfo.extra_info)
    pub extra_info: ::protobuf::MessageField<super::PropExtraInfo::PropExtraInfo>,
    // @@protoc_insertion_point(field:ScenePropInfo.prop_id)
    pub prop_id: u32,
    // @@protoc_insertion_point(field:ScenePropInfo.prop_state)
    pub prop_state: u32,
    // @@protoc_insertion_point(field:ScenePropInfo.create_time_ms)
    pub create_time_ms: u64,
    // special fields
    // @@protoc_insertion_point(special_field:ScenePropInfo.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a ScenePropInfo {
    fn default() -> &'a ScenePropInfo {
        <ScenePropInfo as ::protobuf::Message>::default_instance()
    }
}

impl ScenePropInfo {
    pub fn new() -> ScenePropInfo {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "life_time_ms",
            |m: &ScenePropInfo| { &m.life_time_ms },
            |m: &mut ScenePropInfo| { &mut m.life_time_ms },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::PropExtraInfo::PropExtraInfo>(
            "extra_info",
            |m: &ScenePropInfo| { &m.extra_info },
            |m: &mut ScenePropInfo| { &mut m.extra_info },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "prop_id",
            |m: &ScenePropInfo| { &m.prop_id },
            |m: &mut ScenePropInfo| { &mut m.prop_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "prop_state",
            |m: &ScenePropInfo| { &m.prop_state },
            |m: &mut ScenePropInfo| { &mut m.prop_state },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "create_time_ms",
            |m: &ScenePropInfo| { &m.create_time_ms },
            |m: &mut ScenePropInfo| { &mut m.create_time_ms },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<ScenePropInfo>(
            "ScenePropInfo",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for ScenePropInfo {
    const NAME: &'static str = "ScenePropInfo";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                56 => {
                    self.life_time_ms = is.read_uint32()?;
                },
                26 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.extra_info)?;
                },
                32 => {
                    self.prop_id = is.read_uint32()?;
                },
                80 => {
                    self.prop_state = is.read_uint32()?;
                },
                88 => {
                    self.create_time_ms = is.read_uint64()?;
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
        if self.life_time_ms != 0 {
            my_size += ::protobuf::rt::uint32_size(7, self.life_time_ms);
        }
        if let Some(v) = self.extra_info.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.prop_id != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.prop_id);
        }
        if self.prop_state != 0 {
            my_size += ::protobuf::rt::uint32_size(10, self.prop_state);
        }
        if self.create_time_ms != 0 {
            my_size += ::protobuf::rt::uint64_size(11, self.create_time_ms);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.life_time_ms != 0 {
            os.write_uint32(7, self.life_time_ms)?;
        }
        if let Some(v) = self.extra_info.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(3, v, os)?;
        }
        if self.prop_id != 0 {
            os.write_uint32(4, self.prop_id)?;
        }
        if self.prop_state != 0 {
            os.write_uint32(10, self.prop_state)?;
        }
        if self.create_time_ms != 0 {
            os.write_uint64(11, self.create_time_ms)?;
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

    fn new() -> ScenePropInfo {
        ScenePropInfo::new()
    }

    fn clear(&mut self) {
        self.life_time_ms = 0;
        self.extra_info.clear();
        self.prop_id = 0;
        self.prop_state = 0;
        self.create_time_ms = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static ScenePropInfo {
        static instance: ScenePropInfo = ScenePropInfo {
            life_time_ms: 0,
            extra_info: ::protobuf::MessageField::none(),
            prop_id: 0,
            prop_state: 0,
            create_time_ms: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for ScenePropInfo {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("ScenePropInfo").unwrap()).clone()
    }
}

impl ::std::fmt::Display for ScenePropInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ScenePropInfo {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x13ScenePropInfo.proto\x1a\x13PropExtraInfo.proto\"\xbe\x01\n\rSceneP\
    ropInfo\x12\x20\n\x0clife_time_ms\x18\x07\x20\x01(\rR\nlifeTimeMs\x12-\n\
    \nextra_info\x18\x03\x20\x01(\x0b2\x0e.PropExtraInfoR\textraInfo\x12\x17\
    \n\x07prop_id\x18\x04\x20\x01(\rR\x06propId\x12\x1d\n\nprop_state\x18\n\
    \x20\x01(\rR\tpropState\x12$\n\x0ecreate_time_ms\x18\x0b\x20\x01(\x04R\
    \x0ccreateTimeMsB\x15\n\x13emu.lunarcore.protob\x06proto3\
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
            deps.push(super::PropExtraInfo::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(ScenePropInfo::generated_message_descriptor_data());
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
