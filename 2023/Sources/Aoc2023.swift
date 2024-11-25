import Foundation
import AocTools
import ArgumentParser

@main
struct Aoc2023: ParsableCommand {
    @Option(help: "Path to input file (.txt)")
    public var input: String
    
    @Option(help: "Advent of code day (Int)")
    public var day: Int
    
    public func run() throws {
        let url = URL(fileURLWithPath: input)
        guard let input = try? String(contentsOf: url) else {
            fatalError("Could not read input file at \(input)")
        }
        
        var res: AdventDay
        switch day {
        case 1:
            res = Day01(input: input)
            break
        case 2:
            res = Day02(input: input)
            break
        case 3:
            res = Day03(input: input)
            break
        case 4:
            res = Day04(input: input)
            break
        case 5:
            res = Day05(input: input)
            break
        case 6:
            res = Day06(input: input)
            break
        case 7:
            res = Day07(input: input)
            break
        case 8:
            res = Day08(input: input)
            break
        case 9:
            res = Day09(input: input)
            break
        case 10:
            res = Day10(input: input)
            break
        default:
            fatalError("Day not implemented")
        }
        
        print("p1 \(res.p1())")
        print("p2 \(res.p2())")
    }
}
