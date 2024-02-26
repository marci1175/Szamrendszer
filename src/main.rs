use std::{env::current_exe, fmt::Debug, ops::Neg, vec};
struct DivOutput {
    num: f64,
    remainder: f64,
}

impl Debug for DivOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!(
            "Number: {}, Remainder: {}",
            self.num, self.remainder
        ))
    }
}

fn division_with_remainder(number: f64, division: f64) -> DivOutput {
    let division_num = number / division;

    let whole_num = division_num.floor();

    let remainder = (division_num - whole_num) * division;

    DivOutput {
        num: whole_num,
        remainder: remainder.round(),
    }
}

trait DecimalNumber {
    fn is_decimal(number: i64) -> bool;
}

impl DecimalNumber for i64 {
    fn is_decimal(number: i64) -> bool {
        if number > 9 || number < 0 {
            false
        } else {
            true
        }
    }
}

#[derive(Debug, Clone)]
struct Decimal {
    number: i64,
}

impl Decimal {
    fn new(num: i64) -> Self {
        if i64::is_decimal(num) {
            Self { number: num }
        } else {
            panic!("{} Cannot be converted into a decimal.", num)
        }
    }
}

#[derive(Debug, Clone)]
struct BasePair {
    base: i64,
    nth: usize,
}

impl BasePair {
    fn new(num: i64, nth: usize) -> Self {
        Self { base: num, nth }
    }
}

#[derive(Clone)]
struct DecimalPair {
    number: Decimal,
    base: BasePair,
}

impl DecimalPair {
    fn new(number: Decimal, base: i64, nth: usize) -> Self {
        Self {
            number,
            base: BasePair::new(base, nth),
        }
    }

    fn into_vec_pairs(number: i64, base: i64) -> Vec<DecimalPair> {
        let mut vec_pairs: Vec<DecimalPair> = Vec::new();

        for (index, item) in number.to_string().chars().rev().enumerate() {
            let item_num: i64 = item.to_digit(10).unwrap().into();

            vec_pairs.push(dbg!(DecimalPair::new(Decimal::new(item_num), base, index)))
        }

        vec_pairs
    }
}

impl Debug for DecimalPair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("Num: {:?}, Base: {:?}", self.number, self.base))
    }
}

fn vec_pair_into_whole(numpair_vec: Vec<DecimalPair>) -> anyhow::Result<i64> {
    let mut result_str = String::new();

    for item in numpair_vec.iter().rev() {
        result_str.push_str(&item.number.number.to_string());
    }

    Ok(result_str.parse()?)
}

fn main() -> anyhow::Result<()> {
    loop {
        let mut number = String::new();
        let mut szamrendszer = String::new();
        let mut szamrendszer_target = String::new();

        println!("Szám: ");
        std::io::stdin().read_line(&mut number)?;

        println!("Ennek a számnak a számrendszere: ");
        std::io::stdin().read_line(&mut szamrendszer)?;

        println!("Számrendszer cél: ");
        std::io::stdin().read_line(&mut szamrendszer_target)?;

        let decimal_vec =
            DecimalPair::into_vec_pairs(number.trim().parse()?, szamrendszer.trim().parse()?);

        if szamrendszer_target.trim() == "10" {
            let mut result_num: i64 = 0;

            for item in decimal_vec {
                result_num += item.number.number * (item.base.base.pow(item.base.nth as u32));
            }

            dbg!(result_num);
        } else if szamrendszer.trim() == "10" {
            let mut remainder_vec: Vec<Decimal> = Vec::new();

            let whole_num = vec_pair_into_whole(decimal_vec.clone())?;

            let divider: i64 = szamrendszer_target.trim().parse()?;

            //This will get overridden anyway
            let mut current_num: i64 = dbg!(whole_num);

            while current_num != 0 {
                let division = dbg!(division_with_remainder(current_num as f64, divider as f64));

                current_num = division.num as i64;

                remainder_vec.push(Decimal::new(division.remainder as i64));
            }

            dbg!(vec_decimal_into_whole(remainder_vec)?);
        }

        std::io::stdin().read_line(&mut String::new());
    }

    Ok(())
}

fn vec_decimal_into_whole(numpair_vec: Vec<Decimal>) -> anyhow::Result<i64> {
    let mut result_str = String::new();

    for item in numpair_vec.iter().rev() {
        result_str.push_str(&item.number.to_string());
    }

    Ok(result_str.parse()?)
}

#[test]
fn test() {
    assert_eq!(DecimalPair::into_vec_pairs(235, 10).len(), 3)
}
