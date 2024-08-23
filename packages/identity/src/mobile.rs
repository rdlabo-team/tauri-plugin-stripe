use serde::de::DeserializeOwned;
use tauri::{
  plugin::{PluginApi, PluginHandle},
  AppHandle, Runtime,
};

use crate::models::*;

#[cfg(target_os = "android")]
const PLUGIN_IDENTIFIER: &str = "";

#[cfg(target_os = "ios")]
tauri::ios_plugin_binding!(init_plugin_stripe_identity);

// initializes the Kotlin or Swift plugin classes
pub fn init<R: Runtime, C: DeserializeOwned>(
  _app: &AppHandle<R>,
  api: PluginApi<R, C>,
) -> crate::Result<StripeIdentity<R>> {
  #[cfg(target_os = "android")]
  let handle = api.register_android_plugin(PLUGIN_IDENTIFIER, "ExamplePlugin")?;
  #[cfg(target_os = "ios")]
  let handle = api.register_ios_plugin(init_plugin_stripe_identity)?;
  Ok(StripeIdentity(handle))
}

/// Access to the stripe-identity APIs.
pub struct StripeIdentity<R: Runtime>(PluginHandle<R>);

impl<R: Runtime> StripeIdentity<R> {
  pub fn initialize(&self, payload: InitializeIdentityVerificationSheetOption) -> crate::Result<()> {
    self
      .0
      .run_mobile_plugin("initialize", payload)
      .map_err(Into::into)
  }

  pub fn createPaymentSheet(&self, payload: CreateIdentityVerificationSheetOption) -> crate::Result<()> {
    self
      .0
      .run_mobile_plugin("create", payload)
      .map_err(Into::into)
  }

  pub fn presentPaymentSheet(&self, payload: VoidOption) -> crate::Result<()> {
    self
      .0
      .run_mobile_plugin("present", payload)
      .map_err(Into::into)
  }
}
