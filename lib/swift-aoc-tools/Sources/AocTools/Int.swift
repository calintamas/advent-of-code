//
//  Int.swift
//  2023
//
//  Created by Calin Tamas on 04.12.2023.
//

import Foundation

public extension Int {
    func pow(exponent: Int) -> Int {
        var res = 1
        for _ in 0..<exponent {
            res *= self
        }
        return res
    }
    
    func lcm(with other: Int) -> Int {
        let first = self
        let second = other
        return abs(first * second) / gcd(first, second)
    }

    private func gcd(_ a: Int, _ b: Int) -> Int {
        var a = a
        var b = b

        while b != 0 {
            let temp = b
            b = a % b
            a = temp
        }

        return abs(a)
    }
}
