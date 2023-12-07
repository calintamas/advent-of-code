//
//  Day07.swift
//  2023
//
//  Created by Calin Tamas on 07.12.2023.
//

import Foundation

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

struct Rules {
    let strengths: [Character]
    let withJoker: Bool
    
    func strengthFor(char: Character) -> Int {
        strengths.firstIndex(of: char)!
    }
}

struct Card: Comparable {
    static func == (lhs: Card, rhs: Card) -> Bool {
        lhs.kind == rhs.kind
    }
    
    static func < (lhs: Card, rhs: Card) -> Bool {
        if lhs.kind == rhs.kind {
            // compare each char
            for (idx, char) in lhs.hand.enumerated() {
                let s1 = lhs.rules.strengthFor(char: char)
                let s2 = rhs.rules.strengthFor(char: rhs.hand[idx]!)
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
    var rules: Rules

    
    func valueFor(rank: Int) -> Int {
        bid * rank
    }
    
    init(bid: Int, hand: String, rules: Rules) {
        self.bid = bid
        self.hand = hand
        self.rules = rules
        
        var map = [Character: Int]()
        
        // the hand we will evaluate
        let handValue = hand.filter { char in
            if rules.withJoker {
                return char != "J"
            }
            return true
        }
        
        for char in handValue {
            map.seen(char: char)
        }
        
        var sortedMap = map.sortedByValue()
        
        let sum = sortedMap.map({ $0.value }).sum()
        let jokerCount = hand.count - sum
        // if we play with Joker, "replace" most seen char with "J" chars
        if rules.withJoker {
            for _ in 0..<jokerCount {
                map.seen(char: sortedMap.first?.key ?? "J")
            }
        }
        
        // map was updated, sort again
        sortedMap = map.sortedByValue()
        
        let rawKind = sortedMap.map({ String($0.value) }).joined()
        kind = Kind(rawValue: rawKind)!
    }
}


struct Day07: AdventDay {
    var lines: [String] = []
    
    init(input: String) {
        lines = input.splitByLine()
    }
    
    func generateCardsWith(rules: Rules) -> [Card] {
        var cards: [Card] = []
        for line in lines {
            let values = line.splitByWhiteSpace()
            let card = Card(bid: values.last!.toInt(), hand: values.first!, rules: rules)
            cards.append(card)
        }
        return cards
    }
    
    func computeWinnings(_ cards: [Card]) -> Int {
        let sorted = cards.sorted(by: <)
        var sum = 0
        for (idx, card) in sorted.enumerated() {
            sum += card.valueFor(rank: idx + 1)
        }
        return sum
    }
    
    mutating func p1() -> String {
        let rules = Rules(strengths: ["2", "3", "4", "5", "6", "7", "8", "9", "T", "J", "Q", "K", "A"], withJoker: false)
        let cards = generateCardsWith(rules: rules)
  
        let sum = computeWinnings(cards)
        return "\(sum)"
    }
    
    mutating func p2() -> String {
        let rules = Rules(strengths: ["J", "2", "3", "4", "5", "6", "7", "8", "9", "T", "Q", "K", "A"], withJoker: true)
        let cards = generateCardsWith(rules: rules)
  
        let sum = computeWinnings(cards)
        return "\(sum)"
    }
}
