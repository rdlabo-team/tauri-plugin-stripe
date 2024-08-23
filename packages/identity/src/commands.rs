use tauri::{AppHandle, command, Runtime};

use crate::models::*;
use crate::Result;
use crate::StripeIdentityExt;

#[command]
pub(crate) async fn initialize<R: Runtime>(
    app: AppHandle<R>,
    payload: InitializeIdentityVerificationSheetOption,
) -> Result<()> {
    app.stripe_identity().initialize(payload)
}

#[command]
pub(crate) async fn create<R: Runtime>(
    app: AppHandle<R>,
    payload: CreateIdentityVerificationSheetOption,
) -> Result<()> {
    app.stripe_identity().create(payload)
}

#[command]
pub(crate) async fn present<R: Runtime>(
    app: AppHandle<R>,
    payload: VoidOption,
) -> Result<IdentityVerificationSheetResultInterface> {
    app.stripe_identity().present(payload)
}
