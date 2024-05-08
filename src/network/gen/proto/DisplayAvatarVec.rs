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

//! Generated file from `DisplayAvatarVec.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:DisplayAvatarVec)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct DisplayAvatarVec {
    // message fields
    // @@protoc_insertion_point(field:DisplayAvatarVec.display_avatar_list)
    pub display_avatar_list: ::std::vec::Vec<super::DisplayAvatar::DisplayAvatar>,
    // @@protoc_insertion_point(field:DisplayAvatarVec.is_display)
    pub is_display: bool,
    // special fields
    // @@protoc_insertion_point(special_field:DisplayAvatarVec.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a DisplayAvatarVec {
    fn default() -> &'a DisplayAvatarVec {
        <DisplayAvatarVec as ::protobuf::Message>::default_instance()
    }
}

impl DisplayAvatarVec {
    pub fn new() -> DisplayAvatarVec {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "display_avatar_list",
            |m: &DisplayAvatarVec| { &m.display_avatar_list },
            |m: &mut DisplayAvatarVec| { &mut m.display_avatar_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "is_display",
            |m: &DisplayAvatarVec| { &m.is_display },
            |m: &mut DisplayAvatarVec| { &mut m.is_display },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<DisplayAvatarVec>(
            "DisplayAvatarVec",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for DisplayAvatarVec {
    const NAME: &'static str = "DisplayAvatarVec";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                114 => {
                    self.display_avatar_list.push(is.read_message()?);
                },
                40 => {
                    self.is_display = is.read_bool()?;
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
        for value in &self.display_avatar_list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.is_display != false {
            my_size += 1 + 1;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.display_avatar_list {
            ::protobuf::rt::write_message_field_with_cached_size(14, v, os)?;
        };
        if self.is_display != false {
            os.write_bool(5, self.is_display)?;
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

    fn new() -> DisplayAvatarVec {
        DisplayAvatarVec::new()
    }

    fn clear(&mut self) {
        self.display_avatar_list.clear();
        self.is_display = false;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static DisplayAvatarVec {
        static instance: DisplayAvatarVec = DisplayAvatarVec {
            display_avatar_list: ::std::vec::Vec::new(),
            is_display: false,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for DisplayAvatarVec {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("DisplayAvatarVec").unwrap()).clone()
    }
}

impl ::std::fmt::Display for DisplayAvatarVec {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DisplayAvatarVec {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x16DisplayAvatarVec.proto\x1a\x13DisplayAvatar.proto\"q\n\x10DisplayA\
    vatarVec\x12>\n\x13display_avatar_list\x18\x0e\x20\x03(\x0b2\x0e.Display\
    AvatarR\x11displayAvatarList\x12\x1d\n\nis_display\x18\x05\x20\x01(\x08R\
    \tisDisplayB\x15\n\x13emu.lunarcore.protob\x06proto3\
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
            deps.push(super::DisplayAvatar::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(DisplayAvatarVec::generated_message_descriptor_data());
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
