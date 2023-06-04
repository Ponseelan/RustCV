

#[derive(Clone,PartialEq,serde::Serialize,serde::Deserialize,Default)]
pub enum EntityType
{
    #[default] External,
    Files
}