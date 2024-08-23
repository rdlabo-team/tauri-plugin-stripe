import Foundation
import StripeIdentity
import PassKit
import Tauri

class StripeIdentityPlugin: CAPPlugin {
    private let implementation = StripeIdentity()

    @objc func initialize(_ call: Invoke) {
        self.implementation.plugin = self
        STPAPIClient.shared.appInfo = STPAppInfo(name: "@rdlabo/tauri-plugin-stripe-identity", partnerId: nil, version: nil, url: nil)
        self.implementation.initialize(CAPPluginCall(call))
    }

    @objc func create(_ call: Invoke) {
        self.implementation.create(CAPPluginCall(call))
    }

    @objc func present(_ call: Invoke) {
        self.implementation.present(CAPPluginCall(call))
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
