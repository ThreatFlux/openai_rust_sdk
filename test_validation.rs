use openai_rust_sdk::api::*;

fn main() {
    println!("Testing API key validation...\n");
    
    let apis = vec![
        ("AssistantsApi", assistants::AssistantsApi::new("").is_err()),
        ("AudioApi", audio::AudioApi::new("").is_err()),
        ("BatchApi", batch::BatchApi::new("").is_err()),
        ("EmbeddingsApi", embeddings::EmbeddingsApi::new("").is_err()),
        ("FilesApi", files::FilesApi::new("").is_err()),
        ("FineTuningApi", fine_tuning::FineTuningApi::new("").is_err()),
        ("ImagesApi", images::ImagesApi::new("").is_err()),
        ("ModelsApi", models::ModelsApi::new("").is_err()),
        ("ModerationsApi", moderations::ModerationsApi::new("").is_err()),
        ("ResponsesApi", responses::ResponsesApi::new("").is_err()),
        ("RunsApi", runs::RunsApi::new("").is_err()),
        ("ThreadsApi", threads::ThreadsApi::new("").is_err()),
        ("VectorStoresApi", vector_stores::VectorStoresApi::new("").is_err()),
    ];
    
    for (name, rejects_empty) in apis {
        println!("{}: {}", name, if rejects_empty { "✅ Rejects empty key" } else { "❌ Accepts empty key" });
    }
}