use serde::de::DeserializeOwned;
use tauri::{
  plugin::{PluginApi, PluginHandle},
  AppHandle, Runtime,
};

use crate::models::*;

#[cfg(target_os = "android")]
const PLUGIN_IDENTIFIER: &str = "";

#[cfg(target_os = "ios")]
tauri::ios_plugin_binding!(init_plugin_stripe-payment);

// initializes the Kotlin or Swift plugin classes
pub fn init<R: Runtime, C: DeserializeOwned>(
  _app: &AppHandle<R>,
  api: PluginApi<R, C>,
) -> crate::Result<StripePayment<R>> {
  #[cfg(target_os = "android")]
  let handle = api.register_android_plugin(PLUGIN_IDENTIFIER, "ExamplePlugin")?;
  #[cfg(target_os = "ios")]
  let handle = api.register_ios_plugin(init_plugin_stripe-payment)?;
  Ok(StripePayment(handle))
}

/// Access to the stripe-payment APIs.
pub struct StripePayment<R: Runtime>(PluginHandle<R>);

impl<R: Runtime> StripePayment<R> {
  pub fn initialize(&self, payload: InitializeOption) -> crate::Result<()> {
    self
      .0
      .run_mobile_plugin("initialize", payload)
      .map_err(Into::into)
  }
  
  pub fn createPaymentSheet(&self, payload: CreatePaymentSheetOption) -> crate::Result<()> {
    self
      .0
      .run_mobile_plugin("createPaymentSheet", payload)
      .map_err(Into::into)
  }

  pub fn presentPaymentSheet(&self, payload: ()) -> crate::Result<PaymentSheetResultInterface> {
    self
      .0
      .run_mobile_plugin("presentPaymentSheet", payload)
      .map_err(Into::into)
  }
}
