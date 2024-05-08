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

//! Generated file from `RogueFinishInfo.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:RogueFinishInfo)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct RogueFinishInfo {
    // message fields
    // @@protoc_insertion_point(field:RogueFinishInfo.cur_score_reward_info)
    pub cur_score_reward_info: ::protobuf::MessageField<super::RogueScoreRewardInfo::RogueScoreRewardInfo>,
    // @@protoc_insertion_point(field:RogueFinishInfo.score_reward_info)
    pub score_reward_info: ::protobuf::MessageField<super::RogueScoreRewardInfo::RogueScoreRewardInfo>,
    // @@protoc_insertion_point(field:RogueFinishInfo.record_info)
    pub record_info: ::protobuf::MessageField<super::RogueRecordInfo::RogueRecordInfo>,
    // @@protoc_insertion_point(field:RogueFinishInfo.area_id)
    pub area_id: u32,
    // @@protoc_insertion_point(field:RogueFinishInfo.total_score)
    pub total_score: u32,
    // @@protoc_insertion_point(field:RogueFinishInfo.is_win)
    pub is_win: bool,
    // @@protoc_insertion_point(field:RogueFinishInfo.reach_room_count)
    pub reach_room_count: u32,
    // @@protoc_insertion_point(field:RogueFinishInfo.pass_room_count)
    pub pass_room_count: u32,
    // @@protoc_insertion_point(field:RogueFinishInfo.taken_score)
    pub taken_score: u32,
    // special fields
    // @@protoc_insertion_point(special_field:RogueFinishInfo.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a RogueFinishInfo {
    fn default() -> &'a RogueFinishInfo {
        <RogueFinishInfo as ::protobuf::Message>::default_instance()
    }
}

