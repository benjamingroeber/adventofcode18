fn main() {
    if let Err(e) = run() {
        println!("FATAL ERROR: {}", e)
    }
}

fn run() -> Result<(), Box<Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let numbers : Result<Vec<usize>,_> = input.split_whitespace().map(|number| number.parse()).collect();

    let root = parse_nodes(numbers?.as_slice())?;
    println!("The sum of all metadata is: {}", root.metadata_sum());
    println!("The value of the root node is: {}", root.value());

    Ok(())
}

use std::error::Error;
use std::io;
use std::io::Read;

#[derive(Clone, Debug)]
struct Node {
    id: usize,
    child_count: usize,
    children: Vec<Node>,
    metadata_count: usize,
    metadata: Vec<usize>
}

impl Node {
    fn new(id: usize, child_count: usize, metadata_count: usize) -> Self {
        Node {
            id,
            child_count,
            children: Vec::new(),
            metadata_count,
            metadata: Vec::new()
        }
    }

    fn metadata_sum(&self) -> usize {
        let child_sum : usize = self.children.iter().map(|c|c.metadata_sum()).sum();
        let own: usize = self.metadata.iter().sum();
        child_sum + own
    }

    fn value(&self) -> usize {
        // The value of Nodes with no children equals the sum of the metadata
        if self.child_count == 0 {
            self.metadata.iter().sum()

        // The value of Nodes with children equals to The Sum of the child nodes values,
        // where child nodes are referred by the metadata entries used as index
        } else {
            self.metadata.iter().map(|data| {
                // 1 refers to first entry, and so on
                let idx = data - 1;
                // ignore non existing referenced children
                self.children.get(idx).map(|child|child.value()).unwrap_or(0)
            }).sum()
        }
    }

}

enum State {
    NewNode,
    ReadMetadata,
    AddChild,
    Done
}

// NewNode -> ReadMetadata: if no Children, read nodes metadata immediately
// NewNode -> NewNode:      push next child onto stack

// ReadMetadata -> AddChild: Reading Metadata completes a Node

// AddChild -> Done:         if last child of last stack item added, we're done
// AddChild -> ReadMetadata: if last child of parent added, read parent metadata
// AddChild -> NewNode:      else push next child node onto stack

fn parse_nodes(input: &[usize]) -> Result<Node,Box<Error>>{
    let mut node_count = 0;
    let mut state : State = State::NewNode;
    let mut next = input.iter().peekable();
    let mut stack = Vec::new();
    loop {
        match state {
            State::NewNode => {
                if let Some(child_count) = next.next() {
                    if let Some(metadata_count) = next.next() {
                        node_count += 1;
                        stack.push(Node::new(node_count, *child_count, *metadata_count));

                        if *child_count == 0 {
                            state = State::ReadMetadata
                        } else {
                            state = State::NewNode
                        }
                    } else {
                        return Err(From::from("Could not read metadata count for new Node."))
                    }
                } else {
                    return Err(From::from("Could not read metadata count for new Node."))
                }
            },
            State::ReadMetadata => {
                if let Some(last) = stack.last_mut() {
                    while last.metadata.len() < last.metadata_count  {
                        if let Some(data) = next.next() {
                            last.metadata.push(*data);
                        } else {
                            return Err(From::from("Could not read enough Metadata"))
                        }
                    }
                    state = State::AddChild
                } else {
                    return Err(From::from("Can't read metadata for nothing"))
                }
            },
            State::AddChild => {
                if stack.len() > 1 {
                    let child = stack.pop().unwrap();
                    let parent = stack.last_mut().unwrap();
                    parent.children.push(child);

                    if parent.children.len() == parent.child_count {
                        state = State::ReadMetadata
                    } else {
                        state = State::NewNode
                    }
                } else {
                    state = State::Done
                }
            }
            State::Done => break,
        }
    }
    if stack.len() == 1 {
        Ok(stack.pop().unwrap())
    } else {
        Err(From::from("More than 1 root left."))
    }
}

#[test]
fn test_example() {
    let input = &[2,3,0,3,10,11,12,1,1,0,1,99,2,1,1,2];
    println!("{:?}", input);
    
    let root = parse_nodes(input);
    println!("{:?}", root);

    let sum = root.unwrap().metadata_sum();
    println!("SUM: {}", sum)
}
