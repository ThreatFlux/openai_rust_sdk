//! Category flags indicating which policies are violated

use crate::{De, Ser};

/// Category flags indicating which policies are violated
#[derive(Debug, Clone, Ser, De)]
pub struct ModerationCategories {
    /// Content that expresses, incites, or promotes hate based on race, gender, ethnicity, religion, nationality, sexual orientation, disability status, or caste
    pub hate: bool,

    /// Content that expresses, incites, or promotes harassing language towards any target
    #[serde(rename = "hate/threatening")]
    pub hate_threatening: bool,

    /// Content that promotes, encourages, or depicts acts of self-harm
    #[serde(rename = "self-harm")]
    pub self_harm: bool,

    /// Content where the speaker expresses that they are engaging or intend to engage in acts of self-harm
    #[serde(rename = "self-harm/intent")]
    pub self_harm_intent: bool,

    /// Content that encourages performing acts of self-harm
    #[serde(rename = "self-harm/instructions")]
    pub self_harm_instructions: bool,

    /// Content meant to arouse sexual excitement
    pub sexual: bool,

    /// Sexual content that includes an individual who is under 18 years old
    #[serde(rename = "sexual/minors")]
    pub sexual_minors: bool,

    /// Content that depicts death, violence, or physical injury
    pub violence: bool,

    /// Content that depicts death, violence, or physical injury in graphic detail
    #[serde(rename = "violence/graphic")]
    pub violence_graphic: bool,

    /// Content intended to harass, threaten, or bully an individual
    pub harassment: bool,

    /// Harassment content that also includes violence or serious harm towards any target
    #[serde(rename = "harassment/threatening")]
    pub harassment_threatening: bool,
}
