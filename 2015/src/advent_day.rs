pub trait AdventDay {
    /// Parses the input for the day's challenge
    fn parse(&mut self, input: &str);

    /// Solves part 1
    fn p1(&self) -> String;

    /// Solves part 2
    fn p2(&self) -> String;

    /// Runs both parts and returns formatted results.
    fn run(&mut self, input: &str) {
        self.parse(input);
        println!("Part 1: {}", self.p1());
        println!("Part 2: {}", self.p2());
    }
}
