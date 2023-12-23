use std::collections::{HashMap, VecDeque};

trait Module {
    fn connect_src<'a>(&mut self, module: String);
    fn connect_dst<'a>(&mut self, module: String);
    fn receive(&mut self, source: &str, value: bool) -> (bool, Vec<String>);
    fn invals(&self) -> Option<HashMap<String, bool>>;
}

struct FlipFlop {
    state: bool,
    targets: Vec<String>,
}

impl FlipFlop {
    fn new() -> Self {
        FlipFlop {
            state: false,
            targets: Vec::new(),
        }
    }
}

impl Module for FlipFlop {
    fn connect_src<'a>(&mut self, _module: String) {}

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
    fn invals(&self) -> Option<HashMap<String, bool>> {
        None
    }
}

struct Conjunction {
    targets: Vec<String>,
    source_vals: HashMap<String, bool>,
}

impl Conjunction {
    fn new() -> Self {
        Conjunction {
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
    fn invals(&self) -> Option<HashMap<String, bool>> {
        Some(self.source_vals.clone())
    }
}

struct Broadcast {
    targets: Vec<String>,
}

impl Broadcast {
    fn new() -> Self {
        Broadcast {
            targets: Vec::new(),
        }
    }
}

impl Module for Broadcast {
    fn connect_src<'a>(&mut self, _module: String) {}

    fn connect_dst<'a>(&mut self, module: String) {
        self.targets.push(module);
    }

    fn receive(&mut self, _source: &str, value: bool) -> (bool, Vec<String>) {
        (value, self.targets.clone())
    }
    fn invals(&self) -> Option<HashMap<String, bool>> {
        None
    }
}

struct Dummy;
impl Module for Dummy {
    fn connect_src<'a>(&mut self, _module: String) {}

    fn connect_dst<'a>(&mut self, _module: String) {}

    fn receive(&mut self, _source: &str, _value: bool) -> (bool, Vec<String>) {
        (false, vec![])
    }
    fn invals(&self) -> Option<HashMap<String, bool>> {
        None
    }
}

fn connect(modules: &mut HashMap<String, Box<dyn Module>>, from: &str, to: &str) {
    modules.get_mut(from).unwrap().connect_dst(to.to_owned());
    if !modules.contains_key(to) {
        modules.insert(to.to_owned(), Box::new(Dummy));
    }
    modules.get_mut(to).unwrap().connect_src(from.to_owned());
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
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
            '%' => Box::new(FlipFlop::new()),
            '&' => Box::new(Conjunction::new()),
            _ => Box::new(Broadcast::new()),
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

    let mut offsets = HashMap::new();
    let mut periods = HashMap::new();

    'outer: for i in 0u64.. {
        if i == 1000 {
            res1 = counter_high * counter_low;
        }
        next_signals.push_back(("".to_owned(), false, "roadcaster".to_owned()));
        while let Some((src, value, dst)) = next_signals.pop_front() {
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
            for (k, v) in modules[&rx_src].invals().unwrap() {
                if !v {
                    continue;
                }
                if let Some(i0) = offsets.get(&k) {
                    if i == *i0 {
                        break;
                    }
                    if !periods.contains_key(&k) {
                        periods.insert(k, i - i0);
                    }
                } else {
                    offsets.insert(k, i);
                }
            }
            if periods.len() == modules[&rx_src].invals().unwrap().len() {
                break 'outer;
            }
        }
    }

    let mut res2 = 1;

    for p in periods.values() {
        res2 = res2 * p / gcd(res2, *p);
    }

    (res1, res2).into()
}
