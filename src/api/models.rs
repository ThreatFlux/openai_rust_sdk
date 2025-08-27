//! # Models API
//!
//! This module provides access to OpenAI's Models API for listing available models
//! and retrieving detailed information about specific models and their capabilities.

use crate::api::base::HttpClient;
use crate::api::common::ApiClientConstructors;
use crate::error::{OpenAIError, Result};
use crate::http_get;
use crate::models::models::{
    CompletionType, ListModelsResponse, Model, ModelCapabilities, ModelFamily, ModelRequirements,
};
use std::collections::HashMap;

/// Models API client for accessing `OpenAI` model information
pub struct ModelsApi {
    /// HTTP client for making API requests
    http_client: HttpClient,
}

impl ApiClientConstructors for ModelsApi {
    fn from_http_client(http_client: HttpClient) -> Self {
        Self { http_client }
    }
}

impl ModelsApi {
    /// Create a new client with custom base URL
    pub fn with_base_url<S: Into<String>>(api_key: S, base_url: S) -> Result<Self> {
        Ok(Self {
            http_client: HttpClient::new_with_base_url(api_key, base_url)?,
        })
    }

    /// Get the API key (for testing purposes)
    #[must_use]
    pub fn api_key(&self) -> &str {
        self.http_client.api_key()
    }

    // Generate HTTP client methods using macro
    http_get!(list_models, "/v1/models", ListModelsResponse);
    http_get!(retrieve_model, "/v1/models/{}", model_id: impl AsRef<str>, Model);

