use std::cell::{Cell, RefCell};
use std::collections::{HashMap, HashSet, VecDeque};

trait Module {
    fn connect_src<'a>(&mut self, module: String);
    fn connect_dst<'a>(&mut self, module: String);
    fn receive(&mut self, source: &str, value: bool) -> (bool, Vec<String>);
}

struct FlipFlop {
    name: String,
    state: bool,
    targets: Vec<String>,
}

impl FlipFlop {
    fn new(name: String) -> Self {
        FlipFlop {
            name,
            state: false,
            targets: Vec::new(),
        }
    }
}

impl Module for FlipFlop {
    fn connect_src<'a>(&mut self, module: String) {}

    fn connect_dst<'a>(&mut self, module: String) {
        self.targets.push(module);
    }

    fn receive(&mut self, _source: &str, value: bool) -> (bool, Vec<String>) {
        if value {
            return (false, vec![]);
        }
        self.state = !self.state;
        (self.state, self.targets.clone())
    }
}

struct Conjunction {
    name: String,
    targets: Vec<String>,
    source_vals: HashMap<String, bool>,
}

impl Conjunction {
    fn new(name: String) -> Self {
        Conjunction {
            name,
            targets: Vec::new(),
            source_vals: HashMap::new(),
        }
    }
}
impl Module for Conjunction {
    fn connect_src<'a>(&mut self, module: String) {
        self.source_vals.insert(module, false);
    }

    fn connect_dst<'a>(&mut self, module: String) {
        self.targets.push(module);
    }

    fn receive(&mut self, source: &str, value: bool) -> (bool, Vec<String>) {
        *self.source_vals.get_mut(source).unwrap() = value;
        let to_send = !self.source_vals.iter().all(|(_, x)| *x);
        (to_send, self.targets.clone())
    }
}

struct Broadcast {
    name: String,
    targets: Vec<String>,
}

impl Broadcast {
    fn new(name: String) -> Self {
        Broadcast {
            name,
            targets: Vec::new(),
        }
    }
}

impl Module for Broadcast {
    fn connect_src<'a>(&mut self, module: String) {}

    fn connect_dst<'a>(&mut self, module: String) {
        self.targets.push(module);
    }

    fn receive(&mut self, _source: &str, value: bool) -> (bool, Vec<String>) {
        (value, self.targets.clone())
    }
}

struct Dummy;
impl Module for Dummy {
    fn connect_src<'a>(&mut self, module: String) {}

    fn connect_dst<'a>(&mut self, module: String) {}

    fn receive(&mut self, source: &str, value: bool) -> (bool, Vec<String>) {
        (false, vec![])
    }
}

fn connect(modules: &mut HashMap<String, Box<dyn Module>>, from: &str, to: &str) {
    modules.get_mut(from).unwrap().connect_dst(to.to_owned());
    if !modules.contains_key(to) {
        modules.insert(to.to_owned(), Box::new(Dummy));
    }
    modules.get_mut(to).unwrap().connect_src(from.to_owned());
}

pub fn f(input: crate::AocInput) -> crate::AocResult {
    let mut modules = HashMap::new();
    let mut connections = Vec::new();
    for l in input.lines() {
        let l = l.unwrap();
        let (from, to) = l.split_once(" -> ").unwrap();
        let mut cs = from.chars();
        let mod_type = cs.next().unwrap();
        let from_name: String = cs.collect();
        let new_mod: Box<dyn Module> = match mod_type {
            '%' => Box::new(FlipFlop::new(from_name.clone())),
            '&' => Box::new(Conjunction::new(from_name.clone())),
            _ => Box::new(Broadcast::new(from_name.clone())),
        };
        modules.insert(from_name.clone(), new_mod);
        for target in to.split(", ") {
            connections.push((from_name.clone(), target.to_owned()));
        }
    }
    let mut rx_src = "".to_string();
    for (src, dst) in connections {
        if dst == "rx" {
            rx_src = src.to_owned();
        }
        connect(&mut modules, &src, &dst);
    }

    let mut next_signals = VecDeque::new();
    let mut counter_low = 0;
    let mut counter_high = 0;

    let mut res1 = 0;
    let mut res2 = 0;

    'outer: for i in 0..1001 {
        if i == 1000 {
            res1 = counter_high * counter_low;
        }
        if i % 100000 == 0 {
            println!("{}", i);
        }
        next_signals.push_back(("".to_owned(), false, "roadcaster".to_owned()));
        let mut n = 0;
        while let Some((src, value, dst)) = next_signals.pop_front() {
            if dst == "rx" && !value {
                res2 = i;
                break 'outer;
            }
            //println!("{} -{}- -> {}",src, value,dst);
            n += 1;
            //dbg!(&next_signals);
            if value {
                counter_high += 1;
            } else {
                counter_low += 1;
            }
            let new_signals = modules.get_mut(&dst).unwrap().receive(&src, value);
            let s_val = new_signals.0;
            for t in new_signals.1 {
                next_signals.push_back((dst.to_owned(), s_val, t));
            }
            //dbg!(&next_signals);
            if n > 3 {
                //panic!()
            }
        }
    }

    (res1, res2).into()
}
