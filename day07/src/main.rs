use std::collections::HashMap;
use std::error::Error;
use std::io;
use std::io::Read;
use std::str::FromStr;

fn main() {
    if let Err(e) = run() {
        println!("FATAL ERROR: {}", e)
    }
}

fn run() -> Result<(), Box<Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let dependencies: Result<Vec<Dependency>, _> =
        input.lines().map(|m| FromStr::from_str(m)).collect();
    let nodes: Nodes = From::from(dependencies?.as_slice());

    let correct_sequence = nodes.resolve();
    println!("Sequential steps : '{}'", correct_sequence);

    let (par_sequence, duration) = nodes.par_resolve(5, 60).unwrap();
    println!("Parallel steps {} last {} seconds.", par_sequence, duration);
    Ok(())
}

#[derive(Debug, Clone, Eq, Hash, PartialOrd, PartialEq)]
struct Dependency {
    by: Node,
    needed: Node,
}

type Node = char;
type Dependencies = HashMap<Node, Vec<Node>>;
type Worker = Option<(char, usize)>;
struct Nodes(Dependencies);

const USIZE_OFFSET: usize = 64;

impl From<&[Dependency]> for Nodes {
    fn from(dependencies: &[Dependency]) -> Self {
        let deps = dependencies
            .iter()
            .fold(HashMap::new(), |mut acc: Dependencies, item| {
                let Dependency { needed, by } = item;
                // Add Dependency
                acc.entry(*by)
                    .and_modify(|deps| deps.push(*needed))
                    .or_insert(vec![*needed]);
                // Add Other node to Nodes
                acc.entry(*needed).or_insert(Vec::new());
                acc
            });
        Nodes(deps)
    }
}

impl Nodes {
    fn get_root_steps(&self) -> Vec<Node> {
        self.0
            .iter()
            .filter_map(|d| if d.1.is_empty() { Some(*d.0) } else { None })
            .collect()
    }

    fn resolve(&self) -> String {
        let mut dependencies = self.0.clone();
        let mut candidates = self.get_root_steps();
        let mut sequence = String::new();

        while !candidates.is_empty() {
            // Inverse alphabetical order allows to pop() next candidate
            candidates.sort_by(|a, b| b.cmp(a));

            // last candidate will be solved in this round
            let current = candidates.pop().expect("Candidates is never empty here");
            sequence.push(current);

            // For nodes with not yet satisfied dependencies
            for (step, step_dependencies) in dependencies.iter_mut().filter(|d| !d.1.is_empty()) {
                // Remove all occurrences of the current solved step from our dependencies
                while let Some(idx) = step_dependencies.iter().position(|c| *c == current) {
                    step_dependencies.remove(idx);
                }
                // No dependencies for current node left, so it's ready for work
                if step_dependencies.is_empty() {
                    candidates.push(*step);
                }
            }
        }
        sequence
    }

    fn par_resolve(
        &self,
        workers: usize,
        base_duration: usize,
    ) -> Result<(String, usize), Box<Error>> {
        if workers < 1 {
            return Err(From::from("Need at least one worker to complete project"));
        }

        let mut dependencies = self.0.clone();
        let mut candidates = self.get_root_steps();

        // Inverse alphabetical order allows to pop() next candidate
        candidates.sort_by(|a, b| b.cmp(a));

        let mut epoch = 0;
        let mut workers: Vec<Worker> = vec![None; workers];
        let mut sequence = String::new();

        loop {
            for worker in &mut workers.iter_mut() {
                if let Some(busy_worker) = worker {
                    let (current, time_left) = busy_worker;
                    *time_left -= 1;

                    // The current step is completed during this iteration
                    if *time_left == 0 {
                        // Remove the dependency of the completed step from all nodes depending on it
                        for deps in dependencies.iter_mut().filter(|d| !d.1.is_empty()) {
                            while let Some(idx) = deps.1.iter().position(|c| *c == *current) {
                                deps.1.remove(idx);
                            }
                            // If this was the last dependency for this specific node,
                            // it becomes eligible to be completed
                            if deps.1.is_empty() {
                                candidates.push(*deps.0);
                                // Inverse alphabetical order allows to pop() next candidate
                                candidates.sort_by(|a, b| b.cmp(a));
                            }
                        }

                        sequence.push(*current);
                        *worker = None;
                    }
                }
            }
            // All workers must have finished their work for this step,
            // before new work is assigned to workers without work.
            for worker in &mut workers {
                if worker.is_none() {
                    if let Some(next) = candidates.pop() {
                        let duration = next as usize + base_duration - USIZE_OFFSET;
                        *worker = Some((next, duration))
                    }
                }
            }

            // If solved, avoid advancing and counting 1 round too many
            if workers.iter().any(Option::is_some)
                || dependencies.iter().any(|deps| !deps.1.is_empty())
            {
                epoch += 1;
            } else {
                break;
            }
        }
        Ok((sequence, epoch))
    }
}