    /// List models filtered by family
    pub async fn list_models_by_family(&self, family: ModelFamily) -> Result<Vec<Model>> {
        let response = self.list_models().await?;
        Ok(response
            .filter_by_family(&family)
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

    /// Get the best model for a specific use case
    pub async fn get_recommended_model(
        &self,
        requirements: &ModelRequirements,
    ) -> Result<Option<Model>> {
        let suitable_models = self.find_suitable_models(requirements).await?;

        if suitable_models.is_empty() {
            return Ok(None);
        }

        // Prioritize models based on quality and availability
        let best_model = suitable_models
            .into_iter()
            .filter(|m| !m.is_deprecated())
            .max_by(|a, b| {
                let a_caps = a.capabilities();
                let b_caps = b.capabilities();

                // First priority: tier
                match a_caps.tier.partial_cmp(&b_caps.tier) {
                    Some(std::cmp::Ordering::Equal) => {
                        // Second priority: max tokens (if available)
                        match (a_caps.max_tokens, b_caps.max_tokens) {
                            (Some(a_tokens), Some(b_tokens)) => a_tokens.cmp(&b_tokens),
                            (Some(_), None) => std::cmp::Ordering::Greater,
                            (None, Some(_)) => std::cmp::Ordering::Less,
                            (None, None) => a.created.cmp(&b.created),
                        }
                    }
                    Some(ordering) => ordering,
                    None => a.created.cmp(&b.created),
                }
            });

        Ok(best_model)
    }

    /// Check if a model supports specific capabilities
    pub async fn check_model_capabilities(
        &self,
        model_id: impl AsRef<str>,
    ) -> Result<ModelCapabilities> {
        let model = self.retrieve_model(model_id).await?;
        Ok(model.capabilities())
    }

    /// Get cost estimation for using a model
    pub async fn estimate_model_cost(
        &self,
        model_id: impl AsRef<str>,
        input_tokens_per_month: u64,
        output_tokens_per_month: u64,
    ) -> Result<Option<f64>> {
        let capabilities = self.check_model_capabilities(model_id).await?;
        Ok(capabilities.estimate_monthly_cost(input_tokens_per_month, output_tokens_per_month))
    }

    /// Compare costs between multiple models
    pub async fn compare_model_costs(
        &self,
        model_ids: &[impl AsRef<str>],
        input_tokens_per_month: u64,
        output_tokens_per_month: u64,
    ) -> Result<HashMap<String, Option<f64>>> {
        let mut costs = HashMap::new();

        for model_id in model_ids {
            let model_id_str = model_id.as_ref().to_string();
            let cost = self
                .estimate_model_cost(model_id, input_tokens_per_month, output_tokens_per_month)
                .await?;
            costs.insert(model_id_str, cost);
        }

        Ok(costs)
    }

    /// Get models sorted by cost (cheapest first)
    pub async fn get_models_by_cost(
        &self,
        requirements: &ModelRequirements,
        input_tokens_per_month: u64,
        output_tokens_per_month: u64,
    ) -> Result<Vec<(Model, Option<f64>)>> {
        let suitable_models = self.find_suitable_models(requirements).await?;

        let mut model_costs = Vec::new();
        for model in suitable_models {
            let cost = model
                .capabilities()
                .estimate_monthly_cost(input_tokens_per_month, output_tokens_per_month);
            model_costs.push((model, cost));
        }

        // Sort by cost (cheapest first), putting models without cost data at the end
        model_costs.sort_by(|a, b| match (a.1, b.1) {
            (Some(cost_a), Some(cost_b)) => cost_a
                .partial_cmp(&cost_b)
                .unwrap_or(std::cmp::Ordering::Equal),
            (Some(_), None) => std::cmp::Ordering::Less,
            (None, Some(_)) => std::cmp::Ordering::Greater,
            (None, None) => std::cmp::Ordering::Equal,
        });

        Ok(model_costs)
    }

    /// Check if a model is available for use
    pub async fn is_model_available(&self, model_id: impl AsRef<str>) -> Result<bool> {
        match self.retrieve_model(model_id).await {
            Ok(model) => Ok(model.is_available()),
            Err(OpenAIError::ApiError { status: 404, .. }) => Ok(false),
            Err(e) => Err(e),
        }
    }

    /// Get model statistics and summary
    pub async fn get_model_statistics(&self) -> Result<ModelStatistics> {
        let response = self.list_models().await?;
        let models = &response.data;

        let total_models = models.len();
        let available_models = models.iter().filter(|m| m.is_available()).count();
        let deprecated_models = models.iter().filter(|m| m.is_deprecated()).count();

        let family_counts = response.group_by_family();
        let family_distribution: HashMap<String, usize> = family_counts
            .iter()
            .map(|(family, models)| (format!("{family:?}"), models.len()))
            .collect();

        let tier_distribution = {
            let mut tier_counts = HashMap::new();
            for model in models {
                let tier = model.capabilities().tier;
                *tier_counts.entry(format!("{tier:?}")).or_insert(0) += 1;
            }
            tier_counts
        };

        let completion_type_distribution = {
            let mut type_counts = HashMap::new();
            for model in models {
                let capabilities = model.capabilities();
                for completion_type in &capabilities.completion_types {
                    *type_counts
                        .entry(format!("{completion_type:?}"))
                        .or_insert(0) += 1;
                }
            }
            type_counts
        };

        Ok(ModelStatistics {
            total_models,
            available_models,
            deprecated_models,
            family_distribution,
            tier_distribution,
            completion_type_distribution,
        })
    }
}

/// Statistics about available models
#[derive(Debug, Clone)]
pub struct ModelStatistics {
    /// Total number of models
    pub total_models: usize,
    /// Number of available (non-deprecated) models
    pub available_models: usize,
    /// Number of deprecated models
    pub deprecated_models: usize,
    /// Distribution of models by family
    pub family_distribution: HashMap<String, usize>,
    /// Distribution of models by tier
    pub tier_distribution: HashMap<String, usize>,
    /// Distribution of models by completion type support
    pub completion_type_distribution: HashMap<String, usize>,
}

impl ModelStatistics {
    /// Get the deprecation rate as a percentage
    #[must_use]
    pub fn deprecation_rate(&self) -> f64 {
        if self.total_models == 0 {
            0.0
        } else {
            (self.deprecated_models as f64 / self.total_models as f64) * 100.0
        }
    }

    /// Get the availability rate as a percentage
    #[must_use]
    pub fn availability_rate(&self) -> f64 {
        if self.total_models == 0 {
            0.0
        } else {
            (self.available_models as f64 / self.total_models as f64) * 100.0
        }
    }

    /// Get the most common model family
    #[must_use]
    pub fn most_common_family(&self) -> Option<(&String, &usize)> {
        self.family_distribution
            .iter()
            .max_by_key(|(_, count)| *count)
    }

    /// Get the most common model tier
    #[must_use]
    pub fn most_common_tier(&self) -> Option<(&String, &usize)> {
        self.tier_distribution
            .iter()
            .max_by_key(|(_, count)| *count)
    }
}

/// Utility functions for working with models
pub struct ModelUtils;

impl ModelUtils {
    /// Extract the base model name from a versioned model ID
    #[must_use]
    pub fn extract_base_model_name(model_id: &str) -> String {
        // Remove common version suffixes
        let suffixes = [
            "-0613",
            "-0314",
            "-0301",
            "-16k",
            "-32k",
            "-instruct",
            "-chat",
        ];

        let mut base_name = model_id.to_string();
        for suffix in &suffixes {
            if let Some(pos) = base_name.rfind(suffix) {
                base_name = base_name[..pos].to_string();
            }
        }

        base_name
    }

    /// Check if two models are from the same family
    #[must_use]
    pub fn are_same_family(model_id_1: &str, model_id_2: &str) -> bool {
        ModelCapabilities::classify_family(model_id_1)
            == ModelCapabilities::classify_family(model_id_2)
    }

    /// Get the newest model from a list by creation timestamp
    #[must_use]
    pub fn get_newest_model(models: &[Model]) -> Option<&Model> {
        models.iter().max_by_key(|model| model.created)
    }

    /// Get the oldest model from a list by creation timestamp
    #[must_use]
    pub fn get_oldest_model(models: &[Model]) -> Option<&Model> {
        models.iter().min_by_key(|model| model.created)
    }

    /// Sort models by preference (non-deprecated, premium tier, higher context)
    pub fn sort_by_preference(models: &mut [Model]) {
        models.sort_by(|a, b| {
            let a_caps = a.capabilities();
            let b_caps = b.capabilities();

            // First: non-deprecated models
            match (a.is_deprecated(), b.is_deprecated()) {
                (false, true) => return std::cmp::Ordering::Less,
                (true, false) => return std::cmp::Ordering::Greater,
                _ => {}
            }

            // Second: tier
            match a_caps.tier.partial_cmp(&b_caps.tier) {
                Some(std::cmp::Ordering::Equal) => {}
                Some(ordering) => return ordering.reverse(), // Higher tier first
                None => {}
            }

            // Third: max tokens
            match (a_caps.max_tokens, b_caps.max_tokens) {
                (Some(a_tokens), Some(b_tokens)) => {
                    match a_tokens.cmp(&b_tokens) {
                        std::cmp::Ordering::Equal => {}
                        ordering => return ordering.reverse(), // Higher context first
                    }
                }
                (Some(_), None) => return std::cmp::Ordering::Less,
                (None, Some(_)) => return std::cmp::Ordering::Greater,
                _ => {}
            }

            // Finally: creation date (newer first)
            b.created.cmp(&a.created)
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::models::Model;

    #[tokio::test]
    async fn test_models_api_creation() {
        let api = ModelsApi::new("test-key").unwrap();
        assert_eq!(api.api_key(), "test-key");
    }

    #[tokio::test]
    async fn test_models_api_empty_key() {
        let result = ModelsApi::new("");
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_models_api_with_base_url() {
        let api = ModelsApi::with_base_url("test-key", "https://custom.api.com/v1").unwrap();
        assert_eq!(api.api_key(), "test-key");
    }

    #[test]
    fn test_model_statistics() {
        let stats = ModelStatistics {
            total_models: 10,
            available_models: 8,
            deprecated_models: 2,
            family_distribution: HashMap::new(),
            tier_distribution: HashMap::new(),
            completion_type_distribution: HashMap::new(),
        };

        assert_eq!(stats.deprecation_rate(), 20.0);
        assert_eq!(stats.availability_rate(), 80.0);
    }

    #[test]
    fn test_model_utils_extract_base_name() {
        assert_eq!(
            ModelUtils::extract_base_model_name("gpt-3.5-turbo-0613"),
            "gpt-3.5-turbo"
        );
        assert_eq!(ModelUtils::extract_base_model_name("gpt-4-32k"), "gpt-4");
        assert_eq!(
            ModelUtils::extract_base_model_name("text-davinci-003"),
            "text-davinci-003"
        );
    }

    #[test]
    fn test_model_utils_same_family() {
        // GPT-4 and GPT-4 Turbo are actually different families in our classification
        assert!(!ModelUtils::are_same_family("gpt-4", "gpt-4-turbo"));
        assert!(ModelUtils::are_same_family(
            "gpt-3.5-turbo",
            "gpt-3.5-turbo-16k"
        ));
        assert!(!ModelUtils::are_same_family("gpt-4", "dall-e-3"));
        // These should be same family
        assert!(ModelUtils::are_same_family("gpt-4", "gpt-4-32k"));
        assert!(ModelUtils::are_same_family(
            "gpt-4-turbo",
            "gpt-4-turbo-preview"
        ));
    }

    #[test]
    fn test_model_utils_newest_oldest() {
        let models = vec![
            Model {
                id: "old-model".to_string(),
                object: "model".to_string(),
                created: 1000,
                owned_by: "openai".to_string(),
                root: None,
                parent: None,
                permission: None,
            },
            Model {
                id: "new-model".to_string(),
                object: "model".to_string(),
                created: 2000,
                owned_by: "openai".to_string(),
                root: None,
                parent: None,
                permission: None,
            },
        ];

        let newest = ModelUtils::get_newest_model(&models);
        let oldest = ModelUtils::get_oldest_model(&models);

        assert_eq!(newest.unwrap().id, "new-model");
        assert_eq!(oldest.unwrap().id, "old-model");
    }

    #[test]
    fn test_model_utils_sort_by_preference() {
        let mut models = vec![
            Model {
                id: "text-davinci-003".to_string(), // deprecated
                object: "model".to_string(),
                created: 1000,
                owned_by: "openai".to_string(),
                root: None,
                parent: None,
                permission: None,
            },
            Model {
                id: "gpt-4".to_string(), // premium, not deprecated
                object: "model".to_string(),
                created: 2000,
                owned_by: "openai".to_string(),
                root: None,
                parent: None,
                permission: None,
            },
        ];

        ModelUtils::sort_by_preference(&mut models);
        assert_eq!(models[0].id, "gpt-4");
        assert_eq!(models[1].id, "text-davinci-003");
    }

    #[test]
    fn test_model_requirements_creation() {
        let req = ModelRequirements::chat();
        assert_eq!(req.completion_types, vec![CompletionType::Chat]);
        assert!(!req.requires_function_calling);

        let req = ModelRequirements::function_calling();
        assert!(req.requires_function_calling);

        let req = ModelRequirements::vision();
        assert!(req.requires_vision);

        let req = ModelRequirements::high_context(100_000);
        assert_eq!(req.min_max_tokens, Some(100_000));
    }

    #[test]
    fn test_list_models_response_methods() {
        let models = vec![
            Model {
                id: "gpt-4".to_string(),
                object: "model".to_string(),
                created: 1000,
                owned_by: "openai".to_string(),
                root: None,
                parent: None,
                permission: None,
            },
            Model {
                id: "dall-e-3".to_string(),
                object: "model".to_string(),
                created: 2000,
                owned_by: "openai".to_string(),
                root: None,
                parent: None,
                permission: None,
            },
            Model {
                id: "text-davinci-003".to_string(), // deprecated
                object: "model".to_string(),
                created: 500,
                owned_by: "openai".to_string(),
                root: None,
                parent: None,
                permission: None,
            },
        ];

        let response = ListModelsResponse {
            object: "list".to_string(),
            data: models,
        };

        // Test family filtering
        let gpt4_models = response.filter_by_family(&ModelFamily::GPT4);
        assert_eq!(gpt4_models.len(), 1);
        assert_eq!(gpt4_models[0].id, "gpt-4");

        // Test completion type filtering
        let chat_models = response.filter_by_completion_type(&CompletionType::Chat);
        assert_eq!(chat_models.len(), 1);
        assert_eq!(chat_models[0].id, "gpt-4");

        // Test available models
        let available = response.available_models();
        assert_eq!(available.len(), 2); // gpt-4 and dall-e-3

        // Test grouping by family
        let grouped = response.group_by_family();
        assert!(grouped.contains_key(&ModelFamily::GPT4));
        assert!(grouped.contains_key(&ModelFamily::DALLE));

        // Test latest models
        let latest = response.latest_models();
        assert!(latest.contains_key(&ModelFamily::GPT4));
        assert_eq!(latest.get(&ModelFamily::GPT4).unwrap().id, "gpt-4");

        // Test finding suitable models
        let requirements = ModelRequirements::chat();
        let suitable = response.find_suitable_models(&requirements);
        assert_eq!(suitable.len(), 1);
        assert_eq!(suitable[0].id, "gpt-4");
    }

    #[test]
    fn test_model_capabilities_cost_estimation() {
        let caps = ModelCapabilities::from_model_id("gpt-4");

        // Test with both input and output costs
        let cost = caps.estimate_monthly_cost(1_000_000, 500_000);
        assert!(cost.is_some());

        // Test with model that has no cost data
        let caps = ModelCapabilities::from_model_id("dall-e-3");
        let cost = caps.estimate_monthly_cost(1_000_000, 500_000);
        assert!(cost.is_none());
    }
}
