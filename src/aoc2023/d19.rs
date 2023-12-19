use std::collections::HashMap;

#[derive(Clone, Copy, Debug)]
enum Operator {
    Lt,
    Gt,
}
#[derive(Clone, Copy, Debug)]
enum Coord {
    X,
    M,
    A,
    S,
}
struct Condition {
    coord: Coord,
    operator: Operator,
    value: i64,
}
impl Condition {
    fn check(&self, value: &Value) -> bool {
        match self.operator {
            Operator::Gt => value.get_coord(self.coord) > self.value,
            Operator::Lt => value.get_coord(self.coord) < self.value,
        }
    }
}
struct Rule {
    condition: Option<Condition>,
    target: String,
}
impl Rule {
    fn apply(&self, value: &Value) -> Option<String> {
        self.condition
            .as_ref()
            .map(|c| c.check(value))
            .unwrap_or(true)
            .then(|| self.target.clone())
    }
}
struct Value {
    x: i64,
    m: i64,
    a: i64,
    s: i64,
}
impl Value {
    fn get_coord(&self, coord: Coord) -> i64 {
        match coord {
            Coord::X => self.x,
            Coord::M => self.m,
            Coord::A => self.a,
            Coord::S => self.s,
        }
    }
}

pub fn f(input: crate::AocInput) -> crate::AocResult {
    let mut lines = input.lines();
    let mut rulechains = HashMap::new();
    loop {
        let l = lines.next().unwrap().unwrap();
        if l.is_empty() {
            break;
        }
        let (name, rest) = l.split_once('{').unwrap();
        let mut rulechain = Vec::new();
        for rule in rest[0..rest.len() - 1].split(',') {
            let rule = if let Some((condition, target)) = rule.split_once(':') {
                let mut cond_chars = condition.chars();
                let coord = match cond_chars.next().unwrap() {
                    'x' => Coord::X,
                    'm' => Coord::M,
                    'a' => Coord::A,
                    's' => Coord::S,
                    c => panic!("invalid coordinate: '{}'", c),
                };
                let operator = match cond_chars.next().unwrap() {
                    '<' => Operator::Lt,
                    '>' => Operator::Gt,
                    c => panic!("invalid operator: '{}'", c),
                };
                let value = condition.split_at(2).1.parse().unwrap();
                Rule {
                    condition: Some(Condition {
                        coord,
                        operator,
                        value,
                    }),
                    target: target.to_owned(),
                }
            } else {
                Rule {
                    condition: None,
                    target: rule.to_owned(),
                }
            };
            rulechain.push(rule);
        }
        rulechains.insert(name.to_owned(), rulechain);
    }
    let rulechains = rulechains;
    let mut res1 = 0;
    for l in lines {
        let l = l.unwrap();
        let mut value = Value {
            x: 0,
            m: 0,
            a: 0,
            s: 0,
        };
        for v in l[1..l.len() - 1].split(',') {
            let (coord, val) = v.split_once('=').unwrap();
            match coord {
                "x" => value.x = val.parse().unwrap(),
                "m" => value.m = val.parse().unwrap(),
                "a" => value.a = val.parse().unwrap(),
                "s" => value.s = val.parse().unwrap(),
                s => panic!("Invalid coordinate '{}'", s),
            }
        }
        let mut current_rulechain = "in".to_string();
        while current_rulechain != "A" && current_rulechain != "R" {
            let rc = rulechains.get(&current_rulechain).unwrap();
            for r in rc {
                if let Some(x) = r.apply(&value) {
                    current_rulechain = x;
                    break;
                }
            }
        }
        if current_rulechain == "A" {
            res1 += value.x + value.m + value.a + value.s;
        }
    }
    res1.into()
}
