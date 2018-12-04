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
    println!("{:?}", sorted);

    let events: Result<Vec<Event>, _> = sorted.iter().map(|line| FromStr::from_str(line)).collect();

    let mut guards: HashMap<GuardId, Guard> = HashMap::new();
    let mut guard = None;
    let mut asleep = false;
    let mut slept_in = 0;
    for event in events? {
        match event {
            Event::SwitchOver(id) => {
                println!("New Guard: {}", id);
                guard = Some(id);
                slept_in = 0;
                asleep = false;
            }
            Event::SleepIn(minute) => {
                println!("Sleep in at: {}", minute);
                if asleep {
                    return Err(From::from("Can't sleep in: Guard is asleep."));
                }
                slept_in = minute;
                asleep = true
            }
            Event::WakeUp(minute) => {
                println!("Wake up at: {}", minute);
                if !asleep {
                    return Err(From::from("Can't wake up: Guard is already awake."));
                }
                if let Some(guard) = guard {
                    for min in slept_in..minute {
                        guards
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
                asleep = false;
            }
        }
    }

    let (sleepiest_guard_id, amount_slept) = guards
        .iter()
        .map(|(id, guard)| (id.to_owned(), guard.asleep_per_minute.iter().sum()))
        .max_by_key(|x: &(usize, usize)| x.1)
        .ok_or("Could not declare sleepiest guard")?;

    println!(
        "The sleepiest guard is {}, and sleeps for {} minutes",
        sleepiest_guard_id, amount_slept
    );
    let sleepiest_guard = guards
        .get(&sleepiest_guard_id)
        .expect("Sleepiest guard must be in guards");
    let (sleepiest_minute, count) = sleepiest_guard
        .asleep_per_minute
        .iter()
        .enumerate()
        .max_by_key(|x| x.1)
        .ok_or("Could not determine sleepiest minute for sleepiest guard")?;
    println!(
        "The sleepiest minute for the sleepiest guard is minute {} slept {} times.",
        sleepiest_minute, count
    );
    println!(
        "Sleepiest Guard ID times sleepiest minute is {}",
        sleepiest_guard_id * sleepiest_minute
    );

    // ;) Naming, right?
    let (most_sleepy_guard, most_sleepy_minute, most_sleepy_count) = guards
        .iter()
        .map(|(id, guard)| {
            let (minute, max_asleep) = guard
                .asleep_per_minute
                .iter()
                .enumerate()
                .max_by_key(|x| x.1)
                .expect("By part 1 there MUST be a maximum minute");
            (id, minute, max_asleep)
        }).max_by_key(|x| x.2).expect("By part 1 there MUST be a maximum among guards");
    println!(
        "Guard {}, slept on Minute {}, for {} times, more than any other guard on any other minute",
        most_sleepy_guard, most_sleepy_minute, most_sleepy_count
    );
    println!(
        "Most sleepy minute among guards times sleeping guard is {}", most_sleepy_guard * most_sleepy_minute,
    );
    Ok(())
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
        println!("String: {}", s);
        //        caps.iter().for_each(|c|println!("{:?}", c.unwrap().as_str()));
        if let Some(guard_id) = caps.get(3) {
            let guard_id = guard_id.as_str().parse()?;
            Ok(Event::SwitchOver(guard_id))
        } else if let Some(event) = caps.get(2) {
            let minute: Minute = if let Some(minute) = caps.get(1) {
                println!("MINUTE: {}", minute.as_str());
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
