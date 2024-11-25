//
//  Day05.swift
//  2023
//
//  Created by Calin Tamas on 05.12.2023.
//

import Foundation
import AocTools

func lineToArray(line: String) -> [Int] {
    line.splitByWhiteSpace().map({ $0.toInt() })
}

struct Lookup {
    var src: (Int, Int)
    var dest: (Int, Int)
    var offset: Int
}

struct Mapping {
    var lookups: [Lookup] = []
    
    init(lines: [String]) {
        for line in lines {
            let values = lineToArray(line: line)
            
            let d = values[0]
            let s = values[1]
            let r = values[2]
            
            lookups.append(Lookup(src: (s, s+r-1), dest: (d, d+r-1), offset: d-s))
        }
    }
}

struct Day05: AdventDay {
    var seeds: [Int]
    var maps: [Mapping]
    
    init(input: String) {
        let data = input.components(separatedBy: "\n\n").map({ $0.components(separatedBy: ":").last!.trim() })
        
        seeds = lineToArray(line: data.first!)
        maps = data.dropFirst().map({ $0.splitByLine() }).map { lines in
            Mapping(lines: lines)
        }
    }
    
    mutating func p1() -> String {
        var min = Int.max
        for seed in seeds {
            var value = seed
            
            for map in maps {
                for lookup in map.lookups {
                    if value >= lookup.src.0 && value <= lookup.src.1 {
                        value = value + lookup.offset
                        break
                    }
                }
            }
            if (value < min) {
                min = value
            }
        }
        return "\(min)"
    }
    
    mutating func p2() -> String {
        ""
    }
}
