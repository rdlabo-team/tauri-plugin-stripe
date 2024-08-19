# This is development Repository. Still working on the release.

<p align="center"><br><img src="https://v2.tauri.app/_astro/header.DJC8YrJ3_Z2lir5I.webp" width="256" height="128" /></p>

<h3 align="center">Stripe</h3>
<p align="center">
  Tauri plugin for native Stripe.
</p>

<p align="center">
  <img src="https://img.shields.io/maintenance/yes/2024?style=flat-square" />
  <a href="https://www.npmjs.com/package/@capacitor-community/stripe"><img src="https://img.shields.io/npm/l/@capacitor-community/stripe?style=flat-square" /></a>
</p>

## packages

| package name                         | description | path                                                                                                   |
|--------------------------------------|-------------|--------------------------------------------------------------------------------------------------------|
| @rdlabo/tauri-plugin-stripe-payment  | Support for non-personal payments using Stripe | [/packages/payment](https://github.com/capacitor-community/stripe/tree/main/packages/payment#readme)   |
| @rdlabo/tauri-plugin-stripe-identity | [/packages/identity](https://github.com/capacitor-community/stripe/tree/main/packages/identity#readme) |
| @rdlabo/tauri-plugin-stripe-terminal | Support for in-person payments using Stripe  | [/packages/terminal](https://github.com/capacitor-community/stripe/tree/main/packages/terminal#readme) |


## Hint

### How to use Stripe Android currently package
To use the latest Stripe Android, you need to version these up. To use the latest features, follow these steps.

1. Open `android/variables.gradle` and change sdkVersion version, if need.
2. Add `stripeAndroidVersion`, `identityVersion` or `stripeterminalCoreVersion` and set required version. Release information is here: 
- https://github.com/stripe/stripe-android/releases
- https://github.com/stripe/stripe-terminal-android/releases

```diff
  ext {
-   minSdkVersion = 22
+   minSdkVersion = 26
    compileSdkVersion = 34
    targetSdkVersion = 33
    androidxActivityVersion = '1.7.0'
    androidxAppCompatVersion = '1.6.1'
    androidxCoordinatorLayoutVersion = '1.2.0'
    androidxCoreVersion = '1.10.0'
    androidxFragmentVersion = '1.5.6'
    coreSplashScreenVersion = '1.0.0'
    androidxWebkitVersion = '1.6.1'
    junitVersion = '4.13.2'
    androidxJunitVersion = '1.1.5'
    androidxEspressoCoreVersion = '3.5.1'
    cordovaAndroidVersion = '10.1.1'

    // If you use @capacitor-community/stripe:
+   stripeAndroidVersion = '20.39.+'

    // If you use @capacitor-community/stripe-identity:
+   identityVersion = '20.39.+'

    // If you use @capacitor-community/stripe-terminal:
+   stripeterminalCoreVersion = '3.5.0'
+   stripeterminalLocalmobileVersion = '3.5.0'
  }
```


## Maintainers

| Maintainer          | GitHub                              | Social                                |
| ------------------- | ----------------------------------- | ------------------------------------- |
| Masahiko Sakakibara | [rdlabo](https://github.com/rdlabo) | [@rdlabo](https://twitter.com/rdlabo) |


## Demo

- [Demo code is here](https://github.com/capacitor-community/stripe/tree/master/demo). Please check these code before ask at issues.
- Demo of Web is [hosting here](https://capacitor-community-stripe.netlify.app/).

### Screenshots

#### @capacitor-community/stripe

|              |                     Android                     |                     iOS                     |                     Web                     |
|:------------:|:-----------------------------------------------:|:-------------------------------------------:|:-------------------------------------------:|
| PaymentSheet | ![](demo/screenshots/payment-sheet-android.png) | ![](demo/screenshots/payment-sheet-ios.png) | ![](demo/screenshots/payment-sheet-web.png) |
| PaymentFlow  | ![](demo/screenshots/payment-flow-android.png)  | ![](demo/screenshots/payment-flow-ios.png)  | ![](demo/screenshots/payment-sheet-web.png) |
|   ApplePay   |                  Not supported                  |   ![](demo/screenshots/apple-pay-ios.png)   |                    beta.                    |
|  GooglePay   |  ![](demo/screenshots/google-pay-android.png)   |                Not supported                |  ![](demo/screenshots/google-pay-web.png)   |


#### @capacitor-community/stripe-identity

|              |                     Android                     |                     iOS                     |                  Web                   |
|:------------:|:-----------------------------------------------:|:-------------------------------------------:|:--------------------------------------:|
|   Identity   |   ![](demo/screenshots/identity-android.png)    |    ![](demo/screenshots/identity-ios.png)     | ![](demo/screenshots/identity-web.png) |
