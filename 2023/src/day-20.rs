use std::collections::VecDeque;
use std::io::{self, BufRead};

#[derive(Debug, Clone)]
struct Module {
    name: String,
    mod_type: char,
    dests: Vec<String>,

    state: bool,
    inputs: Vec<(usize, bool)>,
}

#[derive(Debug)]
struct Pulse {
    from: usize,
    to: Option<usize>,
    value: bool,
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn lcm(vals: &[usize]) -> usize {
    vals.iter().fold(1, |acc, x| acc * x / gcd(acc, *x))
}

fn pulse_button(modules: &mut [Module], target_idx: usize) -> ([usize; 2], Vec<usize>) {
    let mut queue = VecDeque::new();
    queue.push_back(Pulse {
        from: 0,
        to: modules.iter().position(|m| m.name == "broadcaster"),
        value: false,
    });

    let mut pulse_counts = [0; 2];
    let mut pulses_to_target = vec![];
    while let Some(pulse) = queue.pop_front() {
        pulse_counts[pulse.value as usize] += 1;
        if pulse.to.is_none() {
            continue;
        }

        let module_idx = pulse.to.unwrap();
        if module_idx == target_idx && pulse.value {
            pulses_to_target.push(pulse.from);
        }

        let module = &modules[module_idx];

        match module.mod_type {
            ' ' => {
                for dest in &module.dests {
                    queue.push_back(Pulse {
                        from: module_idx,
                        to: modules.iter().position(|m| m.name == *dest),
                        value: pulse.value,
                    });
                }
            }
            '%' => {
                if !pulse.value {
                    let next_state = !module.state;
                    for dest in &module.dests {
                        queue.push_back(Pulse {
                            from: module_idx,
                            to: modules.iter().position(|m| m.name == *dest),
                            value: next_state,
                        });
                    }
                    modules[module_idx].state = !module.state;
                }
            }
            '&' => {
                let mut next_inputs = module.inputs.clone();
                next_inputs
                    .iter_mut()
                    .find(|(idx, _)| *idx == pulse.from)
                    .map(|el| el.1 = pulse.value);

                let value = !next_inputs.iter().all(|&(_, v)| v);
                for dest in &module.dests {
                    queue.push_back(Pulse {
                        from: module_idx,
                        to: modules.iter().position(|m| m.name == *dest),
                        value,
                    });
                }
                modules[module_idx].inputs = next_inputs;
            }
            _ => unreachable!(),
        };
    }
    (pulse_counts, pulses_to_target)
}

fn main() {
    let mut input = io::stdin()
        .lock()
        .lines()
        .flatten()
        .flat_map(|line| {
            if let Some((module, dests)) = line.split_once(" -> ") {
                let mut mod_type = module.chars().next().unwrap();
                if mod_type.is_alphabetic() {
                    mod_type = ' ';
                }
                let name = if mod_type == ' ' {
                    module
                } else {
                    &module[1..]
                };

                Some(Module {
                    name: name.to_string(),
                    mod_type,
                    dests: dests.split(',').map(|x| x.trim().to_string()).collect(),
                    state: false,
                    inputs: vec![],
                })
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    for i in 0..input.len() {
        if input[i].mod_type != '&' {
            continue;
        }
        input[i].inputs = input
            .iter()
            .enumerate()
            .filter(|(_, m)| m.dests.contains(&input[i].name))
            .map(|(idx, _)| (idx, false))
            .collect();
    }

    let mut module_state = input.clone();
    let (mut low_pulses, mut high_pulses) = (0, 0);
    for _ in 0..1000 {
        let (pulses, _) = pulse_button(&mut module_state, 0);
        low_pulses += pulses[0];
        high_pulses += pulses[1];
    }
    println!("(1) Total pulses sent is {}", low_pulses * high_pulses);

    let target = input
        .iter()
        .position(|m| m.dests.contains(&"rx".to_string()))
        .unwrap();
    let mut rx_conj_inputs = input[target]
        .inputs
        .iter()
        .map(|(idx, _)| (*idx, 0))
        .collect::<Vec<_>>();
    let mut module_state = input.clone();
    for i in 1..4086 {
        let (_, pulses_to_target) = pulse_button(&mut module_state, target);
        for pulse in pulses_to_target {
            rx_conj_inputs
                .iter_mut()
                .find(|(idx, _)| *idx == pulse)
                .map(|el| el.1 = i);
        }
    }
    println!(
        "(2) Module rx will be pulsed after {} button presses",
        lcm(&rx_conj_inputs.iter().map(|(_, i)| *i).collect::<Vec<_>>()[..])
    );
}
