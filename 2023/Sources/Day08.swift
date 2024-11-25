//
//  Day08.swift
//  2023
//
//  Created by Calin Tamas on 08.12.2023.
//

import Foundation
import RegexBuilder
import AocTools

struct Node {
    var key: String
    var left: String
    var right: String
}

struct Day08: AdventDay {
    var map = [String: Node]()
    let instructions: [Character]
    
    init(input: String) {
        let data = input.splitByLine()
        
        instructions = data.first!.map { Character(String($0)) }
    
        for value in data.dropFirst() {
            let nodeRegex = Regex {
                Capture { OneOrMore(.word) } // key
                " = "
                "("
                Capture { OneOrMore(.word) } // left
                ", "
                Capture { OneOrMore(.word) } // right
                ")"
            }
            let res = value.firstMatch(of: nodeRegex)!
            let key = String(res.1)
            let left = String(res.2)
            let right = String(res.3)
                        
            map[key] = Node(key: key, left: left, right: right)
        }
    }
    
    mutating func p1() -> String {
        let count = countStepsFrom(start: map["AAA"]!) { $0.key == "ZZZ"}
        
        return "\(count)"
    }
    
    func countStepsFrom(start: Node, predicate: (_ node: Node) -> Bool) -> Int {
        var node = start
                
        var idx = 0
        var stepCount = 0
        
        repeat {
            stepCount += 1
            
            let k = instructions[safe: idx]!
    
            if (k == "R") {
                node = map[node.right]!
            }
            if (k == "L") {
                node = map[node.left]!
            }

            if idx == instructions.count - 1 {
                idx = 0
            } else {
                idx += 1
            }
        } while !predicate(node)
        
        return stepCount
    }
    
    mutating func p2() -> String {
        var nodes: [Node] = []
        
        for node in map {
            if node.key.hasSuffix("A") {
                nodes.append(node.value)
            }
        }
        
        var counts: [Int] = []
        for node in nodes {
            let count = countStepsFrom(start: node) { $0.key.hasSuffix("Z") }
            counts.append(count)
        }
    
        return "\(counts.lcm())"
    }
}
