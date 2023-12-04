//
//  test.swift
//  test
//
//  Created by Calin Tamas on 04.12.2023.
//

import XCTest

final class ArrayTests: XCTestCase {
    func testSum() {
        let values = [1, 2, 3, -1]
        XCTAssertEqual(values.sum(), 5)
    }
}