impl FromStr for Dependency {
    type Err = Box<Error>;

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        let dependency = s
            .replace("Step", "")
            .replace("must be finished before step", "")
            .replace("can begin.", "");
        let needs_what: Result<Vec<Node>, Self::Err> = dependency
            .split_whitespace()
            .map(|a| {
                let node = a.trim();
                if node.len() == 1 {
                    node.chars()
                        .next()
                        .map(|c| c.to_ascii_uppercase())
                        .ok_or(From::from("Len was 1 but no char was contained"))
                } else {
                    Err(From::from("Error while parsing Nodes from instructions."))
                }
            })
            .collect();

        let needs_what = needs_what?;
        if needs_what.len() == 2 {
            Ok(Self {
                needed: needs_what[0],
                by: needs_what[1],
            })
        } else {
            Err(From::from("Could not parse Dependency from input"))
        }
    }
}
#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_parse_deps() {
        let dependencies: Result<Vec<Dependency>, _> =
            INPUT.iter().map(|s| FromStr::from_str(s)).collect();

        assert!(dependencies.is_ok());
        assert_eq!(NODES, dependencies.unwrap().as_slice())
    }

    #[test]
    fn test_example_root() {
        let dependencies: Result<Vec<Dependency>, _> =
            INPUT.iter().map(|s| FromStr::from_str(s)).collect();

        let dependencies = dependencies.unwrap();
        let nodes: Nodes = From::from(dependencies.as_slice());

        let root = nodes.get_root_steps();
        assert_eq!(root.len(), 1);
        assert_eq!(root[0], 'C');
    }

    #[test]
    fn test_example_resolve() {
        let dependencies: Result<Vec<Dependency>, _> =
            INPUT.iter().map(|s| FromStr::from_str(s)).collect();

        let dependencies = dependencies.unwrap();
        let nodes: Nodes = From::from(dependencies.as_slice());

        let sequence = nodes.resolve();

        assert_eq!("CABDFE", sequence);
    }

    #[test]
    fn test_example_par_resolve() {
        let dependencies: Result<Vec<Dependency>, _> =
            INPUT.iter().map(|s| FromStr::from_str(s)).collect();

        let dependencies = dependencies.unwrap();
        let nodes: Nodes = From::from(dependencies.as_slice());

        let (sequence, time) = nodes.par_resolve(2, 0).unwrap();

        assert_eq!("CABFDE", sequence);
        assert_eq!(15, time);
    }

    static INPUT: &[&str] = &[
        "Step C must be finished before step A can begin.",
        "Step C must be finished before step F can begin.",
        "Step A must be finished before step B can begin.",
        "Step A must be finished before step D can begin.",
        "Step B must be finished before step E can begin.",
        "Step D must be finished before step E can begin.",
        "Step F must be finished before step E can begin.",
    ];

    static NODES: &[Dependency] = &[
        Dependency {
            by: 'A',
            needed: 'C',
        },
        Dependency {
            by: 'F',
            needed: 'C',
        },
        Dependency {
            by: 'B',
            needed: 'A',
        },
        Dependency {
            by: 'D',
            needed: 'A',
        },
        Dependency {
            by: 'E',
            needed: 'B',
        },
        Dependency {
            by: 'E',
            needed: 'D',
        },
        Dependency {
            by: 'E',
            needed: 'F',
        },
    ];
}
