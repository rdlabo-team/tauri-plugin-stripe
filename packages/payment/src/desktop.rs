use serde::de::DeserializeOwned;
use tauri::{plugin::PluginApi, AppHandle, Runtime};

use crate::models::*;

pub fn init<R: Runtime, C: DeserializeOwned>(
  app: &AppHandle<R>,
  _api: PluginApi<R, C>,
) -> crate::Result<StripePayment<R>> {
  Ok(StripePayment(app.clone()))
}

/// Access to the stripe-payment APIs.
pub struct StripePayment<R: Runtime>(AppHandle<R>);

impl<R: Runtime> StripePayment<R> {
  pub fn ping(&self, payload: PingRequest) -> crate::Result<PingResponse> {
    Ok(PingResponse {
      value: payload.value,
    })
  }
}
