//
//  Day03.swift
//  2023
//
//  Created by Calin Tamas on 03.12.2023.
//

import Foundation
import AocTools

struct Point: Equatable {
    var x: Int
    var y: Int
    
    var isValid: Bool {
        return x >= 0 && y >= 0
    }
}

struct PartNumber {
    let value: Int
    let symbol: Character
    let symbolPoint: Point
}

struct Day03: AdventDay {
    let rows: [String]
    var parts: [PartNumber] = []
    
    init(input: String) {
        self.rows = input.splitByLine().map({ row in
            // there can be a number right at the end of the row
            // so we'll keep the special newline char there
            row.appending("\n")
        })
    }
    
    func findSymbolIn(row: String, rowIdx: Int, for points: [Point]) -> (Character, Point)? {
        for point in points {
            let char = row[point.y]
            if let char {
                if (!char.isNumber && char != "." && !char.isNewline) {
                    return (char, Point(x: rowIdx, y: point.y))
                }
            }
        }
        return nil
    }
    
    mutating func p1() -> String {
        for (row, line) in rows.enumerated() {
            let prevRow = rows[safe: row - 1]
            let nextRow = rows[safe: row + 1]
            
            var value = 0
            // touch-points around the value
            var points: [Point] = []
            
            for (col, char) in line.enumerated() {
                if (char.isNumber) {
                    if (value == 0) {
                        // starting a new value, add leading point
                        points.append(Point(x: row, y: col - 1))
                    }
                    // add every value point
                    points.append(Point(x: row, y: col))
                    value = value * 10 + char.wholeNumberValue!
                } else {
                    if (value == 0) {
                        continue
                    }
                    // value is complete, add trailing point
                    points.append(Point(x: row, y: col))
                    // keep only valid points
                    points = points.filter({ $0.isValid })
                    
                    var symbol: (value: Character, point: Point)? = nil
                    
                    // check current row (leading and trailing chars)
                    symbol = findSymbolIn(row: line, rowIdx: row, for: points)
                    
                    // check prev row
                    if symbol == nil, let prevRow {
                        symbol = findSymbolIn(row: prevRow, rowIdx: row - 1, for: points)
                    }
                    
                    // check next row
                    if symbol == nil, let nextRow {
                        symbol = findSymbolIn(row: nextRow, rowIdx: row + 1, for: points)
                    }
                    
                    if let symbol {
                        parts.append(PartNumber(value: value, symbol: symbol.value, symbolPoint: symbol.point))
                    }
                    
                    // reset current value
                    value = 0
                    points = []
                }
            }
        }
        
        let sum = parts.reduce(0) { (result, item) in
            return result + item.value
        }
        return "\(sum)"
    }
    
    // find gears: adjacent parts, connected by "*"
    mutating func p2() -> String {
        let gears = parts.filter { $0.symbol == "*" }
        var gearRatios: [Int] = []

        for (idx, gear) in gears.enumerated() {
            let rest = gears.suffix(from: idx + 1)
            let otherGear = rest.filter { item in
                item.symbolPoint == gear.symbolPoint
            }.first
            if let otherGear {
                gearRatios.append(gear.value * otherGear.value)
            }
        }
        let sum = gearRatios.sum()
        return "\(sum)"
    }
    
    
}
