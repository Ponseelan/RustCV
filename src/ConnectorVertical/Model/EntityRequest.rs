use std::collections::HashMap;

use super::{EntityRequestType::EntityRequestType, Query::Query, EntityType::EntityType, ProviderType::ProviderType, SearchSettingsProvider, VerticalSetting::VerticalSetting, FlexibleSchemaSettingItem::FlexibleSchemaSettingItem};




#[repr(C)]
#[derive(Clone,serde::Serialize,serde::Deserialize,Default)]
pub struct EntityRequest
{
    #[serde(default)]
    pub EntityRequestType : EntityRequestType,
    #[serde(default)]
    pub query: Query,
    // #[serde(default)]
    // pub TenantId: String,
    #[serde(default)]
    pub CorrelationId: String,
    #[serde(default)]
    pub EntityType: EntityType,
    #[serde(default)]
    pub ProviderType:ProviderType,
    #[serde(default)]
    pub VerticalSettings:Vec<VerticalSetting>,
    #[serde(default)]
    pub FlexibleSchemaSettings:HashMap<String,FlexibleSchemaSettingItem>
}


impl EntityRequest{
    pub fn new() -> EntityRequest
    {
        EntityRequest
        {
            EntityRequestType: EntityRequestType::LssEntityRequest,
            query: Query::new(),
           // TenantId: String::new(),
            CorrelationId: String::new(),
            EntityType: EntityType::External,
            ProviderType: ProviderType::ConnectorExternal,
            VerticalSettings: Vec::new(),
            FlexibleSchemaSettings: HashMap::new()
        }
    }
}