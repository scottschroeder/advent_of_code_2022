use crate::challenge::day11::monkey::Arg;

use super::monkey::Barrel;
use super::monkey::Item;
use super::monkey::Monkey;
use super::monkey::MonkeyId;
use super::monkey::OpCode;
use super::monkey::Operation;
use anyhow::Context;

pub fn parse(input: &str) -> anyhow::Result<(Barrel, Vec<Monkey>)> {
    let mut line_reader = input.lines();
    let mut monkeys = Vec::new();
    let mut barrel = Barrel::default();
    while let Some((items, m)) = parse_monkey(&mut line_reader).context("failed to parse monkey")? {
        if m.id.0 != monkeys.len() {
            anyhow::bail!(
                "monkeys in input are out of order: input={} expected={}",
                m.id.0,
                monkeys.len()
            )
        }
        monkeys.push(m);
        barrel.monkeys.push(items)
    }

    Ok((barrel, monkeys))
}

fn parse_monkey<'a>(
    line_reader: &mut impl Iterator<Item = &'a str>,
) -> anyhow::Result<Option<(Vec<Item>, Monkey)>> {
    let id = if let Ok(line) = get_line(line_reader) {
        parse_monkey_id(line).context("monkey id")?
    } else {
        // End of input, no more monkeys
        return Ok(None);
    };

    let items = get_line(line_reader)
        .and_then(parse_starting_items)
        .context("starting items")?;

    let operation = get_line(line_reader)
        .and_then(parse_operation)
        .context("operation")?;

    let test = get_line(line_reader)
        .and_then(parse_test_condition)
        .context("test condition")?;

    let true_monkey = get_line(line_reader)
        .and_then(|s| parse_throw(s, true))
        .context("true throw target")?;

    let false_monkey = get_line(line_reader)
        .and_then(|s| parse_throw(s, false))
        .context("false throw target")?;

    if let Ok(last_line) = get_line(line_reader) {
        if !last_line.is_empty() {
            anyhow::bail!("no blank line between monkeys")
        }
    }

    Ok(Some((
        items,
        Monkey {
            id,
            operation,
            test,
            true_monkey,
            false_monkey,
        },
    )))
}

fn parse_monkey_id(input: &str) -> anyhow::Result<MonkeyId> {
    let id_str = input
        .strip_prefix("Monkey ")
        .ok_or_else(|| anyhow::anyhow!("monkey not found in: {:?}", input))?
        .strip_suffix(':')
        .ok_or_else(|| anyhow::anyhow!("monkey id did not end with `:`: {:?}", input))?;
    let id = aoc::parse::parse_from_str(id_str, "monkey id")?;
    Ok(MonkeyId(id))
}

fn parse_starting_items(input: &str) -> anyhow::Result<Vec<Item>> {
    let item_list = input
        .strip_prefix("Starting items: ")
        .ok_or_else(|| anyhow::anyhow!("starting items not found in: {:?}", input))?;

    item_list
        .split(", ")
        .map(|s| aoc::parse::parse_from_str(s, "item").map(Item))
        .collect()
}

fn parse_operation(input: &str) -> anyhow::Result<Operation> {
    let op_info = input
        .strip_prefix("Operation: new = old ")
        .ok_or_else(|| anyhow::anyhow!("operation not found in: {:?}", input))?;

    let mut split = op_info.split_whitespace();
    let op_char = aoc::parse::expect_word(&mut split, "operation symbol")?;

    let op_arg_str = aoc::parse::expect_word(&mut split, "operation argument")?;

    let arg = if op_arg_str == "old" {
        Arg::Old
    } else {
        let op_value: i64 = aoc::parse::parse_from_str(op_arg_str, "operation value")?;
        Arg::Const(op_value)
    };

    let code = match op_char {
        "*" => Ok(OpCode::Mul),
        "+" => Ok(OpCode::Add),
        _ => Err(anyhow::anyhow!("unrecognized op symbol: `{}`", op_char)),
    }?;
    Ok(Operation { code, arg })
}

fn parse_test_condition(input: &str) -> anyhow::Result<i64> {
    let test_value = input
        .strip_prefix("Test: divisible by ")
        .ok_or_else(|| anyhow::anyhow!("test not found in: {:?}", input))?;

    aoc::parse::parse_from_str(test_value, "divisor number")
}

fn parse_throw(input: &str, side: bool) -> anyhow::Result<MonkeyId> {
    let prefix = if side {
        "If true: throw to monkey "
    } else {
        "If false: throw to monkey "
    };
    let monkey_number = input
        .strip_prefix(prefix)
        .ok_or_else(|| anyhow::anyhow!("condition `{:?}` not found in: {:?}", side, input))?;

    aoc::parse::parse_from_str(monkey_number, "monkey_number").map(MonkeyId)
}

fn get_line<'a>(input: &mut impl Iterator<Item = &'a str>) -> anyhow::Result<&'a str> {
    input
        .next()
        .map(|s| s.trim())
        .ok_or_else(|| anyhow::anyhow!("out of lines to parse"))
}
