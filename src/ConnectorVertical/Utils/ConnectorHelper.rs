use crate::ConnectorVertical::Model::{EntityRequest::EntityRequest, VerticalSetting::VerticalSetting};

pub struct ConnectorHelper{}


impl ConnectorHelper {
    pub fn IsValidFlexibleSchemaSettingForAllConnections(entityRequest:&EntityRequest,verticalSetting:VerticalSetting) -> bool
{
    let connectionIds=&verticalSetting.ConnectionIds;

    let mut isValidFlexibleSchemaSettingsForAllConnection:bool=true;

    let flexibleSchemaSettings=&entityRequest.FlexibleSchemaSettings;
    //iterate connection ids
    for connectionId in connectionIds
    {
        //check if connection id is present in flexible schema settings
        if(flexibleSchemaSettings.contains_key(connectionId))
        {
            let flexibleSchemaSettingItem=flexibleSchemaSettings.get(connectionId);
            let flexibleSchemaSettingItem=flexibleSchemaSettingItem.unwrap();
            let candidatePropertySettings=&flexibleSchemaSettingItem.CandidatePropertySettings;
            //check if candidate property settings is present
            if(candidatePropertySettings.len() == 0)
            {
                isValidFlexibleSchemaSettingsForAllConnection=false;
            }
           
        }
        else {
            isValidFlexibleSchemaSettingsForAllConnection=false;
        }
    }
    isValidFlexibleSchemaSettingsForAllConnection

}


}