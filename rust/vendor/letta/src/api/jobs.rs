//! Job and batch processing API endpoints.

use crate::api::endpoints;
use crate::client::LettaClient;
use crate::error::LettaResult;
use crate::types::{
    Job, LettaId, LettaMessageUnion, ListJobsParams, ModifyFeedbackRequest, Step, StepFeedback,
    StepMetrics, TelemetryTrace,
};

/// Job API operations.
#[derive(Debug)]
pub struct JobApi<'a> {
    client: &'a LettaClient,
}

impl<'a> JobApi<'a> {
    /// Create a new job API instance.
    pub fn new(client: &'a LettaClient) -> Self {
        Self { client }
    }

    /// List all jobs.
    ///
    /// # Arguments
    ///
    /// * `params` - Optional parameters for filtering and pagination
    ///
    /// # Errors
    ///
    /// Returns a [crate::error::LettaError] if the request fails or if the response cannot be parsed.
    pub async fn list(&self, params: Option<ListJobsParams>) -> LettaResult<Vec<Job>> {
        self.client
            .get_with_query(endpoints::jobs::LIST, &params.unwrap_or_default())
            .await
    }

    /// List active jobs.
    ///
    /// # Arguments
    ///
    /// * `params` - Optional parameters for filtering and pagination
    ///
    /// # Errors
    ///
    /// Returns a [crate::error::LettaError] if the request fails or if the response cannot be parsed.
    pub async fn list_active(&self, params: Option<ListJobsParams>) -> LettaResult<Vec<Job>> {
        self.client
            .get_with_query(endpoints::jobs::LIST_ACTIVE, &params.unwrap_or_default())
            .await
    }

    /// Get a specific job.
    ///
    /// # Arguments
    ///
    /// * `job_id` - The ID of the job to retrieve
    ///
    /// # Errors
    ///
    /// Returns a [crate::error::LettaError] if the request fails or if the response cannot be parsed.
    pub async fn get(&self, job_id: &LettaId) -> LettaResult<Job> {
        self.client.get(&endpoints::jobs::get(job_id)).await
    }

    /// Delete/cancel a job.
    ///
    /// # Arguments
    ///
    /// * `job_id` - The ID of the job to delete/cancel
    ///
    /// # Errors
    ///
    /// Returns a [crate::error::LettaError] if the request fails.
    pub async fn delete(&self, job_id: &LettaId) -> LettaResult<Job> {
        self.client.delete(&endpoints::jobs::delete(job_id)).await
    }
}

/// Step API operations.
#[derive(Debug)]
pub struct StepApi<'a> {
    client: &'a LettaClient,
}

impl<'a> StepApi<'a> {
    /// Create a new step API instance.
    pub fn new(client: &'a LettaClient) -> Self {
        Self { client }
    }

    /// List all steps.
    ///
    /// # Arguments
    ///
    /// * `params` - Query parameters for filtering and pagination
    ///
    /// # Errors
    ///
    /// Returns a [crate::error::LettaError] if the request fails or if the response cannot be parsed.
    pub async fn list(
        &self,
        params: Option<crate::types::ListStepsParams>,
    ) -> LettaResult<Vec<Step>> {
        self.client
            .get_with_query(endpoints::steps::LIST, &params.unwrap_or_default())
            .await
    }

    /// Get a specific step.
    ///
    /// # Arguments
    ///
    /// * `step_id` - The ID of the step to retrieve
    ///
    /// # Errors
    ///
    /// Returns a [crate::error::LettaError] if the request fails or if the response cannot be parsed.
    pub async fn get(&self, step_id: &LettaId) -> LettaResult<Step> {
        self.client.get(&endpoints::steps::get(step_id)).await
    }

    /// Provide feedback on a step.
    ///
    /// # Arguments
    ///
    /// * `step_id` - The ID of the step to provide feedback for
    /// * `request` - The feedback request
    ///
    /// # Errors
    ///
    /// Returns a [crate::error::LettaError] if the request fails.
    pub async fn provide_feedback(
        &self,
        step_id: &LettaId,
        feedback: StepFeedback,
    ) -> LettaResult<String> {
        self.client
            .patch_no_body(&endpoints::steps::feedback_with_value(step_id, &feedback))
            .await
    }

    /// Modify feedback for a step.
    pub async fn modify_feedback(
        &self,
        step_id: &LettaId,
        request: ModifyFeedbackRequest,
    ) -> LettaResult<Step> {
        self.client
            .patch(&endpoints::steps::feedback(step_id), &request)
            .await
    }

    /// List messages for a step.
    pub async fn list_messages(&self, step_id: &LettaId) -> LettaResult<Vec<LettaMessageUnion>> {
        self.client.get(&endpoints::steps::messages(step_id)).await
    }

    /// Get metrics for a step.
    pub async fn get_metrics(&self, step_id: &LettaId) -> LettaResult<StepMetrics> {
        self.client.get(&endpoints::steps::metrics(step_id)).await
    }

    /// Get trace payload for a step.
    pub async fn get_trace(&self, step_id: &LettaId) -> LettaResult<Option<TelemetryTrace>> {
        self.client.get(&endpoints::steps::trace(step_id)).await
    }

    /// Step transaction patch endpoint.
    pub async fn update_transaction_id(
        &self,
        step_id: &LettaId,
        transaction_id: &str,
    ) -> LettaResult<Step> {
        self.client
            .patch_no_body(&endpoints::steps::transaction(step_id, transaction_id))
            .await
    }

    /// Get provider telemetry payload for a step.
    pub async fn get_provider_trace(&self, step_id: &LettaId) -> LettaResult<TelemetryTrace> {
        self.client.get(&endpoints::telemetry::get(step_id)).await
    }
}

/// Convenience methods for job and step operations.
impl LettaClient {
    /// Get the job API for this client.
    pub fn jobs(&self) -> JobApi<'_> {
        JobApi::new(self)
    }

    /// Get the step API for this client.
    pub fn steps(&self) -> StepApi<'_> {
        StepApi::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::ClientConfig;

    #[test]
    fn test_job_api_creation() {
        let config = ClientConfig::new("http://localhost:8283").unwrap();
        let client = LettaClient::new(config).unwrap();
        let _api = JobApi::new(&client);
    }

    #[test]
    fn test_step_api_creation() {
        let config = ClientConfig::new("http://localhost:8283").unwrap();
        let client = LettaClient::new(config).unwrap();
        let _api = StepApi::new(&client);
    }
}
