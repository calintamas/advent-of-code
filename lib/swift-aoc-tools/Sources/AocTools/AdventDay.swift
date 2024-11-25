//
//  File.swift
//  SwiftToolbox
//
//  Created by Calin Tamas on 09.11.2024.
//

import Foundation

public protocol AdventDay {
    init(input: String)

    mutating func p1() -> String
    mutating func p2() -> String
}
