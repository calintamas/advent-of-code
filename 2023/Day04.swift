//
//  Day04.swift
//  2023
//
//  Created by Calin Tamas on 04.12.2023.
//

import Foundation

typealias Card = (id: Int, winning: [Int], actual: [Int])

struct Day04: AdventDay {
    let rows: [String]
    var values: [Card] = []
    
    init(input: String) {
        self.rows = input.splitByLine()
        
        for (idx, row) in rows.enumerated() {
            let data = row.components(separatedBy: ":").last
            if let data = data?.components(separatedBy: "|") {
                let winningValues = Day04.extractValues(data: data.first)
                let actualValues = Day04.extractValues(data: data.last)
                self.values.append((idx + 1, winningValues, actualValues))
                
            }
        }
    }
    
    static func extractValues(data: String?) -> [Int] {
        guard let data = data else {
            return []
        }
        return data.components(separatedBy: " ").filter({ !$0.isEmpty }).map({ $0.toInt() })
    }
    
    mutating func p1() -> String {
        var points = 0
        for value in values {
            let (_, winning, actual) = value
            let intersection = Set(winning).intersection(Set(actual))
            
            let power = intersection.count - 1
            var newPoints = 0
            if power >= 0 {
                newPoints = 2.pow(exponent: intersection.count - 1)
                points += newPoints
            }
        }
        return "\(points)"
    }
    
    mutating func p2() -> String {
        var copiesDict: Dictionary<Int, Int> = [:]
        
        var idx = 0
        repeat {
            let value = values[idx]
            
            let (id, winning, actual) = value
            let intersection = Set(winning).intersection(Set(actual))
            let count = intersection.count
            
            var copiedCards: [Card] = []
            
            for offset in 0..<count {
                let copy = values[id + offset]
                copiesDict[copy.id] = (copiesDict[copy.id] ?? 0) + 1
                copiedCards.append(copy)
            }
            
            if let copies = copiesDict[id] {
                // check how many copies we have for current card
                for _ in 0..<copies {
                    // and copy again each already copied card, that many times
                    copiedCards.forEach { card in
                        copiesDict[card.id] = (copiesDict[card.id] ?? 0) + 1
                    }
                }
            }
            
            idx += 1
        } while idx < values.count
        
        let count = copiesDict.values.reduce(0, +) + values.count
        return "\(count)"
    }
    
    
}
