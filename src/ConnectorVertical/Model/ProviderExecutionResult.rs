#[derive(PartialEq,Clone,serde::Serialize,serde::Deserialize,Debug,Default)]
pub struct ProviderExecutionResult
{
    pub ProviderName: String,
    pub Success : bool,
    pub Latency : i32,
    pub ResultsCount : i32,
    #[serde(default)]
    pub ProviderDiagnostics : Vec<String>
}