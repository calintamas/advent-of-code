//
//  Day01.swift
//  2023
//
//  Created by Calin Tamas on 01.12.2023.
//

import Foundation

struct Day01: AdventDay {
    let lines: Array<String>
    
    init(input: String) {
        self.lines = input.splitByLine()
    }
    
    func calibrationValueFor(line: String) -> Int {
        let digits = String(line.filter { $0.isNumber })
        let value = digits.prefix(1) + digits.suffix(1)
        if let number = Int(value) {
            return number
        }
        return 0
    }
    
    func p1() -> String {
        let values = lines.map { calibrationValueFor(line: $0) }
        let sum = values.sum()
        return "\(sum)"
    }
    
    func p2() -> String {
        let digitStrings = [
            "one": 1, "two": 2, "three": 3, "four": 4, "five": 5, "six": 6, "seven": 7, "eight": 8, "nine": 9
        ]
        
        let values = lines.map { line in
            var updatedLine = "\(line)"
            digitStrings.forEach { (key, value) in
                updatedLine = updatedLine.replacingOccurrences(of: key, with: "\(key)\(value)\(key)")
            }
            return calibrationValueFor(line: updatedLine)
        }
        let sum = values.sum()
        return "\(sum)"
    }
}
