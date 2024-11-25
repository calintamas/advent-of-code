//
//  Day10.swift
//  2023
//
//  Created by Calin Tamas on 10.12.2023.
//

import Foundation
import AocTools

enum Direction {
    case north
    case south
    case west
    case east
    
    func opposite() -> Self {
        if self == .east {
            return .west
        }
        if self == .south {
            return .north
        }
        if self == .west {
            return .east
        }
        return .south
    }
}

struct Pipe {
    var coords: (row: Int, col: Int)
    var key: Character
    
    var connections: [Direction] {
        if (key == "|") {
            return [.north, .south]
        }
        if (key == "-") {
            return [.east, .west]
        }
        if (key == "L") {
            return [.north, .east]
        }
        if (key == "J") {
            return [.north, .west]
        }
        if (key == "7") {
            return [.south, .west]
        }
        if (key == "F") {
            return [.south, .east]
        }
        if (key == "S") {
            return [.north, .east, .south, .west]
        }
        return []
    }
    
    func goThrough(with map: Map, prevDir: Direction?) -> (Pipe?, direction: Direction?) {
        let row = self.coords.row
        let col = self.coords.col
        let directions = self.connections.filter { $0 != prevDir?.opposite()}
        
        for direction in directions {
            switch direction {
            case .east:
                if let key = map[safe: row]?[safe: col+1] {
                    let newPipe = Pipe(coords: (row, col+1), key: key)
                    if newPipe.connections.contains(.west) {
                        return (newPipe, direction)
                    }
                }
            case .south:
                if let key = map[safe: row+1]?[safe: col] {
                    let newPipe = Pipe(coords: (row+1, col), key: key)
                    if newPipe.connections.contains(.north) {
                        return (newPipe, direction)
                    }
                }
            case .west:
                if let key = map[safe: row]?[safe: col-1] {
                    let newPipe = Pipe(coords: (row, col-1), key: key)
                    if newPipe.connections.contains(.east) {
                        return (newPipe, direction)
                    }
                }
            case .north:
                if let key = map[safe: row-1]?[safe: col] {
                    let newPipe = Pipe(coords: (row-1, col), key: key)
                    if newPipe.connections.contains(.south) {
                        return (newPipe, direction)
                    }
                }
            }
        }
        return (nil, nil)
    }
}

extension Pipe: CustomStringConvertible {
    var description: String {
        "\(key) (\(coords.row+1),\(coords.col+1))"
    }
}

typealias Map = [[Character]]

struct Day10: AdventDay {
    var map: Map = []
    
    init(input: String) {
        // build map
        let lines = input.splitByLine()
        for (row, line) in lines.enumerated() {
            map.append(Array(repeating: " ", count: line.count))
            for (col, char) in line.enumerated() {
                map[row][col] = char
            }
        }
    }
    
    func findStart() -> Pipe? {
        for (row, _) in map.enumerated() {
            for (col, char) in map[row].enumerated() {
                if (char == "S") {
                    return Pipe(coords: (row, col), key: char)
                }
            }
        }
        return nil
    }
    
    mutating func p1() -> String {
        let start = findStart()!
        var pipe = start
        var dir: Direction? = nil
        var len = 0

        repeat {
            let (newPipe, newDir) = pipe.goThrough(with: map, prevDir: dir)
            pipe = newPipe!
            dir = newDir!
            if (pipe.key != "S") {
                len += 1
            }
        } while pipe.key != "S"
        
        let maxDist = Int(round(Double(len)/2))
        
        return "\(maxDist)"
    }
    
    mutating func p2() -> String {
        return ""
    }
}
