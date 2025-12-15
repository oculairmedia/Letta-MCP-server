//! Job and batch processing API endpoints.

use crate::client::LettaClient;
use crate::error::LettaResult;
use crate::types::{Job, JobStatus, LettaId, Step, StepFeedback};

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
    /// * `status` - Optional status filter
    /// * `limit` - Maximum number of jobs to return
    /// * `source_id` - Optional source ID to filter jobs
    ///
    /// # Errors
    ///
    ///
    /// Returns a [crate::error::LettaError] if the request fails or if the response cannot be parsed.
    pub async fn list(
        &self,
        status: Option<JobStatus>,
        limit: Option<i32>,
        source_id: Option<&LettaId>,
    ) -> LettaResult<Vec<Job>> {
        let mut params = Vec::new();
        if let Some(s) = status {
            params.push(("status", serde_json::to_string(&s)?));
        }
        if let Some(l) = limit {
            params.push(("limit", l.to_string()));
        }
        if let Some(sid) = source_id {
            params.push(("source_id", sid.to_string()));
        }

        if params.is_empty() {
            self.client.get("v1/jobs").await
        } else {
            self.client.get_with_query("v1/jobs", &params).await
        }
    }

    /// List active jobs.
    ///
    /// # Arguments
    ///
    /// * `status` - Optional status filter
    /// * `limit` - Maximum number of jobs to return
    ///
    /// # Errors
    ///
    /// Returns a [crate::error::LettaError] if the request fails or if the response cannot be parsed.
    pub async fn list_active(
        &self,
        status: Option<JobStatus>,
        limit: Option<i32>,
    ) -> LettaResult<Vec<Job>> {
        let mut params = Vec::new();
        if let Some(s) = status {
            params.push(("status", serde_json::to_string(&s)?));
        }
        if let Some(l) = limit {
            params.push(("limit", l.to_string()));
        }

        if params.is_empty() {
            self.client.get("v1/jobs/active").await
        } else {
            self.client.get_with_query("v1/jobs/active", &params).await
        }
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
        self.client.get(&format!("v1/jobs/{}", job_id)).await
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
    pub async fn delete(&self, job_id: &LettaId) -> LettaResult<String> {
        self.client.delete(&format!("v1/jobs/{}", job_id)).await
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
            .get_with_query("v1/steps/", &params.unwrap_or_default())
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
        self.client.get(&format!("v1/steps/{}", step_id)).await
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
            .patch_no_body(&format!(
                "v1/steps/{}/feedback?feedback={}",
                step_id, feedback
            ))
            .await
    }
}

/// Convenience methods for job and step operations.
impl LettaClient {
    /// Get the job API for this client.
    pub fn jobs(&self) -> JobApi {
        JobApi::new(self)
    }

    /// Get the step API for this client.
    pub fn steps(&self) -> StepApi {
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
