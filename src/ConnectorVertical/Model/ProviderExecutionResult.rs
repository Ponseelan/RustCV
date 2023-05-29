pub struct ProviderExecutionResult
{
    pub ProviderName: String,
    pub Success : bool,
    pub Latency : i32,
    pub ResultsCount : i32,
    pub ProviderDiagnostics : Vec<String>
}