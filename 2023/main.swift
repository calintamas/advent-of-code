//
//  main.swift
//  2023
//
//  Created by Calin Tamas on 01.12.2023.
//

import Foundation

protocol AdventDay {    
    init(input: String)

    mutating func p1() -> String
    mutating func p2() -> String
}

func main() {
    // Access command line argument passed as "-path some/file/path"
    guard let path = UserDefaults.standard.string(forKey: "path") else {
        fatalError("-path argument was not provided")
    }
    let url = URL(fileURLWithPath: path)
    
    guard let input = try? String(contentsOf: url) else {
        fatalError("could not read input file at \(path)")
    }
    
    var day = Day09(input: input)
    let res1 = day.p1()
    print("p1 \(res1)")
    let res2 = day.p2()
    print("p2 \(res2)")
}

main()
