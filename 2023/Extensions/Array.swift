//
//  Array.swift
//  2023
//
//  Created by Calin Tamas on 03.12.2023.
//

import Foundation

extension [Int] {
    func sum() -> Int {
        return self.reduce(0, +)
    }
}

