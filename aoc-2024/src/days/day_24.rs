//!day_24.rs

use anyhow::Result;
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Operator {
    And,
    Or,
    Xor,
}

impl Operator {
    fn get_output(&self, input1: bool, input2: bool) -> bool {
        match self {
            Operator::And => input1 & input2,
            Operator::Or => input1 | input2,
            Operator::Xor => input1 ^ input2,
        }
    }
}

impl From<&str> for Operator {
    fn from(value: &str) -> Self {
        match value {
            "AND" => Operator::And,
            "OR" => Operator::Or,
            "XOR" => Operator::Xor,
            _ => panic!("Unknown operator."),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Gate {
    operator: Operator,
    input1: String,
    input2: String,
}

impl Gate {
    fn new(operator: Operator, in1: &str, in2: &str) -> Self {
        let (input1, input2) = if in1 < in2 {
            (in1.to_string(), in2.to_string())
        } else {
            (in2.to_string(), in1.to_string())
        };
        Self {
            operator,
            input1,
            input2,
        }
    }
}

#[derive(Debug)]
struct Day24Data {
    gates: HashMap<String, Gate>,
    values: HashMap<String, bool>,
}

impl From<&str> for Day24Data {
    fn from(value: &str) -> Self {
        let mut gates: HashMap<String, Gate> = HashMap::new();
        let mut values: HashMap<String, bool> = HashMap::new();
        let (init_values, gate_definitions) = value.split_once("\n\n").unwrap();
        for (node, value) in init_values.lines().filter_map(|l| l.split_once(": ")) {
            let value: bool = value == "1";
            values.insert(node.to_owned(), value);
        }
        let re = Regex::new(r"(\w+)\s+(XOR|OR|AND)\s+(\w+)\s+->\s+(\w+)").unwrap();
        for (output, operator, input1, input2) in gate_definitions.lines().filter_map(|line| {
            re.captures(line.trim()).map(|captures| {
                let input1 = captures.get(1).unwrap().as_str();
                let operator = Operator::from(captures.get(2).unwrap().as_str());
                let input2 = captures.get(3).unwrap().as_str();
                let output = captures.get(4).unwrap().as_str();
                (output, operator, input1, input2)
            })
        }) {
            gates.insert(output.to_owned(), Gate::new(operator, input1, input2));
        }

        Self { gates, values }
    }
}

impl Day24Data {
    fn calc_z_values(&mut self) -> u128 {
        let mut z_values: Vec<String> = self
            .gates
            .keys()
            .filter(|k| &k[..1] == "z")
            .cloned()
            .collect();
        z_values.sort();
        let mut output = 0;
        for z_node in z_values.iter().rev() {
            output <<= 1;
            let value: u128 = if self.calc_value(z_node) { 1 } else { 0 };
            output += value;
        }
        output
    }
    fn calc_value(&mut self, node: &String) -> bool {
        if let Some(value) = self.values.get(node) {
            return *value;
        }
        let gate = self.gates.get(node).cloned().unwrap();
        let input1 = self.calc_value(&gate.input1);
        let input2 = self.calc_value(&gate.input2);
        gate.operator.get_output(input1, input2)
    }
    fn check_add_operator_tree(&mut self) -> String {
        // A bitwise add operation only with AND, OR, and XOR gates requires the following two formulas
        // be true for all input bit i of x and y
        // Sum_i = A_i XOR B_i XOR Carry_i-1
        // Carry_i = (A_i AND B_i) OR (Carry_i-1 AND (A_i XOR B_i))
        // Special case 1: i = 0 -> Carry_i-1 = 0
        // Sum_0 = A_0 XOR B_0
        // Carry_0 = A_0 AND B_0
        // Special case 2: i = 45 -> x and y have only bits until i = 44
        // -> Carry_44 = z45
        //
        // Hypothesis 1:
        // Every swap signal pair is to be identified via the Sum Formula, because because swap signals
        // results in wrong Sum result.
        // -> Check Sum Formulas starting from lsb up to msb and fetch carry signal of each bit
        // Hypothesis 2:
        // Every previous Carry signal before current Sum bit are correct, because of the recursive nature of
        // carry signal. There could only be a swapped signal in Carry_i-1. Fore sake of simplicity we ignore
        // this possibility for our first attempt and assume, that Carry_i-1 is although always correct.
        //

        // key: c00 to c44; value: signal name
        let mut carries: HashMap<String, String> = HashMap::new();
        // Vec to save signals to swap
        let mut signals_to_swap: Vec<String> = Vec::new();
        // check sum and get carry signal, starting from lsb upwards
        for bit in 0..45_u128 {
            let z = format!("z{:02}", bit);
            let x = format!("x{:02}", bit);
            let y = format!("y{:02}", bit);
            let input_xor = Gate::new(Operator::Xor, &x, &y);
            let carry_out = if bit == 0 {
                assert_eq!(z, self.get_signal_from_gate(&input_xor).unwrap());
                let carry_gate = Gate::new(Operator::And, &x, &y);
                let signal_name = self.get_signal_from_gate(&carry_gate).unwrap();
                carries.insert("c00".into(), signal_name.clone());
                signal_name
            } else {
                // carry names
                let carry_in = format!("c{:02}", bit - 1);
                let carry_out = format!("c{:02}", bit);
                // check sum signals
                // input xor signal always exist (but may be falsely linked)
                let mut input_xor_signal = self.get_signal_from_gate(&input_xor).unwrap();
                // We assume Carry signal is correct (see below).
                let carry_in_signal = carries.get(&carry_in).cloned().unwrap();
                // set sum_gate of current bit and extract signal name from gate
                let sum_gate = Gate::new(Operator::Xor, &input_xor_signal, &carry_in_signal);
                if let Some(sum_gate_signal) = self.get_signal_from_gate(&sum_gate) {
                    // found signal name, but it could be swapped. Check it...
                    if sum_gate_signal != z {
                        // signal is swapped with current z. Correct ist
                        self.swap_gate_signals(&sum_gate_signal, &z);
                        signals_to_swap.push(sum_gate_signal.clone());
                        signals_to_swap.push(z.clone());
                    }
                } else {
                    // if sum_gate_signal is not found, than probably the input_xor_signal is swapped
                    // extract swap_xor_signal from current z (we assume correct carry_in_signal)
                    let z_gate = self.gates.get(&z).unwrap();
                    let swap_xor_signal = if z_gate.input1 == carry_in_signal {
                        z_gate.input2.to_owned()
                    } else {
                        z_gate.input1.to_owned()
                    };
                    self.swap_gate_signals(&swap_xor_signal, &input_xor_signal);
                    signals_to_swap.push(swap_xor_signal.clone());
                    signals_to_swap.push(input_xor_signal.clone());
                    input_xor_signal = swap_xor_signal;
                }
                // extract carry signal by putting together all required gates and save it to carries.
                // we assume, that if the sum_gate_signal is correct (or fixed by sre-swapping swapped signals)
                // if any of the following unwraps result in a panic!, Hypothesis 2 is invalid and we
                // need a better approach.
                let input_and_gate = Gate::new(Operator::And, &x, &y);
                // input and signal should always exist
                let input_and_gate_signal = self.get_signal_from_gate(&input_and_gate).unwrap();
                let carry_in_and_gate =
                    Gate::new(Operator::And, &carry_in_signal, &input_xor_signal);
                let carry_in_and_gate_signal =
                    self.get_signal_from_gate(&carry_in_and_gate).unwrap();
                let carry_out_or_gate = Gate::new(
                    Operator::Or,
                    &input_and_gate_signal,
                    &carry_in_and_gate_signal,
                );
                let carry_out_or_gate_signal =
                    self.get_signal_from_gate(&carry_out_or_gate).unwrap();
                carries.insert(carry_out, carry_out_or_gate_signal.clone());
                carry_out_or_gate_signal
            };
            // test carry: set only current bit in x and y. Carry must be true.
            let num = 1_u128 << bit;
            self.set_x(num);
            self.set_y(num);
            assert!(self.calc_value(&carry_out));
        }
        assert_eq!(carries.get("c44"), Some(&"z45".into()));
        assert_eq!(signals_to_swap.len(), 8);
        assert!(self.test_add_over_all_bits());

        signals_to_swap.sort();
        signals_to_swap.join(",")
    }

    fn get_signal_from_gate(&self, gate: &Gate) -> Option<String> {
        self.gates
            .iter()
            .find(|(_, g)| *g == gate)
            .map(|(sn, _)| sn.to_owned())
    }

    fn swap_gate_signals(&mut self, signal_1: &str, signal_2: &str) {
        let gate_signal_1 = self.gates.get(signal_1).unwrap().to_owned();
        let gate_signal_2 = self.gates.get(signal_2).unwrap().to_owned();
        self.gates.insert(signal_1.to_string(), gate_signal_2);
        self.gates.insert(signal_2.to_string(), gate_signal_1);
    }
    fn test_add_over_all_bits(&mut self) -> bool {
        let value = (1_u128 << 45) - 1;
        self.set_x(value);
        self.set_y(value);
        let z = self.calc_z_values();
        value + value == z
    }
    fn set_x(&mut self, mut num: u128) {
        num &= (1_u128 << 45) - 1;
        for shift in 0..45_u128 {
            let bit = 1 << shift;
            let value = num & bit > 0;
            let key = format!("x{:02}", shift);
            self.values.insert(key, value);
        }
    }
    fn set_y(&mut self, mut num: u128) {
        num &= (1_u128 << 45) - 1;
        for shift in 0..45_u128 {
            let bit = 1 << shift;
            let value = num & bit > 0;
            let key = format!("y{:02}", shift);
            self.values.insert(key, value);
        }
    }
}

pub fn day_24() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2024/day_24.txt");
    let mut challenge = Day24Data::from(input);

    let result_part1 = challenge.calc_z_values();
    println!("result day 24 part 1: {}", result_part1);
    assert_eq!(result_part1, 56_620_966_442_854);

    let result_part2 = challenge.check_add_operator_tree();
    println!("result day 24 part 2: {}", result_part2);
    assert_eq!(result_part2, "chv,jpj,kgj,rts,vvw,z07,z12,z26");

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn some_experimenting() {
        // I used these tests to manually get nearer to the solution
        let input = include_str!("../../../../aoc_input/aoc-2024/day_24.txt");
        let mut challenge = Day24Data::from(input);
        challenge.set_x(1);
        assert_eq!(challenge.values.get("x00"), Some(&true));
        for v in 1..45 {
            let key = format!("x{:02}", v);
            assert_eq!(challenge.values.get(&key), Some(&false));
        }

        for exponent in 0..45_u128 {
            let n: u128 = 1 << exponent;
            let m = n + n;
            challenge.set_x(n);
            challenge.set_y(n);
            let z = challenge.calc_z_values();
            if z != m {
                dbg!(exponent);
                dbg!(n);
                dbg!(m);
                dbg!(z);
            }
        }
        let gates: HashMap<String, Gate> = challenge
            .gates
            .iter()
            .filter(|(_, g)| g.operator == Operator::Xor)
            .filter(|(s, g)| {
                (&s[..1] != "z" && &g.input1[..1] != "x")
                    || (&s[..1] == "z" && &g.input1[..1] == "x" && *s != "z00")
            })
            .map(|(s, g)| (s.to_owned(), g.to_owned()))
            .collect();
        dbg!(gates);
        let gates: HashMap<String, Gate> = challenge
            .gates
            .iter()
            .filter(|(s, g)| &s[..1] == "z" && g.operator != Operator::Xor)
            .map(|(s, g)| (s.to_owned(), g.to_owned()))
            .collect();
        dbg!(gates);
        let mut possible_signals = ["rts", "z07", "jpj", "z12", "kgj", "z26", "vvw", "chv"];
        possible_signals.sort();
        println!("{}", possible_signals.join(","));
        challenge.check_add_operator_tree();
    }

    #[test]
    fn test_example_part() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2024/day_24_example.txt");
        let mut challenge = Day24Data::from(input);

        let result_part1 = challenge.calc_z_values();
        println!("result day 24 part 1: {}", result_part1);
        assert_eq!(result_part1, 2_024);
        /*
        let result_part2 = challenge
        println!("result day 24 part 2: {}", result_part2);
        assert_eq!(result_part2, XXX);
        */
        Ok(())
    }
}