impl RogueFinishInfo {
    pub fn new() -> RogueFinishInfo {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(9);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::RogueScoreRewardInfo::RogueScoreRewardInfo>(
            "cur_score_reward_info",
            |m: &RogueFinishInfo| { &m.cur_score_reward_info },
            |m: &mut RogueFinishInfo| { &mut m.cur_score_reward_info },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::RogueScoreRewardInfo::RogueScoreRewardInfo>(
            "score_reward_info",
            |m: &RogueFinishInfo| { &m.score_reward_info },
            |m: &mut RogueFinishInfo| { &mut m.score_reward_info },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::RogueRecordInfo::RogueRecordInfo>(
            "record_info",
            |m: &RogueFinishInfo| { &m.record_info },
            |m: &mut RogueFinishInfo| { &mut m.record_info },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "area_id",
            |m: &RogueFinishInfo| { &m.area_id },
            |m: &mut RogueFinishInfo| { &mut m.area_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "total_score",
            |m: &RogueFinishInfo| { &m.total_score },
            |m: &mut RogueFinishInfo| { &mut m.total_score },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "is_win",
            |m: &RogueFinishInfo| { &m.is_win },
            |m: &mut RogueFinishInfo| { &mut m.is_win },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "reach_room_count",
            |m: &RogueFinishInfo| { &m.reach_room_count },
            |m: &mut RogueFinishInfo| { &mut m.reach_room_count },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "pass_room_count",
            |m: &RogueFinishInfo| { &m.pass_room_count },
            |m: &mut RogueFinishInfo| { &mut m.pass_room_count },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "taken_score",
            |m: &RogueFinishInfo| { &m.taken_score },
            |m: &mut RogueFinishInfo| { &mut m.taken_score },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<RogueFinishInfo>(
            "RogueFinishInfo",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for RogueFinishInfo {
    const NAME: &'static str = "RogueFinishInfo";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                114 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.cur_score_reward_info)?;
                },
                58 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.score_reward_info)?;
                },
                42 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.record_info)?;
                },
                9048 => {
                    self.area_id = is.read_uint32()?;
                },
                72 => {
                    self.total_score = is.read_uint32()?;
                },
                32 => {
                    self.is_win = is.read_bool()?;
                },
                15288 => {
                    self.reach_room_count = is.read_uint32()?;
                },
                24 => {
                    self.pass_room_count = is.read_uint32()?;
                },
                80 => {
                    self.taken_score = is.read_uint32()?;
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
        if let Some(v) = self.cur_score_reward_info.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.score_reward_info.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.record_info.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.area_id != 0 {
            my_size += ::protobuf::rt::uint32_size(1131, self.area_id);
        }
        if self.total_score != 0 {
            my_size += ::protobuf::rt::uint32_size(9, self.total_score);
        }
        if self.is_win != false {
            my_size += 1 + 1;
        }
        if self.reach_room_count != 0 {
            my_size += ::protobuf::rt::uint32_size(1911, self.reach_room_count);
        }
        if self.pass_room_count != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.pass_room_count);
        }
        if self.taken_score != 0 {
            my_size += ::protobuf::rt::uint32_size(10, self.taken_score);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.cur_score_reward_info.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(14, v, os)?;
        }
        if let Some(v) = self.score_reward_info.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(7, v, os)?;
        }
        if let Some(v) = self.record_info.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(5, v, os)?;
        }
        if self.area_id != 0 {
            os.write_uint32(1131, self.area_id)?;
        }
        if self.total_score != 0 {
            os.write_uint32(9, self.total_score)?;
        }
        if self.is_win != false {
            os.write_bool(4, self.is_win)?;
        }
        if self.reach_room_count != 0 {
            os.write_uint32(1911, self.reach_room_count)?;
        }
        if self.pass_room_count != 0 {
            os.write_uint32(3, self.pass_room_count)?;
        }
        if self.taken_score != 0 {
            os.write_uint32(10, self.taken_score)?;
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

    fn new() -> RogueFinishInfo {
        RogueFinishInfo::new()
    }

    fn clear(&mut self) {
        self.cur_score_reward_info.clear();
        self.score_reward_info.clear();
        self.record_info.clear();
        self.area_id = 0;
        self.total_score = 0;
        self.is_win = false;
        self.reach_room_count = 0;
        self.pass_room_count = 0;
        self.taken_score = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static RogueFinishInfo {
        static instance: RogueFinishInfo = RogueFinishInfo {
            cur_score_reward_info: ::protobuf::MessageField::none(),
            score_reward_info: ::protobuf::MessageField::none(),
            record_info: ::protobuf::MessageField::none(),
            area_id: 0,
            total_score: 0,
            is_win: false,
            reach_room_count: 0,
            pass_room_count: 0,
            taken_score: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for RogueFinishInfo {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("RogueFinishInfo").unwrap()).clone()
    }
}

impl ::std::fmt::Display for RogueFinishInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RogueFinishInfo {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x15RogueFinishInfo.proto\x1a\x1aRogueScoreRewardInfo.proto\x1a\x15Rog\
    ueRecordInfo.proto\"\x97\x03\n\x0fRogueFinishInfo\x12H\n\x15cur_score_re\
    ward_info\x18\x0e\x20\x01(\x0b2\x15.RogueScoreRewardInfoR\x12curScoreRew\
    ardInfo\x12A\n\x11score_reward_info\x18\x07\x20\x01(\x0b2\x15.RogueScore\
    RewardInfoR\x0fscoreRewardInfo\x121\n\x0brecord_info\x18\x05\x20\x01(\
    \x0b2\x10.RogueRecordInfoR\nrecordInfo\x12\x18\n\x07area_id\x18\xeb\x08\
    \x20\x01(\rR\x06areaId\x12\x1f\n\x0btotal_score\x18\t\x20\x01(\rR\ntotal\
    Score\x12\x15\n\x06is_win\x18\x04\x20\x01(\x08R\x05isWin\x12)\n\x10reach\
    _room_count\x18\xf7\x0e\x20\x01(\rR\x0ereachRoomCount\x12&\n\x0fpass_roo\
    m_count\x18\x03\x20\x01(\rR\rpassRoomCount\x12\x1f\n\x0btaken_score\x18\
    \n\x20\x01(\rR\ntakenScoreB\x15\n\x13emu.lunarcore.protob\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(2);
            deps.push(super::RogueScoreRewardInfo::file_descriptor().clone());
            deps.push(super::RogueRecordInfo::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(RogueFinishInfo::generated_message_descriptor_data());
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
