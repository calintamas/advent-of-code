//
//  main.swift
//  2023
//
//  Created by Calin Tamas on 01.12.2023.
//

import Foundation

protocol AdventDay {
    var input: String { get }
    
    init(input: String)

    func p1() -> String
    func p2() -> String
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
    
    print(input)
}

main()
