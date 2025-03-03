// Copyright (c) RoochNetwork
// SPDX-License-Identifier: Apache-2.0

use crate::jsonrpc_types::{BytesView, StrView};
use anyhow::Result;
use move_core_types::{
    account_address::AccountAddress,
    identifier::Identifier,
    language_storage::{ModuleId, StructTag, TypeTag},
    u256,
};
use move_resource_viewer::{AnnotatedMoveStruct, AnnotatedMoveValue};
use moveos_types::move_std::string::MoveString;
use moveos_types::move_types::parse_module_id;
use moveos_types::moveos_std::event::{AnnotatedEvent, Event, EventID};
use moveos_types::moveos_std::type_info::TypeInfo;
use moveos_types::transaction::MoveAction;
use moveos_types::{
    access_path::AccessPath,
    event_filter::EventFilter,
    move_types::FunctionId,
    moveos_std::object::{AnnotatedObject, ObjectID},
    transaction::{FunctionCall, ScriptCall},
};
use moveos_types::{move_std::ascii::MoveAsciiString, state::MoveStructType};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeMap;
use std::str::FromStr;

pub type ModuleIdView = StrView<ModuleId>;
pub type TypeTagView = StrView<TypeTag>;
pub type StructTagView = StrView<StructTag>;
pub type FunctionIdView = StrView<FunctionId>;
pub type AccessPathView = StrView<AccessPath>;
pub type IdentifierView = StrView<Identifier>;

impl_str_view_for! {TypeTag StructTag FunctionId AccessPath Identifier}

pub type AccountAddressView = StrView<AccountAddress>;

impl std::fmt::Display for AccountAddressView {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        //Ensure append `0x` before the address, and output full address
        //The Display implemention of AccountAddress has not `0x` prefix
        write!(f, "{:#x}", self.0)
    }
}

impl FromStr for AccountAddressView {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // AccountAddress::from_str suppport both 0xADDRESS and ADDRESS
        Ok(StrView(AccountAddress::from_str(s)?))
    }
}

