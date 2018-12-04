extern crate regex;
#[macro_use]
extern crate lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::error::Error;
use std::io;
use std::io::Read;
use std::str::FromStr;

type Minute = usize;
type GuardId = usize;

fn main() {
    if let Err(e) = run() {
        println!("FATAL ERROR: {}", e)
    }
}

fn run() -> Result<(), Box<Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    // Sort input before processing as needed guarantee
    // for ignoring Guard Changes at 23:xx
    let mut sorted = input.lines().collect::<Vec<_>>();
    sorted.sort_unstable();

    // parse events and play through them
    let events: Result<Vec<Event>, _> = sorted.iter().map(|line| FromStr::from_str(line)).collect();
    let mut world = World::new();
    world.process_sorted_events(events?.as_slice())?;

    // part 1
    let guards = &world.guards;
    let (sleepiest_guard_id, amount_slept) = world
        .get_sleepiest_guard()
        .ok_or("Could not determine sleepiest guard")?;

    println!(
        "The sleepiest guard is {}, and sleeps for {} minutes",
        sleepiest_guard_id, amount_slept
    );

    if let Some((sleepiest_minute, count)) = guards
        .get(&sleepiest_guard_id)
        .expect("Sleepiest guard must be in guards")
        .sleepiest_minute()
    {
        println!(
            "The sleepiest minute for the sleepiest guard {} is minute {} slept {} times. (GuardId x Minute = {} )",
            sleepiest_guard_id, sleepiest_minute, count, sleepiest_guard_id * sleepiest_minute
        );
    } else {
        return Err(From::from(
            "Could not determine sleepiest minute for sleepiest guard",
        ));
    }

    // part 2
    if let Some((most_sleepy_guard, most_sleepy_minute, most_sleepy_count)) =
        world.get_most_slept_minute_by_single_guard()
    {
        println!(
            "Guard {}, slept on Minute {}, for {} times, more than any other guard on any other minute (GuardId x Minute = {})",
            most_sleepy_guard, most_sleepy_minute, most_sleepy_count, most_sleepy_guard * most_sleepy_minute
        );
    } else {
        return Err(From::from(
            "Could not determine most slept minute by single guard",
        ));
    }

    Ok(())
}

struct World {
    guards: HashMap<GuardId, Guard>,
    current_guard: Option<GuardId>,
    asleep_since: Option<Minute>,
}

impl World {
    fn new() -> Self {
        World {
            guards: HashMap::new(),
            current_guard: None,
            asleep_since: None,
        }
    }

    fn step_event(&mut self, event: &Event) -> Result<(), Box<Error>> {
        match event {
            Event::SwitchOver(id) => {
                //                println!("New Guard: {}", id);
                self.current_guard = Some(*id);
            }
            Event::SleepIn(minute) => {
                //                println!("Sleep in at: {}", minute);
                if self.asleep_since.is_some() {
                    return Err(From::from("Can't sleep in: Guard is asleep."));
                }
                self.asleep_since = Some(*minute);
            }
            Event::WakeUp(to_minute) => {
                //                println!("Wake up at: {}", to_minute);
                if let Some(from_minute) = self.asleep_since {
                    if let Some(guard) = self.current_guard {
                        for min in from_minute..*to_minute {
                            self.guards
                                .entry(guard)
                                .and_modify(|count| count.asleep_per_minute[min] += 1)
                                .or_insert_with(|| {
                                    let mut guard = Guard::new(guard);
                                    guard.asleep_per_minute[min] += 1;
                                    guard
                                });
                        }
                    } else {
                        return Err(From::from("No guard set, can't set sleep duration"));
                    }
                } else {
                    return Err(From::from("Can't wake up: Guard is already awake."));
                }

                self.asleep_since = None;
            }
        }
        Ok(())
    }

    fn process_sorted_events(&mut self, events: &[Event]) -> Result<(), Box<Error>> {
        for event in events {
            self.step_event(event)?;
        }
        Ok(())
    }

