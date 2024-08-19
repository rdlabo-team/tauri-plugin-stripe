import Foundation
import UIKit
import StripeCore
import Tauri


class StripePlugin: CAPPlugin {
    private let paymentSheetExecutor = PaymentSheetExecutor()
    private let paymentFlowExecutor = PaymentFlowExecutor()
    private let applePayExecutor = ApplePayExecutor()

    @objc func initialize(_ call: Invoke) {
        self.paymentSheetExecutor.plugin = self
        self.paymentFlowExecutor.plugin = self
        self.applePayExecutor.plugin = self

        let publishableKey = CAPPluginCall(call).getString("publishableKey") ?? ""

        if publishableKey == "" {
            call.reject("you must provide publishableKey")
            return
        }

        StripeAPI.defaultPublishableKey = publishableKey

        let stripeAccount = CAPPluginCall(call).getString("stripeAccount") ?? ""

        if stripeAccount != "" {
            STPAPIClient.shared.stripeAccount = stripeAccount
        }

        STPAPIClient.shared.appInfo = STPAppInfo(name: "@rdlabo/stripe-payment", partnerId: nil, version: nil, url: nil)

        call.resolve()
    }

    @objc func handleURLCallback(_ call: Invoke) {
        self.paymentSheetExecutor.plugin = self
        self.paymentFlowExecutor.plugin = self
        self.applePayExecutor.plugin = self

        let urlString = CAPPluginCall(call).getString("url") ?? ""

        if urlString == "" {
            call.reject("you must provide url returned from browser")
            return
        }

        let url = URL(string: urlString)!
        DispatchQueue.main.async {
            let stripeHandled = StripeAPI.handleURLCallback(with: url)
            if !stripeHandled {
                call.reject("This was not a Stripe url – handle the URL normally as you would")
                return
            }
            call.resolve()
        }

    }

    @objc func createPaymentSheet(_ call: Invoke) {
        self.paymentSheetExecutor.createPaymentSheet(CAPPluginCall(call))
    }

    @objc func presentPaymentSheet(_ call: Invoke) {
        self.paymentSheetExecutor.presentPaymentSheet(CAPPluginCall(call))
    }

    @objc func createPaymentFlow(_ call: Invoke) {
        self.paymentFlowExecutor.createPaymentFlow(CAPPluginCall(call))
    }

    @objc func presentPaymentFlow(_ call: Invoke) {
        self.paymentFlowExecutor.presentPaymentFlow(CAPPluginCall(call))
    }

    @objc func confirmPaymentFlow(_ call: Invoke) {
        self.paymentFlowExecutor.confirmPaymentFlow(CAPPluginCall(call))
    }

    @objc func isApplePayAvailable(_ call: Invoke) {
        self.applePayExecutor.isApplePayAvailable(CAPPluginCall(call))
    }

    @objc func createApplePay(_ call: Invoke) {
        self.applePayExecutor.createApplePay(CAPPluginCall(call))
    }

    @objc func presentApplePay(_ call: Invoke) {
        self.applePayExecutor.presentApplePay(CAPPluginCall(call))
    }

    @objc func isGooglePayAvailable(_ call: Invoke) {
        call.unavailable("Not implemented on iOS.")
    }

    @objc func createGooglePay(_ call: Invoke) {
        call.unavailable("Not implemented on iOS.")
    }

    @objc func presentGooglePay(_ call: Invoke) {
        call.unavailable("Not implemented on iOS.")
    }

    func getRootVC() -> UIViewController? {
        var window: UIWindow? = UIApplication.shared.delegate?.window ?? nil

        if window == nil {
            let scene: UIWindowScene? = UIApplication.shared.connectedScenes.first as? UIWindowScene
            window = scene?.windows.filter({$0.isKeyWindow}).first
            if window == nil {
                window = scene?.windows.first
            }
        }
        return window?.rootViewController
    }

    let bridge: bridgeMember? = bridgeMember()
}
