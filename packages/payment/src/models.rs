use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InitializeOption {
  pub publishableKey: Option<String>,
  pub stripeAccount: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BillingDetailsCollectionConfiguration {
    pub email: Option<CollectionMode>,
    pub name: Option<CollectionMode>,
    pub phone: Option<CollectionMode>,
    pub address: Option<AddressCollectionMode>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum AddressCollectionMode {
    Automatic,
    Full,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum CollectionMode {
    Automatic,
    Always,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BasePaymentOption {
    pub billing_details_collection_configuration: Option<BillingDetailsCollectionConfiguration>,
    pub customer_ephemeral_key_secret: Option<String>,
    pub customer_id: Option<String>,
    pub enable_apple_pay: Option<bool>,
    pub apple_pay_merchant_id: Option<String>,
    pub enable_google_pay: Option<bool>,
    pub google_pay_is_testing: Option<bool>,
    pub country_code: Option<String>,
    pub merchant_display_name: Option<String>,
    pub return_url: Option<String>,
    pub style: Option<PaymentSheetStyle>,
    pub with_zip_code: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum PaymentSheetStyle {
    AlwaysLight,
    AlwaysDark,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatePaymentSheetOption {
    pub base_payment_option: BasePaymentOption,
    pub payment_intent_client_secret: Option<String>,
    pub setup_intent_client_secret: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatePaymentFlowOption {
    pub base_payment_option: BasePaymentOption,
    pub payment_intent_client_secret: Option<String>,
    pub setup_intent_client_secret: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateApplePayOption {
    pub payment_intent_client_secret: String,
    pub payment_summary_items: Vec<PaymentSummaryItem>,
    pub merchant_identifier: String,
    pub country_code: String,
    pub currency: String,
    pub required_shipping_contact_fields: Option<Vec<ShippingContactField>>,
    pub allowed_countries: Option<Vec<String>>,
    pub allowed_countries_error_description: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PaymentSummaryItem {
    pub label: String,
    pub amount: f64,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateGooglePayOption {
    pub payment_intent_client_secret: String,
    pub payment_summary_items: Option<Vec<PaymentSummaryItem>>,
    pub merchant_identifier: Option<String>,
    pub country_code: Option<String>,
    pub currency: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DidSelectShippingContact {
    pub contact: ShippingContact,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ShippingContact {
    pub given_name: Option<String>,
    pub family_name: Option<String>,
    pub middle_name: Option<String>,
    pub name_prefix: Option<String>,
    pub name_suffix: Option<String>,
    pub name_formatted: Option<String>,
    pub phone_number: Option<String>,
    pub nickname: Option<String>,
    pub street: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub postal_code: Option<String>,
    pub country: Option<String>,
    pub iso_country_code: Option<String>,
    pub sub_administrative_area: Option<String>,
    pub sub_locality: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ShippingContactField {
    PostalAddress,
    PhoneNumber,
    EmailAddress,
    Name,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum PaymentSheetEventsEnum {
    Loaded,
    FailedToLoad,
    Completed,
    Canceled,
    Failed,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum PaymentSheetResultInterface {
    Completed,
    Canceled,
    Failed,
}