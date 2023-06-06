#[derive(Clone,PartialEq,serde::Serialize,serde::Deserialize,Default)]
pub enum ProviderType
{
    #[default] ConnectorExternal,
    ConnectorVertical
}