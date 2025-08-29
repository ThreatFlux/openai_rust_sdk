//! # Models API Operations
//!
//! High-level operations and filtering capabilities for the Models API.

use super::client::ModelsApi;
use crate::error::{OpenAIError, Result};
use crate::models::models::{CompletionType, Model, ModelFamily, ModelRequirements};
use std::collections::HashMap;

impl ModelsApi {
    /// List models filtered by family
    pub async fn list_models_by_family(&self, family: ModelFamily) -> Result<Vec<Model>> {
        let response = self.list_models().await?;
        Ok(response
            .filter_by_family(family)
            .into_iter()
            .cloned()
            .collect())
    }

    /// List models that support a specific completion type
    pub async fn list_models_by_completion_type(
        &self,
        completion_type: CompletionType,
    ) -> Result<Vec<Model>> {
        let response = self.list_models().await?;
        Ok(response
            .filter_by_completion_type(&completion_type)
            .into_iter()
            .cloned()
            .collect())
    }

    /// Get only available (non-deprecated) models
    pub async fn list_available_models(&self) -> Result<Vec<Model>> {
        let response = self.list_models().await?;
        Ok(response.available_models().into_iter().cloned().collect())
    }

    /// Get models grouped by family
    pub async fn group_models_by_family(&self) -> Result<HashMap<ModelFamily, Vec<Model>>> {
        let response = self.list_models().await?;
        let grouped = response.group_by_family();

        let mut result = HashMap::new();
        for (family, models) in grouped {
            result.insert(family, models.into_iter().cloned().collect());
        }

        Ok(result)
    }

    /// Get the latest model from each family
    pub async fn get_latest_models(&self) -> Result<HashMap<ModelFamily, Model>> {
        let response = self.list_models().await?;
        let latest = response.latest_models();

        let mut result = HashMap::new();
        for (family, model) in latest {
            result.insert(family, model.clone());
        }

        Ok(result)
    }

    /// Find models that meet specific requirements
    pub async fn find_suitable_models(
        &self,
        requirements: &ModelRequirements,
    ) -> Result<Vec<Model>> {
        let response = self.list_models().await?;
        Ok(response
            .find_suitable_models(requirements)
            .into_iter()
            .cloned()
            .collect())
    }

    /// Check if a model is available for use
    pub async fn is_model_available(&self, model_id: impl AsRef<str>) -> Result<bool> {
        match self.retrieve_model(model_id).await {
            Ok(model) => Ok(model.is_available()),
            Err(OpenAIError::ApiError { status: 404, .. }) => Ok(false),
            Err(e) => Err(e),
        }
    }
}
