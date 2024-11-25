//
//  Day06.swift
//  2023
//
//  Created by Calin Tamas on 06.12.2023.
//

import Foundation
import AocTools

struct Race {
    let time: Int
    let dist: Int
}

struct Boat {
    let holdMs: Int
    
    func distanceFor(time: Int) -> Int {
        let speed = holdMs
        let remainingTime = time - holdMs
        return speed * remainingTime
    }
}

struct Day06: AdventDay {
    var races: [Race] = []
    var actualRace: Race
    
    init(input: String) {
        let data = input.splitByLine()

        let times = data[0].components(separatedBy: ":").last!.splitByWhiteSpace().map({ $0.toInt() })
        let distances = data[1].components(separatedBy: ":").last!.splitByWhiteSpace().map({ $0.toInt() })
       
        for (idx,t) in times.enumerated() {
            self.races.append(Race(time: t, dist: distances[idx]))
        }
        
        let actualTime = data[0].components(separatedBy: ":").last!.replacingOccurrences(of: " ", with: "").toInt()
        let actualDist = data[1].components(separatedBy: ":").last!.replacingOccurrences(of: " ", with: "").toInt()
        actualRace = Race(time: actualTime, dist: actualDist)
    }
    
    func countWaysToWin(race: Race) -> Int {
        var count = 0
        for t in 0..<race.time {
            let d = Boat(holdMs: t).distanceFor(time: race.time)
            if (d > race.dist) {
                count += 1
            }
        }
        return count
    }
    
    mutating func p1() -> String {
        var values: [Int] = []
        
        for race in races {
            values.append(countWaysToWin(race: race))
        }

        let product = values.multiply()
        return "\(product)"
    }
    
    mutating func p2() -> String {
        let count = countWaysToWin(race: actualRace)
        return "\(count)"
    }
    
    
}
