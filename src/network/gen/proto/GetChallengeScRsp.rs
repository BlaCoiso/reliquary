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

//! Generated file from `GetChallengeScRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:GetChallengeScRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct GetChallengeScRsp {
    // message fields
    // @@protoc_insertion_point(field:GetChallengeScRsp.challenge_list)
    pub challenge_list: ::std::vec::Vec<super::Challenge::Challenge>,
    // @@protoc_insertion_point(field:GetChallengeScRsp.retcode)
    pub retcode: u32,
    // @@protoc_insertion_point(field:GetChallengeScRsp.challenge_reward_list)
    pub challenge_reward_list: ::std::vec::Vec<super::ChallengeReward::ChallengeReward>,
    // special fields
    // @@protoc_insertion_point(special_field:GetChallengeScRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a GetChallengeScRsp {
    fn default() -> &'a GetChallengeScRsp {
        <GetChallengeScRsp as ::protobuf::Message>::default_instance()
    }
}

impl GetChallengeScRsp {
    pub fn new() -> GetChallengeScRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "challenge_list",
            |m: &GetChallengeScRsp| { &m.challenge_list },
            |m: &mut GetChallengeScRsp| { &mut m.challenge_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &GetChallengeScRsp| { &m.retcode },
            |m: &mut GetChallengeScRsp| { &mut m.retcode },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "challenge_reward_list",
            |m: &GetChallengeScRsp| { &m.challenge_reward_list },
            |m: &mut GetChallengeScRsp| { &mut m.challenge_reward_list },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<GetChallengeScRsp>(
            "GetChallengeScRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for GetChallengeScRsp {
    const NAME: &'static str = "GetChallengeScRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                90 => {
                    self.challenge_list.push(is.read_message()?);
                },
                8 => {
                    self.retcode = is.read_uint32()?;
                },
                58 => {
                    self.challenge_reward_list.push(is.read_message()?);
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
        for value in &self.challenge_list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.retcode != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.retcode);
        }
        for value in &self.challenge_reward_list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.challenge_list {
            ::protobuf::rt::write_message_field_with_cached_size(11, v, os)?;
        };
        if self.retcode != 0 {
            os.write_uint32(1, self.retcode)?;
        }
        for v in &self.challenge_reward_list {
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

    fn new() -> GetChallengeScRsp {
        GetChallengeScRsp::new()
    }

    fn clear(&mut self) {
        self.challenge_list.clear();
        self.retcode = 0;
        self.challenge_reward_list.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static GetChallengeScRsp {
        static instance: GetChallengeScRsp = GetChallengeScRsp {
            challenge_list: ::std::vec::Vec::new(),
            retcode: 0,
            challenge_reward_list: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for GetChallengeScRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("GetChallengeScRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for GetChallengeScRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetChallengeScRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x17GetChallengeScRsp.proto\x1a\x0fChallenge.proto\x1a\x15ChallengeRew\
    ard.proto\"\xa6\x01\n\x11GetChallengeScRsp\x121\n\x0echallenge_list\x18\
    \x0b\x20\x03(\x0b2\n.ChallengeR\rchallengeList\x12\x18\n\x07retcode\x18\
    \x01\x20\x01(\rR\x07retcode\x12D\n\x15challenge_reward_list\x18\x07\x20\
    \x03(\x0b2\x10.ChallengeRewardR\x13challengeRewardListB\x15\n\x13emu.lun\
    arcore.protob\x06proto3\
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
            deps.push(super::Challenge::file_descriptor().clone());
            deps.push(super::ChallengeReward::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(GetChallengeScRsp::generated_message_descriptor_data());
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
