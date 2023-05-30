#[derive(PartialEq,Clone)]
pub struct ProviderExecutionResult
{
    pub ProviderName: String,
    pub Success : bool,
    pub Latency : i32,
    pub ResultsCount : i32,
    pub ProviderDiagnostics : Vec<String>
}


impl Default for ProviderExecutionResult
{
    fn default() -> Self
    {
        ProviderExecutionResult
        {
            ProviderName: "".to_string(),
            Success : false,
            Latency : 0,
            ResultsCount : 0,
            ProviderDiagnostics : Vec::new()
        }
    }
}