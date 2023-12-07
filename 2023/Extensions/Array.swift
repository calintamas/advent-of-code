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

extension [Character: Int] {
    mutating func seen(char: Character) {
        if let val = self[char] {
            self[char] = val + 1
        } else {
            self[char] = 1
        }
    }
    
    func sortedByValue() -> [Self.Element] {
        self.sorted(by: { prev, curr in
            prev.value > curr.value
        })
    }
}
