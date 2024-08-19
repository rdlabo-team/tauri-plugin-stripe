// swift-tools-version:5.3
// The swift-tools-version declares the minimum version of Swift required to build this package.

import PackageDescription

let package = Package(
    name: "tauri-plugin-stripe-payment",
    platforms: [
        .iOS(.v13),
    ],
    products: [
        // Products define the executables and libraries a package produces, and make them visible to other packages.
        .library(
            name: "tauri-plugin-stripe-payment",
            type: .static,
            targets: ["tauri-plugin-stripe-payment"]),
    ],
    dependencies: [
        .package(name: "Tauri", path: "../.tauri/tauri-api"),
        .package(url: "https://github.com/stripe/stripe-ios-spm", .upToNextMajor(from: "23.0.0"))
    ],
    targets: [
        // Targets are the basic building blocks of a package. A target can define a module or a test suite.
        // Targets can depend on other targets in this package, and on products in packages this package depends on.
        .target(
            name: "tauri-plugin-stripe-payment",
            dependencies: [
                .byName(name: "Tauri"),
                .product(name: "StripePaymentSheet", package: "stripe-ios-spm"),
                .product(name: "StripeApplePay", package: "stripe-ios-spm")
            ],
            path: "Plugin")
    ]
)
