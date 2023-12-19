use std::collections::HashMap;

use regex::Regex;

const RAW_DATA: &str = include_str!("../../input/day_19.txt");

fn main() {
    part_two();
}

#[derive(Debug, Clone)]
enum Category {
    X,
    M,
    A,
    S,
}

#[derive(Debug, Clone, Copy)]
pub struct XMASPart {
    x: (i32, i32),
    m: (i32, i32),
    a: (i32, i32),
    s: (i32, i32),
}

impl XMASPart {
    fn set_value_copy(&self, category: &Category, value: (i32, i32)) -> XMASPart {
        let mut new = self.clone();
        match category {
            Category::X => new.x = value,
            Category::M => new.m = value,
            Category::A => new.a = value,
            Category::S => new.s = value,
        }
        new
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
    fn split_part(&self, part: &XMASPart) -> Vec<(XMASPart, bool)> {
        use Operator::*;
        let v = self.value;
        let c = &self.category;
        let p = match self.category {
            Category::X => part.x,
            Category::M => part.m,
            Category::A => part.a,
            Category::S => part.s,
        };
        match self.operator {
            LessThan => {
                // println!("{} < {}", v, part.0);
                if v <= p.0 {
                    return vec![(part.clone(), false)];
                } else if v <= p.1 {
                    return vec![
                        (part.set_value_copy(c, (p.0, v - 1)), true),
                        (part.set_value_copy(c, (v, p.1)), false),
                    ];
                } else {
                    return vec![(part.clone(), true)];
                }
            }
            GreaterThan => {
                // println!("{} < {}", v, part.0);
                if v >= p.1 {
                    return vec![(part.clone(), false)];
                } else if v >= p.0 {
                    return vec![
                        (part.set_value_copy(c, (p.0, v)), false),
                        (part.set_value_copy(c, (v + 1, p.1)), true),
                    ];
                } else {
                    return vec![(part.clone(), true)];
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
struct Rule {
    next_flow_name: &'static str,
    condition: Option<Condition>,
}

impl From<&'static str> for Rule {
    fn from(s: &'static str) -> Self {
        // println!("creating rule from: {}", s);
        let re_simple = Regex::new(r"^\w+$").unwrap();
        let re = Regex::new(r"(\w+?)?(>|<)?(\d+)?:?(\w+)").unwrap();
        if let Some(c) = re_simple.captures(s) {
            return Rule {
                next_flow_name: c.get(0).unwrap().as_str(),
                condition: None,
            };
        }
        match re.captures(s) {
            Some(c) => {
                let category = Category::from(c.get(1).unwrap().as_str());
                let operator = Operator::from(c.get(2).unwrap().as_str());
                let value = c.get(3).unwrap().as_str().parse::<i32>().unwrap();
                let next_flow_name = c.get(4).unwrap().as_str();
                Rule {
                    next_flow_name,
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

#[derive(Debug, Clone)]
struct WorkFlowEngine {
    workflows_by_name: HashMap<&'static str, WorkFlow>,
    rejected_parts: Vec<XMASPart>,
    accepted_parts: Vec<XMASPart>,
}
impl WorkFlowEngine {
    fn process(&mut self, part: XMASPart) {
        let mut to_process = vec![(part, "in")];
        while let Some((part, flow_name)) = to_process.pop() {
            println!("Processing flow {} for part {:?}", flow_name, part);
            if flow_name == "A" {
                self.accepted_parts.push(part.clone());
                continue;
            }
            if flow_name == "R" {
                self.rejected_parts.push(part.clone());
                continue;
            }
            let flow = self.workflows_by_name.get(flow_name).unwrap();

            let mut parts_rules = vec![(part, 0)];
            while let Some((part, i)) = parts_rules.pop() {
                if i >= flow.rules.len() {
                    println!("ERROR");
                    break;
                }
                let rule = &flow.rules[i];
                // println!("Processing rule: {:?} for part {:?} {}", rule, part, i);
                if let Some(condition) = &rule.condition {
                    let parts: Vec<(XMASPart, bool)> = condition.split_part(&part);
                    for (part, t_f) in parts {
                        if t_f {
                            // println!(
                            //     "Adding part to process {:?} {:?}",
                            //     part, rule.next_flow_name
                            // );
                            to_process.push((part, rule.next_flow_name));
                        } else {
                            // println!("Adding part rule to process {:?} {:?}", part, i + 1);
                            parts_rules.push((part, i + 1));
                        }
                    }
                } else {
                    // println!("No condition start new flow! {part:?} for {:?}", rule.next_flow_name);
                    to_process.push((part, rule.next_flow_name));
                }
            }
        }
    }
}

impl From<&'static str> for WorkFlow {
    fn from(s: &'static str) -> Self {
        // println!("creating workflow from: {}", s);
        let re = Regex::new(r"(\w+)\{(.*)\}").unwrap();
        match re.captures(s) {
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

fn load_data() -> Vec<WorkFlow> {
    let (workflows, _) = RAW_DATA.split_once("\n\n").expect("Couldn't split data");
    let workflows: Vec<WorkFlow> = workflows.lines().map(WorkFlow::from).collect();
    workflows
}

#[allow(dead_code, unused_variables)]
fn part_two() {
    println!("Part 1");
    let workflows = load_data();
    let mut engine = WorkFlowEngine {
        workflows_by_name: workflows.into_iter().map(|w| (w.name, w)).collect(),
        rejected_parts: Vec::new(),
        accepted_parts: Vec::new(),
    };
    // println!("engine: {engine:?}");
    let part = XMASPart {
        x: (1, 4000),
        m: (1, 4000),
        a: (1, 4000),
        s: (1, 4000),
    };
    engine.process(part);

    let res: Vec<_> = engine
        .accepted_parts
        .iter()
        .map(|p| {
            (p.x.1 - p.x.0 + 1) as i128
                * (p.m.1 - p.m.0 + 1) as i128
                * (p.a.1 - p.a.0 + 1) as i128
                * (p.s.1 - p.s.0 + 1) as i128
        })
        .collect();
    // println!("res: {:?}", res);
    let total: i128 = res.iter().sum();
    println!("Total: {}", total);
}