impl From<AccountAddressView> for AccountAddress {
    fn from(value: AccountAddressView) -> Self {
        value.0
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AnnotatedMoveStructView {
    pub abilities: u8,
    #[serde(rename = "type")]
    pub type_: StructTagView,
    //We use BTreeMap to Replace Vec to make the output more readable
    pub value: BTreeMap<Identifier, AnnotatedMoveValueView>,
}

impl From<AnnotatedMoveStruct> for AnnotatedMoveStructView {
    fn from(origin: AnnotatedMoveStruct) -> Self {
        Self {
            abilities: origin.abilities.into_u8(),
            type_: StrView(origin.type_),
            value: origin
                .value
                .into_iter()
                .map(|(k, v)| (k, v.into()))
                .collect(),
        }
    }
}

/// Some specific struct that we want to display in a special way for better readability
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum SpecificStructView {
    MoveString(MoveString),
    MoveAsciiString(MoveAsciiString),
    ObjectID(ObjectID),
}

impl SpecificStructView {
    pub fn try_from_annotated(move_struct: AnnotatedMoveStruct) -> Option<Self> {
        if MoveString::struct_tag_match(&move_struct.type_) {
            MoveString::try_from(move_struct)
                .ok()
                .map(SpecificStructView::MoveString)
        } else if MoveAsciiString::struct_tag_match(&move_struct.type_) {
            MoveAsciiString::try_from(move_struct)
                .ok()
                .map(SpecificStructView::MoveAsciiString)
        } else if ObjectID::struct_tag_match(&move_struct.type_) {
            ObjectID::try_from(move_struct)
                .ok()
                .map(SpecificStructView::ObjectID)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum AnnotatedMoveValueView {
    U8(u8),
    ///u64, u128, U256 is too large to be serialized in json
    /// so we use string to represent them
    U64(StrView<u64>),
    U128(StrView<u128>),
    Bool(bool),
    Address(AccountAddressView),
    Vector(Vec<AnnotatedMoveValueView>),
    Bytes(BytesView),
    Struct(AnnotatedMoveStructView),
    SpecificStruct(SpecificStructView),
    U16(u16),
    U32(u32),
    U256(StrView<u256::U256>),
}

impl From<AnnotatedMoveValue> for AnnotatedMoveValueView {
    fn from(origin: AnnotatedMoveValue) -> Self {
        match origin {
            AnnotatedMoveValue::U8(u) => AnnotatedMoveValueView::U8(u),
            AnnotatedMoveValue::U64(u) => AnnotatedMoveValueView::U64(StrView(u)),
            AnnotatedMoveValue::U128(u) => AnnotatedMoveValueView::U128(StrView(u)),
            AnnotatedMoveValue::Bool(b) => AnnotatedMoveValueView::Bool(b),
            AnnotatedMoveValue::Address(data) => AnnotatedMoveValueView::Address(StrView(data)),
            AnnotatedMoveValue::Vector(_type_tag, data) => {
                AnnotatedMoveValueView::Vector(data.into_iter().map(Into::into).collect())
            }
            AnnotatedMoveValue::Bytes(data) => AnnotatedMoveValueView::Bytes(StrView(data)),
            AnnotatedMoveValue::Struct(data) => {
                match SpecificStructView::try_from_annotated(data.clone()) {
                    Some(struct_view) => AnnotatedMoveValueView::SpecificStruct(struct_view),
                    None => AnnotatedMoveValueView::Struct(data.into()),
                }
            }
            AnnotatedMoveValue::U16(u) => AnnotatedMoveValueView::U16(u),
            AnnotatedMoveValue::U32(u) => AnnotatedMoveValueView::U32(u),
            AnnotatedMoveValue::U256(u) => AnnotatedMoveValueView::U256(StrView(u)),
        }
    }
}

//We can not support convert from AnnotatedMoveValueView to AnnotatedMoveValue
// It is not easy to implement because:
// 1. We need to put type_tag in the Vector
// 2. We need to support convert SpecificStruct to AnnotatedMoveStruct
// impl TryFrom<AnnotatedMoveValueView> for AnnotatedMoveValue {
//     type Error = anyhow::Error;
//     fn try_from(value: AnnotatedMoveValueView) -> Result<Self, Self::Error> {
//         Ok(match value {
//             AnnotatedMoveValueView::U8(u8) => AnnotatedMoveValue::U8(u8),
//             AnnotatedMoveValueView::U64(u64) => AnnotatedMoveValue::U64(u64.0),
//             AnnotatedMoveValueView::U128(u128) => AnnotatedMoveValue::U128(u128.0),
//             AnnotatedMoveValueView::Bool(bool) => AnnotatedMoveValue::Bool(bool),
//             AnnotatedMoveValueView::Address(address) => AnnotatedMoveValue::Address(address.0),
//             AnnotatedMoveValueView::Vector(type_tag, data) =>
//                 AnnotatedMoveValue::Vector(
//                 type_tag.0,
//                 data.into_iter()
//                     .map(AnnotatedMoveValue::try_from)
//                     .collect::<Result<Vec<_>, Self::Error>>()?,
//             ),
//             AnnotatedMoveValueView::Bytes(data) => AnnotatedMoveValue::Bytes(data.0),
//             AnnotatedMoveValueView::Struct(data) => AnnotatedMoveValue::Struct(data.try_into()?),
//             AnnotatedMoveValueView::SpecificStruct(data) => match data {
//                 SpecificStructView::MoveString(string) => AnnotatedMoveValue::Struct(string.into()),
//                 SpecificStructView::MoveAsciiString(string) => AnnotatedMoveValue::Struct(string.into()),
//                 SpecificStructView::ObjectID(id) => AnnotatedMoveValue::Struct(id.into()),
//             },
//             AnnotatedMoveValueView::U16(u16) => AnnotatedMoveValue::U16(u16),
//             AnnotatedMoveValueView::U32(u32) => AnnotatedMoveValue::U32(u32),
//             AnnotatedMoveValueView::U256(u256) => AnnotatedMoveValue::U256(u256.0),
//         })
//     }
// }

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AnnotatedObjectView {
    pub id: ObjectID,
    pub owner: AccountAddressView,
    pub value: AnnotatedMoveStructView,
}

impl From<AnnotatedObject> for AnnotatedObjectView {
    fn from(origin: AnnotatedObject) -> Self {
        Self {
            id: origin.id,
            owner: origin.owner.into(),
            value: origin.value.into(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
pub struct ScriptCallView {
    pub code: BytesView,
    pub ty_args: Vec<TypeTagView>,
    pub args: Vec<BytesView>,
}

impl From<ScriptCall> for ScriptCallView {
    fn from(origin: ScriptCall) -> Self {
        Self {
            code: origin.code.into(),
            ty_args: origin.ty_args.into_iter().map(StrView).collect(),
            args: origin.args.into_iter().map(StrView).collect(),
        }
    }
}

impl From<ScriptCallView> for ScriptCall {
    fn from(value: ScriptCallView) -> Self {
        Self {
            code: value.code.into(),
            ty_args: value.ty_args.into_iter().map(Into::into).collect(),
            args: value.args.into_iter().map(Into::into).collect(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct FunctionCallView {
    pub function_id: FunctionIdView,
    pub ty_args: Vec<TypeTagView>,
    pub args: Vec<BytesView>,
}

impl From<FunctionCall> for FunctionCallView {
    fn from(origin: FunctionCall) -> Self {
        Self {
            function_id: StrView(origin.function_id),
            ty_args: origin.ty_args.into_iter().map(StrView).collect(),
            args: origin.args.into_iter().map(StrView).collect(),
        }
    }
}

impl From<FunctionCallView> for FunctionCall {
    fn from(value: FunctionCallView) -> Self {
        Self {
            function_id: value.function_id.into(),
            ty_args: value.ty_args.into_iter().map(Into::into).collect(),
            args: value.args.into_iter().map(Into::into).collect(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct MoveActionView {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_call: Option<FunctionCallView>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub script_call: Option<ScriptCallView>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub module_bundle: Option<Vec<BytesView>>,
}

impl From<MoveAction> for MoveActionView {
    fn from(action: MoveAction) -> Self {
        match action {
            MoveAction::Script(script) => Self {
                script_call: Some(script.into()),
                function_call: None,
                module_bundle: None,
            },
            MoveAction::Function(fun) => Self {
                script_call: None,
                function_call: Some(fun.into()),
                module_bundle: None,
            },
            MoveAction::ModuleBundle(module) => Self {
                script_call: None,
                function_call: None,
                module_bundle: Some(module.into_iter().map(StrView).collect()),
            },
        }
    }
}

impl From<MoveActionView> for MoveAction {
    fn from(action: MoveActionView) -> Self {
        if let Some(script_call) = action.script_call {
            MoveAction::Script(script_call.into())
        } else if let Some(function_call) = action.function_call {
            MoveAction::Function(function_call.into())
        } else if let Some(module_bundle) = action.module_bundle {
            MoveAction::ModuleBundle(module_bundle.into_iter().map(StrView::into).collect())
        } else {
            panic!("Invalid MoveActionView")
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum MoveActionTypeView {
    ScriptCall,
    FunctionCall,
    ModuleBundle,
}

impl From<MoveAction> for MoveActionTypeView {
    fn from(action: MoveAction) -> Self {
        match action {
            MoveAction::Script(_) => Self::ScriptCall,
            MoveAction::Function(_) => Self::FunctionCall,
            MoveAction::ModuleBundle(_) => Self::ModuleBundle,
        }
    }
}

impl std::fmt::Display for StrView<ModuleId> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.0)
    }
}

impl FromStr for StrView<ModuleId> {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(parse_module_id(s)?))
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub enum EventFilterView {
    // /// Query by sender address.
    // Sender(AccountAddressView),
    // /// Return events emitted by the given transaction.
    // Transaction(
    //     ///tx hash of the transaction
    //     H256View,
    // ),
    /// Return events with the given move event struct name
    MoveEventType(
        // #[schemars(with = "String")]
        // #[serde_as(as = "TypeTag")]
        TypeTagView,
    ),
    MoveEventField {
        path: String,
        value: Value,
    },
    // /// Return events emitted in [start_time, end_time) interval
    // // #[serde(rename_all = "camelCase")]
    // TimeRange {
    //     /// left endpoint of time interval, milliseconds since epoch, inclusive
    //     // #[schemars(with = "u64")]
    //     // #[serde_as(as = "u64")]
    //     start_time: u64,
    //     /// right endpoint of time interval, milliseconds since epoch, exclusive
    //     // #[schemars(with = "u64")]
    //     // #[serde_as(as = "u64")]
    //     end_time: u64,
    // },
    /// Return events emitted in [from_block, to_block) interval
    // #[serde(rename_all = "camelCase")]
    // BlockRange {
    //     /// left endpoint of block height, inclusive
    //     // #[schemars(with = "u64")]
    //     // #[serde_as(as = "u64")]
    //     from_block: u64, //TODO use BlockNumber
    //     /// right endpoint of block height, exclusive
    //     // #[schemars(with = "u64")]
    //     // #[serde_as(as = "u64")]
    //     to_block: u64, //TODO use BlockNumber
    // },
    All(Vec<EventFilterView>),
    Any(Vec<EventFilterView>),
    And(Box<EventFilterView>, Box<EventFilterView>),
    Or(Box<EventFilterView>, Box<EventFilterView>),
}

impl From<EventFilterView> for EventFilter {
    fn from(value: EventFilterView) -> Self {
        match value {
            // EventFilterView::Sender(address) => Self::Sender(address.into()),
            // EventFilterView::Transaction(tx_hash) => Self::Transaction(tx_hash.into()),
            EventFilterView::MoveEventType(type_tag) => Self::MoveEventType(type_tag.into()),
            EventFilterView::MoveEventField { path, value } => Self::MoveEventField { path, value },
            // EventFilterView::TimeRange {
            //     start_time,
            //     end_time,
            // } => Self::TimeRange {
            //     start_time,
            //     end_time,
            // },
            EventFilterView::All(filters) => {
                Self::All(filters.into_iter().map(|f| f.into()).collect())
            }
            EventFilterView::Any(filters) => {
                Self::Any(filters.into_iter().map(|f| f.into()).collect())
            }
            EventFilterView::And(left, right) => {
                Self::And(Box::new((*left).into()), Box::new((*right).into()))
            }
            EventFilterView::Or(left, right) => {
                Self::Or(Box::new((*left).into()), Box::new((*right).into()))
            }
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct EventView {
    pub event_id: EventID,
    pub type_tag: TypeTagView,
    pub event_data: BytesView,
    pub event_index: u64,
    pub decoded_event_data: Option<AnnotatedMoveStructView>,
}

impl From<Event> for EventView {
    fn from(event: Event) -> Self {
        EventView {
            event_id: event.event_id,
            type_tag: event.type_tag.into(),
            event_data: StrView(event.event_data),
            event_index: event.event_index,
            decoded_event_data: None,
        }
    }
}

impl From<EventView> for Event {
    fn from(event: EventView) -> Self {
        Event {
            event_id: event.event_id,
            type_tag: event.type_tag.into(),
            event_data: event.event_data.0,
            event_index: event.event_index,
        }
    }
}

impl From<AnnotatedEvent> for EventView {
    fn from(event: AnnotatedEvent) -> Self {
        EventView {
            event_id: event.event.event_id,
            type_tag: event.event.type_tag.into(),
            event_data: StrView(event.event.event_data),
            event_index: event.event.event_index,
            decoded_event_data: Some(event.decoded_event_data.into()),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TypeInfoView {
    pub account_address: AccountAddress,
    pub module_name: BytesView,
    pub struct_name: BytesView,
}

impl std::fmt::Display for TypeInfoView {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}::{:?}::{:?}",
            &self.account_address, self.module_name, &self.struct_name
        )
    }
}

impl From<TypeInfo> for TypeInfoView {
    fn from(type_info: TypeInfo) -> Self {
        TypeInfoView {
            account_address: type_info.account_address,
            module_name: type_info.module_name.into(),
            struct_name: type_info.struct_name.into(),
        }
    }
}

impl From<TypeInfoView> for TypeInfo {
    fn from(type_info: TypeInfoView) -> Self {
        TypeInfo {
            account_address: type_info.account_address,
            module_name: type_info.module_name.into(),
            struct_name: type_info.struct_name.into(),
        }
    }
}
