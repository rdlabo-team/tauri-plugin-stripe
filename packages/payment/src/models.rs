use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VoidOption {
}

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

// #[derive(Debug, Deserialize, Serialize)]
// #[serde(rename_all = "camelCase")]
// pub struct BasePaymentOption {
// 	  pub billingDetailsCollectionConfiguration: Option<BillingDetailsCollectionConfiguration>,
// 	  pub customerEphemeralKeySecret: Option<String>,
// 	  pub customerId: Option<String>,
// 	  pub enableApplePay: Option<bool>,
// 	  pub applePayMerchantId: Option<String>,
// 	  pub enableGooglePay: Option<bool>,
// 	  pub googlePayIsTesting: Option<bool>,
// 	  pub countryCode: Option<String>,
// 	  pub merchantDisplayName: Option<String>,
// 	  pub returnUrl: Option<String>,
// 	  pub style: Option<PaymentSheetStyle>,
// 	  pub withZipCode: Option<bool>,
// }

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum PaymentSheetStyle {
    AlwaysLight,
    AlwaysDark,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatePaymentSheetOption {
  pub paymentIntentClientSecret: Option<String>,
  pub setupIntentClientSecret: Option<String>,

	pub billingDetailsCollectionConfiguration: Option<BillingDetailsCollectionConfiguration>,
	pub customerEphemeralKeySecret: Option<String>,
	pub customerId: Option<String>,
	pub enableApplePay: Option<bool>,
	pub applePayMerchantId: Option<String>,
	pub enableGooglePay: Option<bool>,
	pub googlePayIsTesting: Option<bool>,
	pub countryCode: Option<String>,
	pub merchantDisplayName: Option<String>,
	pub returnUrl: Option<String>,
	pub style: Option<PaymentSheetStyle>,
	pub withZipCode: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatePaymentFlowOption {
	pub paymentIntentClientSecret: Option<String>,
	pub setupIntentClientSecret: Option<String>,

	pub billingDetailsCollectionConfiguration: Option<BillingDetailsCollectionConfiguration>,
	pub customerEphemeralKeySecret: Option<String>,
	pub customerId: Option<String>,
	pub enableApplePay: Option<bool>,
	pub applePayMerchantId: Option<String>,
	pub enableGooglePay: Option<bool>,
	pub googlePayIsTesting: Option<bool>,
	pub countryCode: Option<String>,
	pub merchantDisplayName: Option<String>,
	pub returnUrl: Option<String>,
	pub style: Option<PaymentSheetStyle>,
	pub withZipCode: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateApplePayOption {
	pub paymentIntentClientSecret: String,
	pub paymentSummaryItems: Vec<PaymentSummaryItem>,
	pub merchantIdentifier: String,
	pub countryCode: String,
	pub currency: String,
	pub requiredShippingContactFields: Option<Vec<ShippingContactField>>,
	pub allowedCountries: Option<Vec<String>>,
	pub allowedCountriesErrorDescription: Option<String>,
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
		pub paymentIntentClientSecret: String,
		pub paymentSummaryItems: Option<Vec<PaymentSummaryItem>>,
		pub merchantIdentifier: Option<String>,
		pub countryCode: Option<String>,
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
		pub givenName: Option<String>,
		pub familyName: Option<String>,
		pub middleName: Option<String>,
		pub namePrefix: Option<String>,
		pub nameSuffix: Option<String>,
		pub nameFormatted: Option<String>,
		pub phoneNumber: Option<String>,
		pub nickname: Option<String>,
		pub street: Option<String>,
		pub city: Option<String>,
		pub state: Option<String>,
		pub postalCode: Option<String>,
		pub country: Option<String>,
		pub isoCountryCode: Option<String>,
		pub subAdministrativeArea: Option<String>,
		pub subLocality: Option<String>,
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
