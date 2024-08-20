#![cfg(mobile)]

use tauri::{
  plugin::{Builder, TauriPlugin},
  Manager, Runtime,
};

pub use models::*;

#[cfg(mobile)]
mod mobile;

mod commands;
mod error;
mod models;

pub use error::{Error, Result};

#[cfg(desktop)]

#[cfg(mobile)]
use mobile::StripePayment;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the stripe-payment APIs.
pub trait StripePaymentExt<R: Runtime> {
  fn stripe_payment(&self) -> &StripePayment<R>;
}

impl<R: Runtime, T: Manager<R>> crate::StripePaymentExt<R> for T {
  fn stripe_payment(&self) -> &StripePayment<R> {
    self.state::<StripePayment<R>>().inner()
  }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
  Builder::new("stripe-payment")
    .invoke_handler(tauri::generate_handler![commands::initialize])
    .setup(|app, api| {
      #[cfg(mobile)]
      let stripe_payment = mobile::init(app, api)?;
      app.manage(stripe_payment);
      Ok(())
    })
    .build()
}
