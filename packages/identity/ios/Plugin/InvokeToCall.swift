import Foundation
import Tauri

struct PaymentSummaryItem: Codable {
    let label: String
    let amount: Double
}

struct BillingDetailsCollectionConfiguration: Codable {
    let email: String
    let name: String
    let phone: String
    let address: String
}

class Args: Decodable {
    // InitializeOption
    let publishableKey: String?
    let stripeAccount: String?
    
    // CreatePaymentSheetOption
    let paymentIntentClientSecret: String?
    let setupIntentClientSecret: String?
    let billingDetailsCollectionConfiguration: BillingDetailsCollectionConfiguration?
    let customerEphemeralKeySecret: String?
    let customerId: String?
    let enableApplePay: Bool?
    let applePayMerchantId: String?
    let enableGooglePay: Bool?
    let GooglePayIsTesting: Bool?
    let countryCode: String?
    let merchantDisplayName: String?
    let returnURL: String?
    let style: String?
    let withZipCode: String?
    
    // CreateApplePayOption
//    let paymentIntentClientSecret: String?
    var paymentSummaryItems: [PaymentSummaryItem]?
    let merchantIdentifier: String?
//    let countryCode: String?
    let currency: String?
    let requiredShippingContactFields: [String]?
    let allowedCountries: String?
    let allowedCountriesErrorDescription: String?
}


typealias CAPPlugin = Plugin

extension StripeIdentityPlugin {
    func notifyListeners(_ event: String, data: JSObject) {
        trigger(event, data: data)
    }
}

public class bridgeMember {
    var call: CAPPluginCall? = nil;
    
    
    func saveCall(_ call: CAPPluginCall) -> Void {
        self.call = call;
    }
    
    func savedCall(withID: String) -> CAPPluginCall? {
        return call;
    }

    static let shared: bridgeMember = bridgeMember()
    init() {}
}

class CAPPluginCall {
    public let callbackId: String = "";
    
    var invoke: Invoke;
    
    init(_ invoke: Invoke) {
        self.invoke = invoke;
    }
    
    func getBool(_ key: String) -> Bool? {
        do {
            let args = try self.invoke.parseArgs(Args.self);
            return try getValueFromParsedArgs(parsedArgs: args, key: key) as? Bool
        } catch {
            return nil
        }
    }

    func getBool(_ key: String, _ defaultValue: Bool) -> Bool {
        do {
            let args = try self.invoke.parseArgs(Args.self);
            if let value = try getValueFromParsedArgs(parsedArgs: args, key: key) as! Bool? {
                return value
            } else {
                return defaultValue
            }
        } catch {
            return defaultValue
        }
    }
    
    func getString(_ key: String) -> String? {
        do {
            let args = try self.invoke.parseArgs(Args.self);
            return try getValueFromParsedArgs(parsedArgs: args, key: key) as! String?
        } catch {
            return nil
        }
    }

    func getString(_ key: String, _ defaultValue: String) -> String {
        do {
            let args = try self.invoke.parseArgs(Args.self);
            if let value = try getValueFromParsedArgs(parsedArgs: args, key: key) as! String? {
                return value
            } else {
                return defaultValue
            }
        } catch {
            return defaultValue
        }
    }
    

    func getInt(_ key: String) -> Int? {
        do {
            let args = try self.invoke.parseArgs(Args.self);
            return try getValueFromParsedArgs(parsedArgs: args, key: key) as! Int?
        } catch {
            return nil
        }
    }

    func getInt(_ key: String, _ defaultValue: Int) -> Int {
        do {
            let args = try self.invoke.parseArgs(Args.self);
            if let value = try getValueFromParsedArgs(parsedArgs: args, key: key) as! Int? {
                return value
            } else {
                return defaultValue
            }
        } catch {
            return defaultValue
        }
    }
    
    func getObject(_ key: String) -> JSObject? {
        do {
            let args = try self.invoke.parseArgs(Args.self);
            return try getValueFromParsedArgs(parsedArgs: args, key: key) as! JSObject?
        } catch {
            return nil
        }
    }

    func getObject(_ key: String, _ defaultValue: JSObject) -> JSObject {
        do {
            let args = try self.invoke.parseArgs(Args.self);
            if let value = try getValueFromParsedArgs(parsedArgs: args, key: key) as! JSObject? {
                return value
            } else {
                return defaultValue
            }
        } catch {
            return defaultValue
        }
    }
    
    func getArray(_ key: String) -> JSArray? {
        do {
            let args = try self.invoke.parseArgs(Args.self);
            return try getValueFromParsedArgs(parsedArgs: args, key: key) as! JSArray?
        } catch {
            return nil
        }
    }

    func getArray(_ key: String, _ defaultValue: JSArray) -> JSArray {
        do {
            let args = try self.invoke.parseArgs(Args.self);
            if let value = try getValueFromParsedArgs(parsedArgs: args, key: key) as! JSArray? {
                return value
            } else {
                return defaultValue
            }
        } catch {
            return defaultValue
        }
    }
    
    func getArray<T>(_ key: String, _ ofType: T.Type) -> [T]? {
        return getArray(key) as? [T]
    }
    
    public func resolve() {
        invoke.resolve()
    }
    
    func resolve(_ response: JSObject) {
        if (response.isEmpty) {
          resolve()
        }
        return invoke.resolve(response)
    }
    
    func reject(_ message: String) {
        return invoke.reject(message)
    }
    
    func unimplemented() {
        return invoke.unimplemented("not implemented")
    }

    func unimplemented(_ message: String) {
        return invoke.reject(message)
    }

    func unavailable() {
        return invoke.unavailable("not available")
    }

    func unavailable(_ message: String) {
        return invoke.reject(message)
    }
    
    private func getValueFromParsedArgs(parsedArgs: Args, key: String) throws -> Any {
        let mirror = Mirror(reflecting: parsedArgs)
        for child in mirror.children {
            if child.label == key {
                return child.value
            }
        }
        throw NSError(domain: "not found key", code: -1, userInfo: nil)
    }
}
