use metrics::counter;

use super::{emit, error_stage, error_type, ComponentEventsDropped, InternalEvent, UNINTENTIONAL};

#[derive(Debug)]
pub struct PollReadyError<E> {
    pub error: E,
}

impl<E: std::fmt::Debug> InternalEvent for PollReadyError<E> {
    fn emit(self) {
        error!(
            message = "Service poll ready failed.",
            error = ?self.error,
            error_type = error_type::REQUEST_FAILED,
            stage = error_stage::SENDING,
            internal_log_rate_limit = true,
        );
        counter!(
            "component_errors_total", 1,
            "error_type" => error_type::REQUEST_FAILED,
            "stage" => error_stage::SENDING,
        );
    }

    fn name(&self) -> Option<&'static str> {
        Some("ServicePollReadyError")
    }
}

#[derive(Debug)]
pub struct ServiceCallError<E> {
    pub error: E,
    pub request_id: usize,
    pub count: u64,
}

impl<E: std::fmt::Debug> InternalEvent for ServiceCallError<E> {
    fn emit(self) {
        let reason = "Service call failed. No retries or retries exhausted";
        error!(
            message = reason,
            error = ?self.error,
            request_id = self.request_id,
            error_type = error_type::REQUEST_FAILED,
            stage = error_stage::SENDING,
            internal_log_rate_secs = true,
        );
        counter!(
            "component_errors_total", 1,
            "error_type" => error_type::REQUEST_FAILED,
            "stage" => error_stage::SENDING,
        );

        emit(ComponentEventsDropped::<UNINTENTIONAL> {
            reason,
            count: self.count,
        });
    }
}
