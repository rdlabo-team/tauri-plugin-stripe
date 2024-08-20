use tauri::{AppHandle, command, Runtime};

use crate::models::*;
use crate::Result;
use crate::StripePaymentExt;

#[command]
pub(crate) async fn initialize<R: Runtime>(
    app: AppHandle<R>,
    payload: InitializeOption,
) -> Result<()> {
    app.stripe_payment().initialize(payload)
}

#[command]
pub(crate) async fn createPaymentSheet<R: Runtime>(
    app: AppHandle<R>,
    payload: CreatePaymentSheetOption,
) -> Result<()> {
    app.stripe_payment().createPaymentSheet(payload)
}

#[command]
pub(crate) async fn presentPaymentSheet<R: Runtime>(
    app: AppHandle<R>,
    payload: (),
) -> Result<PaymentSheetResultInterface> {
    app.stripe_payment().presentPaymentSheet(payload)
}
