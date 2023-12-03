//
//  Day02.swift
//  2023
//
//  Created by Calin Tamas on 03.12.2023.
//

import Foundation
import RegexBuilder

typealias RGB = (red: Int, green: Int, blue: Int)

struct Game {
    var index: Int
    var isPossible = true
    
    var minValues: RGB = (red: 0, green: 0, blue: 0)
    
    func powerOf(values: RGB) -> Int {
        return values.red * values.green * values.blue
    }
    
    mutating func checkPick(values: RGB) {
        if (isPossible) {
            // update the value only if the game is still possible
            isPossible = values.red <= 12 && values.green <= 13 && values.blue <= 14
        }
        minValues = (
            max(minValues.red, values.red),
            max(minValues.green, values.green),
            max(minValues.blue, values.blue)
        )
    }
}

struct Day02: AdventDay {
    let lines: [String]
    
    init(input: String) {
        self.lines = input.splitByLine()
    }
    
    func extractContent(_ line: String) -> (index: Int, content:Substring) {
        let indexRef = Reference(Int.self)
        let contentRef = Reference(Substring.self)
        
        let gameRegex = Regex {
            "Game "
            Capture(as: indexRef) { OneOrMore(.digit) } transform: { Int($0)! } // index
            ": "
            Capture(as: contentRef) { OneOrMore(.any) } // sets content
        }
        if let result = line.firstMatch(of: gameRegex) {
            return (result.1, result.2)
        }
        return (-1, "")
    
    }
    
    func countCubes(_ pick: String) -> RGB {
        var red = 0
        var green = 0
        var blue = 0
        
        let pickRegex = Regex {
            Capture { OneOrMore(.digit) } transform: { Int($0)! } // count
            " "
            Capture { OneOrMore(.word) } // color
            Optionally { "," }
        }
        for result in pick.matches(of: pickRegex) {
            let count = result.1
            let color = result.2

            if color == "red" {
                red += count
            }
            if result.2 == "green" {
                green += count
            }
            if result.2 == "blue" {
                blue += count
            }
        }
        
        return (red, green, blue)
    }
    
    var powersOfMinValues: [Int] = []
    
    mutating func p1() -> String {
        var possibleGames: [Int] = []
        
        for line in lines {
            let (index, content) = extractContent(line)
            let picks = content.components(separatedBy: "; ")
            
            var game = Game(index: index)
            for pick in picks {
                let values = countCubes(pick)
                game.checkPick(values: values)
            }
            if (game.isPossible) {
                possibleGames.append(game.index)
            }
            powersOfMinValues.append(game.powerOf(values: game.minValues))
        }

        let sum = possibleGames.sum()
        return "\(sum)"
    }
    
    func p2() -> String {
        let sum = powersOfMinValues.sum()
        return "\(sum)"
    }
}
