#![cfg(mobile)]

use tauri::{
  plugin::{Builder, TauriPlugin},
  Manager, Runtime,
};

pub use models::*;

mod mobile;

mod commands;
mod error;
mod models;

pub use error::{Error, Result};

use mobile::StripeIdentity;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the stripe-identity APIs.
pub trait StripeIdentityExt<R: Runtime> {
  fn stripe_identity(&self) -> &StripeIdentity<R>;
}

impl<R: Runtime, T: Manager<R>> crate::StripeIdentityExt<R> for T {
  fn stripe_identity(&self) -> &StripeIdentity<R> {
    self.state::<StripeIdentity<R>>().inner()
  }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
  Builder::new("stripe-identity")
    .invoke_handler(tauri::generate_handler![commands::initialize, commands::create, commands::present])
    .setup(|app, api| {
      let stripe_identity = mobile::init(app, api)?;
      app.manage(stripe_identity);
      Ok(())
    })
    .build()
}
