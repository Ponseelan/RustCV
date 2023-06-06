
#[cfg(test)]
pub mod ConnectorHelperTest{
    use std::collections::HashMap;

    use ConnectorVertical::Model::FlexibleSchemaSettingItem::FlexibleSchemaSettingItem;
    use ConnectorVertical::Utils::ConnectorHelper::ConnectorHelper;
    use ConnectorVertical::Model::EntityRequest::EntityRequest;
    use ConnectorVertical::Model::VerticalSetting::VerticalSetting;

    #[test]
pub fn connector_helper_with_valid_vertical_setting()
{
let entityRequest=&create_entity_request();
let verticalSetting=create_vertical_setting();
let isValid=ConnectorHelper::IsValidFlexibleSchemaSettingForAllConnections(entityRequest,verticalSetting);
assert_eq!(isValid,true);
}


pub fn create_vertical_setting() -> VerticalSetting
{
    let mut verticalSetting=VerticalSetting::new();
    verticalSetting.ConnectionIds=vec!["TestConnectionId".to_string()];
    verticalSetting
}

pub fn create_entity_request() -> EntityRequest
{
    let mut entityRequest=EntityRequest::new();
    entityRequest.FlexibleSchemaSettings=HashMap::new();
    let mut flexibleSchemaSettingItem=FlexibleSchemaSettingItem::default();
    flexibleSchemaSettingItem.CandidatePropertySettings=HashMap::new();
    flexibleSchemaSettingItem.CandidatePropertySettings.insert("TestConnectionId".to_string(),vec![]);
    entityRequest.FlexibleSchemaSettings.insert("TestConnectionId".to_string(),flexibleSchemaSettingItem);
    entityRequest
}

}
