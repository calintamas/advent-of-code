//
//  Day09.swift
//  2023
//
//  Created by Calin Tamas on 09.12.2023.
//

import Foundation

struct Day09: AdventDay {
    var data: [[Int]] = []
    
    init(input: String) {
        let lines = input.splitByLine()
        for line in lines {
            data.append(line.extractIntValues())
        }
    }
    
    func finiteDiff(values: [Int]) -> [Int] {
        var diffs: [Int] = []
        for (idx, value) in values.enumerated() {
            if let nextValue = values[safe: idx + 1] {
                diffs.append(nextValue - value)
            }
        }
        return diffs
    }
    
    func computeFiniteDiffs(values: [Int], diffs: [[Int]] = []) -> [[Int]] {
        if (values.allSatisfy({ $0 == 0 }) ) {
            return diffs
        }
        let diff = finiteDiff(values: values)
        return computeFiniteDiffs(values: diff, diffs: diffs + [diff])
    }
    
    func nextItemFor(values: [[Int]], idx: Int) -> Int {
        if idx < 0 {
            return values.first!.last!
        }
        
        let current = values[idx]
        let next = values[safe: idx + 1] ?? [0]
        
        var updated = values.copy()
        updated[idx] = [current.last! + next.last!]
                
        return nextItemFor(values: updated, idx: idx - 1)
    }
    
    mutating func p1() -> String {
        var nextItems: [Int] = []
        
        for values in data {
            let all = [values] + computeFiniteDiffs(values: values)
            let nextItem = nextItemFor(values: all, idx: all.count - 1)
            nextItems.append(nextItem)
        }
        
        let sum = nextItems.sum()
        return "\(sum)"
    }
    
    mutating func p2() -> String {
        return ""
    }
}
