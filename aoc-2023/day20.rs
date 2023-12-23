use aoc_runner_derive::{aoc, aoc_generator};
use num_integer::lcm;
use std::{
    cell::RefCell,
    collections::{HashMap, VecDeque},
    rc::Rc,
};

type SuccesorHashMap = HashMap<String, Vec<String>>;
type ModuleHashMap = HashMap<String, Rc<RefCell<dyn Module>>>;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Pulse {
    Low,
    High,
}

impl std::ops::Not for Pulse {
    type Output = Pulse;

    fn not(self) -> Self {
        match self {
            Pulse::High => Pulse::Low,
            Pulse::Low => Pulse::High,
        }
    }
}

trait Module {
    fn set_pulse(&mut self, name: String, pulse: Pulse) -> bool;
    fn get_pulse(&self) -> Pulse;
    fn add_connected(&mut self, _name: String, _pulse: Pulse) {}
}

struct FlipFlop {
    pulse: Pulse,
}

impl Module for FlipFlop {
    fn set_pulse(&mut self, _: String, pulse: Pulse) -> bool {
        match pulse {
            Pulse::High => false,
            Pulse::Low => {
                self.pulse = !self.pulse;
                true
            }
        }
    }

    fn get_pulse(&self) -> Pulse {
        self.pulse
    }
}

struct Conjunction {
    pulse: Pulse,
    connected: HashMap<String, Pulse>,
}

impl Module for Conjunction {
    fn set_pulse(&mut self, name: String, pulse: Pulse) -> bool {
        self.connected.insert(name, pulse);
        self.pulse = if self.connected.values().all(|v| v == &Pulse::High) {
            Pulse::Low
        } else {
            Pulse::High
        };
        true
    }

    fn get_pulse(&self) -> Pulse {
        self.pulse
    }

    fn add_connected(&mut self, name: String, pulse: Pulse) {
        self.connected.insert(name, pulse);
    }
}

struct Broadcaster {
    pulse: Pulse,
}

impl Module for Broadcaster {
    fn set_pulse(&mut self, _: String, pulse: Pulse) -> bool {
        self.pulse = pulse;
        true
    }

    fn get_pulse(&self) -> Pulse {
        self.pulse
    }
}

#[aoc_generator(day20)]
fn parse(input: &str) -> (SuccesorHashMap, ModuleHashMap) {
    let (mut successors, mut modules): (SuccesorHashMap, ModuleHashMap) =
        (HashMap::new(), HashMap::new());
    input.lines().for_each(|l| {
        let (module_name, succ) = l.split_once(" -> ").unwrap();

        match module_name.split_at(1) {
            ("%", s) => {
                successors.insert(
                    s.to_string(),
                    succ.split(", ").map(|s| s.to_string()).collect(),
                );
                modules.insert(
                    s.to_string(),
                    Rc::new(RefCell::new(FlipFlop { pulse: Pulse::Low })),
                )
            }
            ("&", s) => {
                successors.insert(
                    s.to_string(),
                    succ.split(", ").map(|s| s.to_string()).collect(),
                );
                modules.insert(
                    s.to_string(),
                    Rc::new(RefCell::new(Conjunction {
                        pulse: Pulse::High,
                        connected: HashMap::new(),
                    })),
                )
            }
            _ => {
                successors.insert(
                    "broadcaster".to_string(),
                    succ.split(", ").map(|s| s.to_string()).collect(),
                );
                modules.insert(
                    "broadcaster".to_string(),
                    Rc::new(RefCell::new(Broadcaster { pulse: Pulse::Low })),
                )
            }
        };
    });

    for (module, succ) in successors.iter() {
        for s in succ {
            if let Some(s) = modules.get(s) {
                s.borrow_mut().add_connected(module.to_string(), Pulse::Low)
            }
        }
    }

    (successors, modules)
}

#[aoc(day20, part1)]
fn part1(input: &(SuccesorHashMap, ModuleHashMap)) -> usize {
    let (mut lo, mut hi) = (0, 0);
    for _ in 0..1000 {
        lo += 1;
        let mut queue = VecDeque::from([("broadcaster".to_string(), Pulse::Low)]);
        while let Some((module_name, pulse)) = queue.pop_front() {
            for succ in input.0.get(&module_name).unwrap() {
                match pulse {
                    Pulse::High => hi += 1,
                    Pulse::Low => lo += 1,
                }

                if let Some(module) = input.1.get(succ) {
                    if module.borrow_mut().set_pulse(module_name.clone(), pulse) {
                        queue.push_back((succ.to_string(), module.borrow().get_pulse()));
                    }
                }
            }
        }
    }
    lo * hi
}

#[aoc(day20, part2)]
fn part2(input: &(SuccesorHashMap, ModuleHashMap)) -> usize {
    let mut zr_counts: Vec<usize> = Vec::new();
    let mut count = 0;

    loop {
        count += 1;
        let mut queue = VecDeque::from([("broadcaster".to_string(), Pulse::Low)]);
        while let Some((module_name, pulse)) = queue.pop_front() {
            for succ in input.0.get(&module_name).unwrap() {
                if succ == "zr" {
                    if pulse == Pulse::High {
                        zr_counts.push(count);
                    }
                    if zr_counts.len() > 3 {
                        return zr_counts.into_iter().reduce(lcm).unwrap();
                    }
                    continue;
                }

                if let Some(module) = input.1.get(succ) {
                    if module.borrow_mut().set_pulse(module_name.clone(), pulse) {
                        queue.push_back((succ.to_string(), module.borrow().get_pulse()));
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";

    const INPUT2: &str = "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";

    #[test]
    fn part1_example1() {
        assert_eq!(part1(&parse(INPUT1)), 32000000);
    }

    #[test]
    fn part1_example2() {
        assert_eq!(part1(&parse(INPUT2)), 11687500);
    }
}
