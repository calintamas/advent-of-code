//
//  Day07.swift
//  2023
//
//  Created by Calin Tamas on 07.12.2023.
//

import Foundation

let strengths: [Character] = ["2", "3", "4", "5", "6", "7", "8", "9", "T", "J", "Q", "K", "A"]

func strengthFor(char: Character) -> Int {
    strengths.firstIndex(of: char)!
}

enum Kind: String, Comparable, CaseIterable {
    static func < (lhs: Kind, rhs: Kind) -> Bool {
        lhs.index >= rhs.index
    }
    
    var index: Int {
        let allCases = Self.allCases
        return allCases.firstIndex { $0 == self }!
    }
    
    case five = "5"
    case four = "41"
    case fullHouse = "32"
    case three = "311"
    case two = "221"
    case one = "2111"
    case highCard = "11111"
}

struct Card: Comparable {
    static func < (lhs: Card, rhs: Card) -> Bool {
        if lhs.kind == rhs.kind {
            // compare each char
            for (idx, char) in lhs.hand.enumerated() {
                let s1 = strengthFor(char: char)
                let s2 = strengthFor(char: rhs.hand[idx]!)
                if (s1 == s2) {
                    continue
                }
                return s1 < s2
            }
        }
        return lhs.kind < rhs.kind
    }
    
    var bid: Int
    var kind: Kind
    var hand: String
    
    func valueFor(rank: Int) -> Int {
        bid * rank
    }
    
    init(bid: Int, hand: String) {
        self.bid = bid
        self.hand = hand
        
        var map = [Character: Int]()
        for char in hand {
            map.seen(char: char)
        }
        
        let rawKind = map
            .sortedByValue()
            .map({ String($0.value) }).joined()
        
        kind = Kind(rawValue: rawKind)!
    }
}


struct Day07: AdventDay {
    var cards: [Card] = []
    
    init(input: String) {
        let data = input.splitByLine()
        for line in data {
            let values = line.splitByWhiteSpace()
            let card = Card(bid: values.last!.toInt(), hand: values.first!)
            cards.append(card)
        }
    }
    
    mutating func p1() -> String {
        let sorted = cards.sorted(by: <)
        
        var sum = 0
        for (idx, card) in sorted.enumerated() {
            sum += card.valueFor(rank: idx + 1)
        }
        
        return "\(sum)"
    }
    
    mutating func p2() -> String {
        return ""
    }
}
