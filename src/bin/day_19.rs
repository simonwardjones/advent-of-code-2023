use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

const RAW_DATA: &str = include_str!("../../input/day_19.txt");

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    // Code block to measure.
    {
        part_one();
        part_two();
    }

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}

#[derive(Debug, Clone)]
enum Category {
    X,
    M,
    A,
    S,
}

#[derive(Debug, Clone)]
pub struct XMASPart {
    x: i32,
    m: i32,
    a: i32,
    s: i32,
}

impl XMASPart {
    fn total(&self) -> i32 {
        self.x + self.m + self.a + self.s
    }
}

impl From<&str> for XMASPart {
    fn from(s: &str) -> Self {
        let mut xmas_part = XMASPart {
            x: 0,
            m: 0,
            a: 0,
            s: 0,
        };
        lazy_static! {
            static ref XMAS_PART_RE: Regex = Regex::new(r"(\w+)=(\d+)").unwrap();
        }
        for capture in XMAS_PART_RE.captures_iter(s) {
            match capture.get(1).unwrap().as_str() {
                "x" => xmas_part.x = capture.get(2).unwrap().as_str().parse::<i32>().unwrap(),
                "m" => xmas_part.m = capture.get(2).unwrap().as_str().parse::<i32>().unwrap(),
                "a" => xmas_part.a = capture.get(2).unwrap().as_str().parse::<i32>().unwrap(),
                "s" => xmas_part.s = capture.get(2).unwrap().as_str().parse::<i32>().unwrap(),
                _ => panic!("Unknown label"),
            }
        }
        xmas_part
    }
}

impl From<&str> for Category {
    fn from(s: &str) -> Self {
        match s {
            "X" | "x" => Category::X,
            "M" | "m" => Category::M,
            "A" | "a" => Category::A,
            "S" | "s" => Category::S,
            _ => panic!("Unknown category: {}", s),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Operator {
    LessThan,
    GreaterThan,
}

impl From<&str> for Operator {
    fn from(s: &str) -> Self {
        match s {
            "<" => Operator::LessThan,
            ">" => Operator::GreaterThan,
            _ => panic!("Unknown operator: {}", s),
        }
    }
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Condition {
    category: Category,
    operator: Operator,
    value: i32,
}

impl Condition {
    fn is_true(&self, part: &XMASPart) -> bool {
        use Category::*;
        use Operator::*;
        match self.operator {
            LessThan => match self.category {
                X => part.x < self.value,
                M => part.m < self.value,
                A => part.a < self.value,
                S => part.s < self.value,
            },
            GreaterThan => match self.category {
                X => part.x > self.value,
                M => part.m > self.value,
                A => part.a > self.value,
                S => part.s > self.value,
            },
        }
    }
}

#[derive(Debug, Clone)]
struct Rule {
    rule_name: &'static str,
    condition: Option<Condition>,
}

impl Rule {
    fn process(&self, part: &XMASPart) -> Option<&'static str> {
        // println!(
        //     "Processing rule: {:?} with condition {:?}",
        //     self, self.condition
        // );
        match &self.condition {
            Some(condition) => {
                if condition.is_true(part) {
                    Some(self.rule_name)
                } else {
                    None
                }
            }
            None => Some(self.rule_name),
        }
    }
}

impl From<&'static str> for Rule {
    fn from(s: &'static str) -> Self {
        lazy_static! {
            static ref RE_SIMPLE: Regex = Regex::new(r"^\w+$").unwrap();
        }
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(\w+?)?(>|<)?(\d+)?:?(\w+)").unwrap();
        }
        // super slow without lazy_static
        // let RE: Regex = Regex::new(r"(\w+)(>|<)(\d+):(\w+)").unwrap();

        if let Some(c) = RE_SIMPLE.captures(s) {
            return Rule {
                rule_name: c.get(0).unwrap().as_str(),
                condition: None,
            };
        }
        match RE.captures(s) {
            Some(c) => {
                let category = Category::from(c.get(1).unwrap().as_str());
                let operator = Operator::from(c.get(2).unwrap().as_str());
                let value = c.get(3).unwrap().as_str().parse::<i32>().unwrap();
                let rule_name = c.get(4).unwrap().as_str();
                Rule {
                    rule_name,
                    condition: Some(Condition {
                        category,
                        operator,
                        value,
                    }),
                }
            }
            None => panic!("No match"),
        }
    }
}

#[derive(Debug, Clone)]
struct WorkFlow {
    name: &'static str,
    rules: Vec<Rule>,
}

impl WorkFlow {
    fn process(&self, part: &XMASPart) -> &'static str {
        self.rules
            .iter()
            .find_map(|r| r.process(&part))
            .expect("No rule found")
    }
}

#[derive(Debug, Clone)]
struct WorkFlowEngine {
    workflows_by_name: HashMap<&'static str, WorkFlow>,
    rejected_parts: Vec<XMASPart>,
    accepted_parts: Vec<XMASPart>,
    parts: Vec<XMASPart>,
}
impl WorkFlowEngine {
    fn process(&mut self) {
        for part in self.parts.iter() {
            // println!("Processing part: {:?}", part);
            let mut flow = self.workflows_by_name.get(&"in").unwrap();
            loop {
                let flow_name = flow.process(&part);
                if flow_name == "A" {
                    self.accepted_parts.push(part.clone());
                    break;
                }
                if flow_name == "R" {
                    self.rejected_parts.push(part.clone());
                    break;
                }
                flow = &self.workflows_by_name[flow_name]
            }
        }
    }
}

impl From<&'static str> for WorkFlow {
    fn from(s: &'static str) -> Self {
        lazy_static! {
            static ref WORKFLOW_RE: Regex = Regex::new(r"(\w+)\{(.*)\}").unwrap();
        }
        match WORKFLOW_RE.captures(s) {
            Some(c) => {
                let name = c.get(1).unwrap().as_str();
                let rules = c
                    .get(2)
                    .unwrap()
                    .as_str()
                    .split(",")
                    .map(Rule::from)
                    .collect();
                WorkFlow { name, rules }
            }
            _ => panic!("No rules found"),
        }
    }
}

fn load_data() -> (Vec<WorkFlow>, Vec<XMASPart>) {
    let (workflows, part_ratings) = RAW_DATA.split_once("\n\n").expect("Couldn't split data");
    let workflows: Vec<WorkFlow> = workflows.lines().map(WorkFlow::from).collect();
    let part_ratings: Vec<XMASPart> = part_ratings.lines().map(XMASPart::from).collect();
    (workflows, part_ratings)
}

#[allow(dead_code, unused_variables)]
fn part_one() {
    println!("Part 1");

    let (workflows, part_ratings) = load_data();
    let mut engine = WorkFlowEngine {
        workflows_by_name: workflows.into_iter().map(|w| (w.name, w)).collect(),
        rejected_parts: Vec::new(),
        accepted_parts: Vec::new(),
        parts: part_ratings,
    };
    println!("engine: {engine:?}");
    engine.process();
    // println!("engine: {engine:?}");
    let total = engine.accepted_parts.iter().map(|p| p.total()).sum::<i32>();
    println!("Total: {}", total);
}

#[allow(dead_code, unused_variables)]
fn part_two() {
    println!("Part 2");
    let data = load_data();
}
