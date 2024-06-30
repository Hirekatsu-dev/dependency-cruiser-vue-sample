use itertools::Itertools;
use proconio::input;
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet, VecDeque},
    str::FromStr,
};

#[derive(Serialize, Deserialize)]
struct Dependency {
    resolved: String,
}

#[derive(Serialize, Deserialize)]
struct Module {
    source: String,
    dependencies: Vec<Dependency>,
    dependents: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct Modules {
    modules: Vec<Module>,
}

#[derive(Serialize, Deserialize)]
struct Task {
    name: String,
    difficulty: f64,
    depends: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct TaskGraph {
    tasks: Vec<Task>,
}

fn main() {
    let dependency_str = include_str!("dependencygraph.json");
    let dependency_value = serde_json::Value::from_str(&dependency_str).expect("invalid json.");
    let modules = serde_json::from_value::<Modules>(dependency_value).expect("invalid json.");
    let modules = modules.modules;

    input! {
        targets: [String],
        diffculty_target: f64,
        difficulty_dependant: f64,
    }

    let index_of_source = {
        let mut res = HashMap::new();
        for (index, module) in modules.iter().enumerate() {
            res.insert(&module.source, index);
        }

        res
    };

    let mut queue = VecDeque::new();
    let mut dependent_source_index = HashSet::new();

    for target in &targets {
        if let Some(index) = index_of_source.get(&target) {
            dependent_source_index.insert(index);
            queue.push_back(index);
        }
    }

    dbg!(&queue);

    while let Some(&index) = queue.pop_front() {
        for next in &modules[index].dependents {
            if let Some(next_index) = index_of_source.get(&next) {
                if !dependent_source_index.contains(&next_index) {
                    dependent_source_index.insert(next_index);
                    queue.push_back(next_index);
                }
            }
        }
    }

    let tasks = {
        let mut res = vec![];

        for &index in dependent_source_index {
            let name = modules[index].source.to_owned();
            let mut depends = vec![];
            for dependent in &modules[index].dependents {
                if let Some(_) = index_of_source.get(&dependent) {
                    depends.push(dependent.to_owned());
                }
            }
            let difficulty = if targets.iter().any(|target| target == &name) {
                diffculty_target
            } else {
                difficulty_dependant
            };

            res.push(Task {
                name,
                difficulty,
                depends,
            });
        }

        res
    };

    let taskgraph = TaskGraph { tasks };

    println!("{}", serde_json::to_string(&taskgraph).unwrap());
}
