import XCTest
@testable import AocTools

final class ArrayTests: XCTestCase {
    func testSum() {
        let values = [1, 2, 3, -1]
        XCTAssertEqual(values.sum(), 5)
    }
}
