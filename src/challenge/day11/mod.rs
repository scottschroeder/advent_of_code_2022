use anyhow::Result;

mod parse;

const ROUNDS_PT1: usize = 20;
const ROUNDS_PT2: usize = 10_000;
const MOST_ACTIVE: usize = 2;

mod monkey {
    #[derive(Debug, Clone, Copy)]
    pub struct Item(pub i64);

    impl Item {
        pub fn op_mod(self, r: i64) -> Item {
            Item(self.0 % r)
        }
    }

    #[derive(Debug, Clone, Copy)]
    pub struct MonkeyId(pub usize);

    #[derive(Debug, Clone, Copy)]
    pub struct Operation {
        pub code: OpCode,
        pub arg: Arg,
    }

    impl Operation {
        fn op(&self, lhs: i64) -> i64 {
            let rhs = match self.arg {
                Arg::Const(x) => x,
                Arg::Old => lhs,
            };

            match self.code {
                OpCode::Add => lhs + rhs,
                OpCode::Mul => lhs * rhs,
            }
        }
    }

    #[derive(Debug, Clone, Copy)]
    pub enum OpCode {
        Add,
        Mul,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Arg {
        Const(i64),
        Old,
    }

    #[derive(Debug)]
    pub struct Monkey {
        pub id: MonkeyId,
        pub operation: Operation,
        pub test: i64,
        pub true_monkey: MonkeyId,
        pub false_monkey: MonkeyId,
    }

    impl Monkey {
        pub fn handle_item(&self, item: Item, calm: i64) -> (Item, MonkeyId) {
            let inital = item.0;
            let inspected = self.operation.op(inital);
            let bored = inspected / calm;
            let target = if bored % self.test == 0 {
                self.true_monkey
            } else {
                self.false_monkey
            };
            (Item(bored), target)
        }
    }

    #[derive(Debug, Default)]
    pub struct Barrel {
        pub monkeys: Vec<Vec<Item>>,
    }

    impl Barrel {
        pub fn pop_monkey(&mut self, monkey: MonkeyId) -> Vec<Item> {
            let mut swp = Vec::new();
            if let Some(prev) = self.monkeys.get_mut(monkey.0) {
                std::mem::swap(&mut swp, prev)
            }
            swp
        }
        pub fn push(&mut self, monkey: MonkeyId, item: Item) {
            self.monkeys
                .get_mut(monkey.0)
                .expect("unknown monkey")
                .push(item)
        }
    }
}

pub fn part1(input: &str) -> Result<String> {
    let x = monkey_tossing_rounds(input, ROUNDS_PT1, 3)?;
    Ok(format!("{:?}", x))
}

pub fn part2(input: &str) -> Result<String> {
    let x = monkey_tossing_rounds(input, ROUNDS_PT2, 1)?;
    Ok(format!("{:?}", x))
}

fn monkey_tossing_rounds(input: &str, rounds: usize, calm_factor: i64) -> Result<i64> {
    let (mut barrel, monkeys) = parse::parse(input)?;
    log::debug!("{:#?}", monkeys);
    log::debug!("{:?}", barrel);

    let all_monkey_div = monkeys.iter().map(|m| m.test).product::<i64>() * calm_factor;
    log::debug!("div: {:?}", all_monkey_div);
    let mut inspections = vec![0; monkeys.len()];
    for _ in 0..rounds {
        for monkey in &monkeys {
            for item in barrel.pop_monkey(monkey.id) {
                inspections[monkey.id.0] += 1;
                let (new_item, target) = monkey.handle_item(item, calm_factor);

                barrel.push(target, new_item.op_mod(all_monkey_div));
            }
        }
    }
    log::debug!("inspections: {:?}", inspections);
    inspections.sort();
    Ok(inspections.iter().rev().take(MOST_ACTIVE).product::<i64>())
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../../../input/day11");
    const EX: &str = include_str!("../../../input/day11_ex");

    #[test]
    fn verify_p1() {
        assert_eq!(part1(INPUT).unwrap().as_str(), "98280")
    }
    #[test]
    fn verify_p2() {
        assert_eq!(part2(INPUT).unwrap().as_str(), "17673687232")
    }

    #[test]
    fn p1_ex() {
        assert_eq!(part1(EX).unwrap().as_str(), "10605")
    }
    #[test]
    fn p2_ex() {
        assert_eq!(part2(EX).unwrap().as_str(), "2713310158")
    }
}
