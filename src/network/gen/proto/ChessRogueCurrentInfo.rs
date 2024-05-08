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

//! Generated file from `ChessRogueCurrentInfo.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:ChessRogueCurrentInfo)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct ChessRogueCurrentInfo {
    // message fields
    // @@protoc_insertion_point(field:ChessRogueCurrentInfo.story_info)
    pub story_info: ::protobuf::MessageField<super::ChessRogueNousStoryInfo::ChessRogueNousStoryInfo>,
    // @@protoc_insertion_point(field:ChessRogueCurrentInfo.rogue_avatar_info)
    pub rogue_avatar_info: ::protobuf::MessageField<super::ChessRogueAvatarInfo::ChessRogueAvatarInfo>,
    // @@protoc_insertion_point(field:ChessRogueCurrentInfo.rogue_version_id)
    pub rogue_version_id: u32,
    // @@protoc_insertion_point(field:ChessRogueCurrentInfo.virtual_item_info)
    pub virtual_item_info: ::std::vec::Vec<super::ChessRogueVirtualItemInfo::ChessRogueVirtualItemInfo>,
    // @@protoc_insertion_point(field:ChessRogueCurrentInfo.buff_info)
    pub buff_info: ::protobuf::MessageField<super::ChessRogueBuffInfo::ChessRogueBuffInfo>,
    // @@protoc_insertion_point(field:ChessRogueCurrentInfo.pending_action)
    pub pending_action: ::protobuf::MessageField<super::RogueCommonPendingAction::RogueCommonPendingAction>,
    // @@protoc_insertion_point(field:ChessRogueCurrentInfo.nous_value)
    pub nous_value: ::protobuf::MessageField<super::ChessRogueNousValue::ChessRogueNousValue>,
    // @@protoc_insertion_point(field:ChessRogueCurrentInfo.miracle_info)
    pub miracle_info: ::protobuf::MessageField<super::ChessRogueMiracleInfo::ChessRogueMiracleInfo>,
    // @@protoc_insertion_point(field:ChessRogueCurrentInfo.dice_info)
    pub dice_info: ::protobuf::MessageField<super::ChessRogueNousDice::ChessRogueNousDice>,
    // special fields
    // @@protoc_insertion_point(special_field:ChessRogueCurrentInfo.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a ChessRogueCurrentInfo {
    fn default() -> &'a ChessRogueCurrentInfo {
        <ChessRogueCurrentInfo as ::protobuf::Message>::default_instance()
    }
}

