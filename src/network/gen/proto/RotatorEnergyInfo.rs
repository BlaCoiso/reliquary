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

//! Generated file from `RotatorEnergyInfo.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:RotatorEnergyInfo)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct RotatorEnergyInfo {
    // message fields
    // @@protoc_insertion_point(field:RotatorEnergyInfo.max_num)
    pub max_num: u32,
    // @@protoc_insertion_point(field:RotatorEnergyInfo.cur_num)
    pub cur_num: u32,
    // special fields
    // @@protoc_insertion_point(special_field:RotatorEnergyInfo.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a RotatorEnergyInfo {
    fn default() -> &'a RotatorEnergyInfo {
        <RotatorEnergyInfo as ::protobuf::Message>::default_instance()
    }
}

impl RotatorEnergyInfo {
    pub fn new() -> RotatorEnergyInfo {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "max_num",
            |m: &RotatorEnergyInfo| { &m.max_num },
            |m: &mut RotatorEnergyInfo| { &mut m.max_num },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "cur_num",
            |m: &RotatorEnergyInfo| { &m.cur_num },
            |m: &mut RotatorEnergyInfo| { &mut m.cur_num },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<RotatorEnergyInfo>(
            "RotatorEnergyInfo",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for RotatorEnergyInfo {
    const NAME: &'static str = "RotatorEnergyInfo";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                24 => {
                    self.max_num = is.read_uint32()?;
                },
                56 => {
                    self.cur_num = is.read_uint32()?;
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
        if self.max_num != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.max_num);
        }
        if self.cur_num != 0 {
            my_size += ::protobuf::rt::uint32_size(7, self.cur_num);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.max_num != 0 {
            os.write_uint32(3, self.max_num)?;
        }
        if self.cur_num != 0 {
            os.write_uint32(7, self.cur_num)?;
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

    fn new() -> RotatorEnergyInfo {
        RotatorEnergyInfo::new()
    }

    fn clear(&mut self) {
        self.max_num = 0;
        self.cur_num = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static RotatorEnergyInfo {
        static instance: RotatorEnergyInfo = RotatorEnergyInfo {
            max_num: 0,
            cur_num: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for RotatorEnergyInfo {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("RotatorEnergyInfo").unwrap()).clone()
    }
}

impl ::std::fmt::Display for RotatorEnergyInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RotatorEnergyInfo {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x17RotatorEnergyInfo.proto\"E\n\x11RotatorEnergyInfo\x12\x17\n\x07max\
    _num\x18\x03\x20\x01(\rR\x06maxNum\x12\x17\n\x07cur_num\x18\x07\x20\x01(\
    \rR\x06curNumB\x15\n\x13emu.lunarcore.protob\x06proto3\
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
            messages.push(RotatorEnergyInfo::generated_message_descriptor_data());
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
