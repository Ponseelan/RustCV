#[repr(C)]
#[derive(Clone,serde::Serialize,serde::Deserialize,Default)]
pub struct ManagedPropertySettingItem
{
    pub Name:String
}