impl ChessRogueCurrentInfo {
    pub fn new() -> ChessRogueCurrentInfo {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(9);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::ChessRogueNousStoryInfo::ChessRogueNousStoryInfo>(
            "story_info",
            |m: &ChessRogueCurrentInfo| { &m.story_info },
            |m: &mut ChessRogueCurrentInfo| { &mut m.story_info },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::ChessRogueAvatarInfo::ChessRogueAvatarInfo>(
            "rogue_avatar_info",
            |m: &ChessRogueCurrentInfo| { &m.rogue_avatar_info },
            |m: &mut ChessRogueCurrentInfo| { &mut m.rogue_avatar_info },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "rogue_version_id",
            |m: &ChessRogueCurrentInfo| { &m.rogue_version_id },
            |m: &mut ChessRogueCurrentInfo| { &mut m.rogue_version_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "virtual_item_info",
            |m: &ChessRogueCurrentInfo| { &m.virtual_item_info },
            |m: &mut ChessRogueCurrentInfo| { &mut m.virtual_item_info },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::ChessRogueBuffInfo::ChessRogueBuffInfo>(
            "buff_info",
            |m: &ChessRogueCurrentInfo| { &m.buff_info },
            |m: &mut ChessRogueCurrentInfo| { &mut m.buff_info },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::RogueCommonPendingAction::RogueCommonPendingAction>(
            "pending_action",
            |m: &ChessRogueCurrentInfo| { &m.pending_action },
            |m: &mut ChessRogueCurrentInfo| { &mut m.pending_action },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::ChessRogueNousValue::ChessRogueNousValue>(
            "nous_value",
            |m: &ChessRogueCurrentInfo| { &m.nous_value },
            |m: &mut ChessRogueCurrentInfo| { &mut m.nous_value },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::ChessRogueMiracleInfo::ChessRogueMiracleInfo>(
            "miracle_info",
            |m: &ChessRogueCurrentInfo| { &m.miracle_info },
            |m: &mut ChessRogueCurrentInfo| { &mut m.miracle_info },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::ChessRogueNousDice::ChessRogueNousDice>(
            "dice_info",
            |m: &ChessRogueCurrentInfo| { &m.dice_info },
            |m: &mut ChessRogueCurrentInfo| { &mut m.dice_info },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<ChessRogueCurrentInfo>(
            "ChessRogueCurrentInfo",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for ChessRogueCurrentInfo {
    const NAME: &'static str = "ChessRogueCurrentInfo";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                82 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.story_info)?;
                },
                74 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.rogue_avatar_info)?;
                },
                48 => {
                    self.rogue_version_id = is.read_uint32()?;
                },
                34 => {
                    self.virtual_item_info.push(is.read_message()?);
                },
                98 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.buff_info)?;
                },
                90 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.pending_action)?;
                },
                10 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.nous_value)?;
                },
                114 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.miracle_info)?;
                },
                122 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.dice_info)?;
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
        if let Some(v) = self.story_info.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.rogue_avatar_info.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.rogue_version_id != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.rogue_version_id);
        }
        for value in &self.virtual_item_info {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if let Some(v) = self.buff_info.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.pending_action.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.nous_value.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.miracle_info.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.dice_info.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.story_info.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(10, v, os)?;
        }
        if let Some(v) = self.rogue_avatar_info.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(9, v, os)?;
        }
        if self.rogue_version_id != 0 {
            os.write_uint32(6, self.rogue_version_id)?;
        }
        for v in &self.virtual_item_info {
            ::protobuf::rt::write_message_field_with_cached_size(4, v, os)?;
        };
        if let Some(v) = self.buff_info.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(12, v, os)?;
        }
        if let Some(v) = self.pending_action.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(11, v, os)?;
        }
        if let Some(v) = self.nous_value.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        }
        if let Some(v) = self.miracle_info.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(14, v, os)?;
        }
        if let Some(v) = self.dice_info.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(15, v, os)?;
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

    fn new() -> ChessRogueCurrentInfo {
        ChessRogueCurrentInfo::new()
    }

    fn clear(&mut self) {
        self.story_info.clear();
        self.rogue_avatar_info.clear();
        self.rogue_version_id = 0;
        self.virtual_item_info.clear();
        self.buff_info.clear();
        self.pending_action.clear();
        self.nous_value.clear();
        self.miracle_info.clear();
        self.dice_info.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static ChessRogueCurrentInfo {
        static instance: ChessRogueCurrentInfo = ChessRogueCurrentInfo {
            story_info: ::protobuf::MessageField::none(),
            rogue_avatar_info: ::protobuf::MessageField::none(),
            rogue_version_id: 0,
            virtual_item_info: ::std::vec::Vec::new(),
            buff_info: ::protobuf::MessageField::none(),
            pending_action: ::protobuf::MessageField::none(),
            nous_value: ::protobuf::MessageField::none(),
            miracle_info: ::protobuf::MessageField::none(),
            dice_info: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for ChessRogueCurrentInfo {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("ChessRogueCurrentInfo").unwrap()).clone()
    }
}

impl ::std::fmt::Display for ChessRogueCurrentInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ChessRogueCurrentInfo {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1bChessRogueCurrentInfo.proto\x1a\x1dChessRogueNousStoryInfo.proto\
    \x1a\x1aChessRogueAvatarInfo.proto\x1a\x1fChessRogueVirtualItemInfo.prot\
    o\x1a\x1eRogueCommonPendingAction.proto\x1a\x18ChessRogueBuffInfo.proto\
    \x1a\x1bChessRogueMiracleInfo.proto\x1a\x18ChessRogueNousDice.proto\x1a\
    \x19ChessRogueNousValue.proto\"\x9b\x04\n\x15ChessRogueCurrentInfo\x127\
    \n\nstory_info\x18\n\x20\x01(\x0b2\x18.ChessRogueNousStoryInfoR\tstoryIn\
    fo\x12A\n\x11rogue_avatar_info\x18\t\x20\x01(\x0b2\x15.ChessRogueAvatarI\
    nfoR\x0frogueAvatarInfo\x12(\n\x10rogue_version_id\x18\x06\x20\x01(\rR\
    \x0erogueVersionId\x12F\n\x11virtual_item_info\x18\x04\x20\x03(\x0b2\x1a\
    .ChessRogueVirtualItemInfoR\x0fvirtualItemInfo\x120\n\tbuff_info\x18\x0c\
    \x20\x01(\x0b2\x13.ChessRogueBuffInfoR\x08buffInfo\x12@\n\x0epending_act\
    ion\x18\x0b\x20\x01(\x0b2\x19.RogueCommonPendingActionR\rpendingAction\
    \x123\n\nnous_value\x18\x01\x20\x01(\x0b2\x14.ChessRogueNousValueR\tnous\
    Value\x129\n\x0cmiracle_info\x18\x0e\x20\x01(\x0b2\x16.ChessRogueMiracle\
    InfoR\x0bmiracleInfo\x120\n\tdice_info\x18\x0f\x20\x01(\x0b2\x13.ChessRo\
    gueNousDiceR\x08diceInfoB\x15\n\x13emu.lunarcore.protob\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(8);
            deps.push(super::ChessRogueNousStoryInfo::file_descriptor().clone());
            deps.push(super::ChessRogueAvatarInfo::file_descriptor().clone());
            deps.push(super::ChessRogueVirtualItemInfo::file_descriptor().clone());
            deps.push(super::RogueCommonPendingAction::file_descriptor().clone());
            deps.push(super::ChessRogueBuffInfo::file_descriptor().clone());
            deps.push(super::ChessRogueMiracleInfo::file_descriptor().clone());
            deps.push(super::ChessRogueNousDice::file_descriptor().clone());
            deps.push(super::ChessRogueNousValue::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(ChessRogueCurrentInfo::generated_message_descriptor_data());
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
