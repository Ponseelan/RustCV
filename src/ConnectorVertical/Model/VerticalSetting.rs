#[repr(C)]
#[derive(Clone,serde::Serialize,serde::Deserialize,Default)]
pub struct VerticalSetting
{
    //list of string
    pub ConnectionIds:Vec<String>,
    pub DisplayName:String,
    pub EnableFanout:bool,
}

impl VerticalSetting
{
    pub fn new() -> VerticalSetting
    {
        VerticalSetting
        {
            ConnectionIds: Vec::new(),
            DisplayName: String::new(),
            EnableFanout: false
        }
    }
}
    