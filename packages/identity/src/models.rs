use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VoidOption {
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InitializeIdentityVerificationSheetOption {
  pub publishableKey: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateIdentityVerificationSheetOption {
  pub verificationId: Option<String>,
  pub ephemeralKeySecret: Option<String>,
  pub clientSecret: Option<String>,
}
