#[derive(Clone,serde::Serialize,serde::Deserialize,Default)]
pub enum EntityRequestType
{
    #[default] LssEntityRequest
}