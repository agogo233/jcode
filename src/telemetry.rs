#[derive(Clone, Copy)]
pub enum SessionEndReason {
    NormalExit,
    Unknown,
}

#[derive(Clone, Copy)]
pub enum ErrorCategory {
    RateLimited,
    ProviderTimeout,
    AuthFailed,
}

pub fn begin_session_with_parent(
    _provider: &str,
    _model: &str,
    _parent_id: Option<&str>,
    _is_continuation: bool,
) {
}

pub fn begin_resumed_session(_provider: &str, _model: &str) {}

pub fn end_session_with_reason(_provider: &str, _model: &str, _reason: SessionEndReason) {}

pub fn record_crash(_provider: &str, _model: &str, _reason: SessionEndReason) {}

pub fn record_turn() {}

pub fn record_tool_call() {}

pub fn record_tool_failure() {}

pub fn record_assistant_response() {}

pub fn record_connection_type(_connection: &str) {}

pub fn record_token_usage(_input: u64, _output: u64, _model: &str) {}

pub fn record_user_cancelled() {}

pub fn record_setup_step_once(_step: &str) {}

pub fn record_provider_selected(_provider_id: &str) {}

pub fn record_auth_started(_provider: &str, _method: &str) {}

pub fn record_auth_success(_provider: &str, _method: &str) {}

pub fn record_auth_cancelled(_provider: &str, _method: &str) {}

pub fn record_auth_failed_reason(_provider: &str, _method: &str, _reason: &str) {}

pub fn record_auth_surface_blocked(_provider: &str, _method: &str) {}

pub fn record_auth_surface_blocked_reason(_provider: &str, _method: &str, _reason: &str) {}

pub fn record_tool_execution(_name: &str, _input: &str, _success: bool, _latency_ms: u64) {}

pub fn record_model_switch() {}

pub fn record_provider_switch() {}

pub fn record_command_family(_command: &str) {}

pub fn record_error(_category: ErrorCategory) {}

pub fn record_memory_injected(_count: usize, _age_ms: u64) {}

pub fn record_feedback(_feedback: &str) {}
