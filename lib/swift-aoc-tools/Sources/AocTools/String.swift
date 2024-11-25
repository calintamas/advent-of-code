//
//  String.swift
//  2023
//
//  Created by Calin Tamas on 03.12.2023.
//

import Foundation

public extension String {
    func splitByLine() -> [String] {
        self.components(separatedBy: "\n").filter({ !$0.isEmpty })
    }
    
    func splitByWhiteSpace() -> [String] {
        self.components(separatedBy: " ").filter({ !$0.isEmpty })
    }
    
    func trim() -> String {
        self.trimmingCharacters(in: .whitespacesAndNewlines)
    }
    
    func toInt() -> Int {
        Int(self)!
    }
    
    func extractIntValues() -> [Int] {
        self.splitByWhiteSpace().map({ $0.toInt() })
    }
    
    subscript(index: Int) -> Character? {
        guard index >= 0, index < count else {
            return nil // Return nil for out-of-bounds access
        }
        return self[self.index(self.startIndex, offsetBy: index)]
    }
}
