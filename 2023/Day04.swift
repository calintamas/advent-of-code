//
//  Day04.swift
//  2023
//
//  Created by Calin Tamas on 04.12.2023.
//

import Foundation

struct Scratchcard {
    var winning: [Int]
    var actual: [Int]
    
    func score() -> Int {
        let intersection = Set(winning).intersection(Set(actual))
        return intersection.count
    }
}

func extractValues(data: String?) -> [Int] {
    guard let data = data else {
        return []
    }
    return data.components(separatedBy: " ").filter({ !$0.isEmpty }).map({ $0.toInt() })
}

struct Day04: AdventDay {
    var scratchcards: [Scratchcard] = []
    
    init(input: String) {
        let rows = input.splitByLine()
        
        for row in rows {
            let data = row.components(separatedBy: ":").last!.components(separatedBy: "|")
            
            let winning = extractValues(data: data.first)
            let actual = extractValues(data: data.last)
            
            self.scratchcards.append(Scratchcard(winning: winning, actual: actual))
        }
    }
    
    mutating func p1() -> String {
        var points = 0
        for card in scratchcards {
            let exponent = card.score() - 1
            if exponent >= 0 {
                points += 2.pow(exponent: exponent)
            }
        }
        return "\(points)"
    }
    
    mutating func p2() -> String {
        var tracker = Array(repeating: 1, count: scratchcards.count)
 
        for (idx, card) in scratchcards.enumerated() {
            let count = card.score()
            
            for offset in 0..<count {
                tracker[idx + offset + 1] += tracker[idx]
            }
        }
        
        let sum = tracker.sum()
        return "\(sum)"
    }
}
