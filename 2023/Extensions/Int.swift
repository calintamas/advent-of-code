//
//  Int.swift
//  2023
//
//  Created by Calin Tamas on 04.12.2023.
//

import Foundation

extension Int {
    func pow(exponent: Int) -> Int {
        var res = 1
        for _ in 0..<exponent {
            res *= self
        }
        return res
    }
}