    fn get_sleepiest_guard(&self) -> Option<(GuardId, usize)> {
        self.guards
            .iter()
            .map(|(id, guard)| (id.to_owned(), guard.asleep_per_minute.iter().sum()))
            .max_by_key(|x: &(usize, usize)| x.1)
    }

    // FIXME improve naming
    fn get_most_slept_minute_by_single_guard(&self) -> Option<(GuardId, Minute, usize)> {
        self.guards
            .iter()
            .filter_map(|(id, guard)| {
                if let Some((minute, max_asleep)) = guard
                    .asleep_per_minute
                    .iter()
                    .cloned()
                    .enumerate()
                    .max_by_key(|x| x.1)
                {
                    Some((*id, minute, max_asleep))
                } else {
                    None
                }
            }).max_by_key(|x| x.2)
    }
}

#[derive(Debug, Clone)]
struct Guard {
    id: GuardId,
    asleep_per_minute: Vec<usize>,
}

impl Guard {
    fn new(id: GuardId) -> Self {
        Guard {
            id,
            asleep_per_minute: vec![0; 60],
        }
    }

    fn sleepiest_minute(&self) -> Option<(Minute, usize)> {
        self.asleep_per_minute
            .iter()
            .cloned()
            .enumerate()
            .max_by_key(|x| x.1)
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Event {
    // Minute needed?
    // Not if we are guaranteed sorted input an sleep times between 00:00 and 00:59
    SwitchOver(GuardId),
    SleepIn(Minute),
    WakeUp(Minute),
}

lazy_static! {
    static ref FROM_STR_RX: Regex =
        Regex::new(r##"^\[[^:]+:([0-9]{2})\]\s+(Guard #([0-9]+)|.*)"##).unwrap();
}

impl FromStr for Event {
    type Err = Box<Error>;

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        let caps = FROM_STR_RX.captures(s).ok_or("Could not match Event")?;

        if let Some(guard_id) = caps.get(3) {
            Ok(Event::SwitchOver(guard_id.as_str().parse()?))
        } else if let Some(event) = caps.get(2) {
            let minute: Minute = if let Some(minute) = caps.get(1) {
                minute.as_str().parse()?
            } else {
                return Err(From::from("First match must exist"));
            };

            match event.as_str() {
                "falls asleep" => Ok(Event::SleepIn(minute)),
                "wakes up" => Ok(Event::WakeUp(minute)),
                _ => Err(From::from(
                    "Second match must exist and correspond to an action",
                )),
            }
        } else {
            Err(From::from("Can not have less than two matches!"))
        }
    }
}

// Format:
// [1518-11-01 00:00] Guard #10 begins shift
// [1518-11-01 00:05] falls asleep
// [1518-11-01 00:25] wakes up

#[test]
fn parse_event() {
    let guard = "[1518-11-01 00:00] Guard #10 begins shift";
    let asleep = "[1518-11-01 00:05] falls asleep";
    let wakeup = "[1518-11-01 00:25] wakes up";

    let guard_event = FromStr::from_str(guard).unwrap();
    let asleep_event = FromStr::from_str(asleep).unwrap();
    let wakeup_event = FromStr::from_str(wakeup).unwrap();

    assert_eq!(Event::SwitchOver(10), guard_event);
    assert_eq!(Event::SleepIn(05), asleep_event);
    assert_eq!(Event::WakeUp(25), wakeup_event);
}

static _TEST_INPUT: &str = "[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up";

#[test]
fn test_full_example() {
    let events = _TEST_INPUT
        .lines()
        .map(FromStr::from_str)
        .collect::<Result<Vec<Event>, _>>()
        .unwrap();
    let mut world = World::new();

    world.process_sorted_events(&events).unwrap();
    let (sleepiest_guard_id, amount_slept) = world.get_sleepiest_guard().unwrap();
    let (sleepiest_minute, count) = world
        .guards
        .get(&sleepiest_guard_id)
        .unwrap()
        .sleepiest_minute()
        .unwrap();

    assert_eq!(sleepiest_guard_id, 10);
    assert_eq!(amount_slept, 50);
    assert_eq!(sleepiest_minute, 24);
    assert_eq!(count, 2);
}
