use std::error::Error;
use std::io;
use std::io::Read;

fn main() {
    if let Err(e) = run() {
        println!("FATAL ERROR: {}", e)
    }
}

fn run() -> Result<(), Box<Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let numbers: Result<Vec<usize>, _> = input
        .split_whitespace()
        .map(|number| number.parse())
        .collect();

    let root = parse_nodes(numbers?.as_slice())?;
    println!("The sum of all metadata is: {}", root.metadata_sum());
    println!("The value of the root node is: {}", root.value());

    Ok(())
}

#[derive(Clone, Debug)]
struct Node {
    child_count: usize,
    children: Vec<Node>,
    metadata_count: usize,
    metadata: Vec<usize>,
}

impl Node {
    fn new(child_count: usize, metadata_count: usize) -> Self {
        Node {
            child_count,
            children: Vec::new(),
            metadata_count,
            metadata: Vec::new(),
        }
    }

    fn metadata_sum(&self) -> usize {
        // Sum own metadata with all child node metadata sum
        let own: usize = self.metadata.iter().sum();
        let child_sum: usize = self.children.iter().map(|c| c.metadata_sum()).sum();
        child_sum + own
    }

    fn value(&self) -> usize {
        // The value of Nodes with no children equals the sum of the metadata
        if self.child_count == 0 {
            self.metadata.iter().sum()

        // The value of Nodes with children equals to The Sum of the child nodes values,
        // where child nodes are referred by the metadata entries used as index
        } else {
            self.metadata
                .iter()
                .map(|data| {
                    // 1 refers to first entry, ...
                    let idx = data - 1;
                    // ignore non existing referenced children
                    self.children
                        .get(idx)
                        .map(|child| child.value())
                        .unwrap_or(0)
                })
                .sum()
        }
    }
}

enum State {
    NewNode,
    ReadMetadata,
    AddChild,
    Done,
}

// NewNode -> ReadMetadata: if no Children, read nodes metadata immediately
// NewNode -> NewNode:      push next child onto stack

// ReadMetadata -> AddChild: Reading Metadata completes a Node
// ReadMetadata -> Done:     if we complete the outermost Node, we're done

// AddChild -> ReadMetadata: if last child of parent added, read parent metadata
// AddChild -> NewNode:      else push next child node onto stack

fn parse_nodes(input: &[usize]) -> Result<Node, Box<Error>> {
    let mut state: State = State::NewNode;
    let mut cursor = input.iter();
    let mut stack = Vec::new();
    loop {
        match state {
            State::NewNode => {
                // We must be able to read at least both count values.
                if let Some(child_count) = cursor.next() {
                    if let Some(metadata_count) = cursor.next() {
                        let new_node = Node::new(*child_count, *metadata_count);
                        stack.push(new_node);

                        // Keep pushing new nodes unless we encounter a leaf node
                        if *child_count == 0 {
                            state = State::ReadMetadata
                        } else {
                            state = State::NewNode
                        }
                    } else {
                        return Err(From::from("Could not read metadata count for new Node."));
                    }
                } else {
                    return Err(From::from("Could not read metadata count for new Node."));
                }
            }
            State::ReadMetadata => {
                // Read n items as metadata for the current stack frame
                if let Some(last) = stack.last_mut() {
                    while last.metadata.len() < last.metadata_count {
                        if let Some(data) = cursor.next() {
                            last.metadata.push(*data);
                        } else {
                            return Err(From::from("Could not read enough Metadata"));
                        }
                    }
                    // Reading metadata completes a Node
                    // if this was the last stack frame, we're done
                    // otherwise we must add the current frame to its parent
                    if stack.len() > 1 {
                        state = State::AddChild
                    } else {
                        state = State::Done
                    }
                } else {
                    return Err(From::from("Can't read metadata for nothing"));
                }
            }
            State::AddChild => {
                // We're guaranteed the the last two items on the stack must be parent & child
                if let Some(child) = stack.pop() {
                    if let Some(parent) = stack.last_mut() {
                        // The previous top of the stack becomes child of the new top of the stack
                        parent.children.push(child);

                        // If we complete the children of the current stack frame
                        // we can start reading metadata for it
                        // otherwise we need to keep adding child nodes
                        if parent.children.len() == parent.child_count {
                            state = State::ReadMetadata
                        } else {
                            state = State::NewNode
                        }
                    } else {
                        return Err(From::from("Could not get parent node."));
                    }
                } else {
                    return Err(From::from("Could not get child node."));
                }
            }
            State::Done => {
                return if stack.len() == 1 {
                    Ok(stack.pop().unwrap())
                } else {
                    Err(From::from("More than 1 root left."))
                }
            }
        }
    }
}

#[test]
fn test_example() {
    let input = &[2, 3, 0, 3, 10, 11, 12, 1, 1, 0, 1, 99, 2, 1, 1, 2];

    let root = parse_nodes(input).unwrap();

    let sum = root.metadata_sum();
    assert_eq!(sum, 138);
}

#[test]
fn test_example_value() {
    let input = &[2, 3, 0, 3, 10, 11, 12, 1, 1, 0, 1, 99, 2, 1, 1, 2];

    let root = parse_nodes(input).unwrap();

    let value = root.value();
    assert_eq!(value, 66)
}
