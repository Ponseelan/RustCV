use std::collections::HashMap;

use super::{ManagedPropertySettingItem::ManagedPropertySettingItem, StorePropertyDefinitionSettingItem::StorePropertyDefinitionSettingItem};

#[repr(C)]
#[derive(Clone,serde::Serialize,serde::Deserialize,Default)]
pub struct FlexibleSchemaSettingItem
{
    //Dictionarry of string and Event
    pub CandidatePropertySettings:HashMap<String,Vec<ManagedPropertySettingItem>>,
    pub AllSearchableProperties:Vec<StorePropertyDefinitionSettingItem>
}

