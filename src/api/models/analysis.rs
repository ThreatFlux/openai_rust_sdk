//! # Models API Analysis
//!
//! Advanced analysis, cost estimation, and recommendation functionality for the Models API.

use super::{client::ModelsApi, types::ModelStatistics};
use crate::error::Result;
use crate::models::models::{Model, ModelCapabilities, ModelRequirements};
use std::collections::HashMap;

impl ModelsApi {
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
