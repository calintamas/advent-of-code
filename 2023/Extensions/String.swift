//
//  String.swift
//  2023
//
//  Created by Calin Tamas on 03.12.2023.
//

import Foundation

extension String {
    func splitByLine() -> [String] {
        self.components(separatedBy: "\n").filter({ !$0.isEmpty })
    }
}
