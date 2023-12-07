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
    
    func multiply() -> Int {
        return self.reduce(1, *)
    }
}

extension Array {
    subscript(safe index: Index) -> Element? {
        indices.contains(index) ? self[index] : nil
    }
}
