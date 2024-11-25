// swift-tools-version: 5.10
// The swift-tools-version declares the minimum version of Swift required to build this package.

import PackageDescription

let package = Package(
    name: "Aoc2023",
    platforms: [
        .macOS(.v13),
    ],
    dependencies: [
        .package(path: "../lib/swift-aoc-tools"),
        .package(url: "https://github.com/apple/swift-argument-parser", exact: "1.5.0")
    ],
    targets: [
        // Targets are the basic building blocks of a package, defining a module or a test suite.
        // Targets can depend on other targets in this package and products from dependencies.
        .executableTarget(
            name: "Aoc2023",
            dependencies: [
                .product(name: "AocTools", package: "swift-aoc-tools"),
                .product(name: "ArgumentParser", package: "swift-argument-parser"),
            ],
            path: "Sources"
        ),
    ]
)
