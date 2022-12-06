use std::collections::BTreeMap;
use std::collections::HashMap;
use std::hash::Hash;
use std::str;

use sscanf::scanf;

use lib::error::Fail;

#[derive(Debug, Hash, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
struct Guard(u32);

#[derive(Debug, Hash, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
struct Minutes(u32);

impl Minutes {
    fn checked_add(&self, n: Minutes) -> Option<Minutes> {
        self.0.checked_add(n.0).map(Minutes)
    }
    fn checked_sub(&self, n: Minutes) -> Option<Minutes> {
        self.0.checked_sub(n.0).map(Minutes)
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
enum EventType {
    ShiftStart(Guard),
    FallsAsleep,
    Wakes,
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
struct Event {
    kind: EventType,
    minute: Minutes,
}

#[derive(Debug, Clone)]
struct EventHistory {
    current_guard: Option<Guard>,
    asleep_since: Option<Minutes>,
    sleeps: HashMap<Guard, Minutes>,
    asleep_during_minute: HashMap<(Guard, Minutes), usize>,
}

fn parse_event(line: &str, line_number: usize) -> Result<Event, Fail> {
    match scanf!(line, "[{u32}-{u32}-{u32} {u32}:{u32}] {str}") {
        Ok((_year, _month, _day, _hour, minute, text)) => {
            let kind = if text == "falls asleep" {
                EventType::FallsAsleep
            } else if text == "wakes up" {
                EventType::Wakes
            } else {
                match scanf!(text, "Guard #{} begins shift", u32) {
                    Ok(guard) => EventType::ShiftStart(Guard(guard)),
                    Err(e) => {
                        return Err(Fail(format!(
                            "text '{text}' on line {line_number} has wrong format: {e}"
                        )));
                    }
                }
            };
            Ok(Event {
                kind,
                minute: Minutes(minute),
            })
        }
        Err(e) => Err(Fail(format!(
            "line number {line_number} '{line}' has wrong format: {e}"
        ))),
    }
}

impl EventHistory {
    fn new() -> EventHistory {
        EventHistory {
            current_guard: None,
            asleep_since: None,
            sleeps: HashMap::new(),
            asleep_during_minute: HashMap::new(),
        }
    }

    fn biggest_sleeper(&self) -> Result<Guard, Fail> {
        let inverted = invert_map(&self.sleeps);
        match inverted.into_iter().rev().next() {
            Some((_minutes, guard)) => Ok(guard),
            None => Err(Fail("there were no events at all".to_string())),
        }
    }

    fn minute_during_which_sleeps_most(&self, g: &Guard) -> Option<(Minutes, usize)> {
        let mut best: Option<(Minutes, usize)> = None;
        for ((_guard, minute), count) in self
            .asleep_during_minute
            .iter()
            .filter(|((guard, _minute), _count)| *guard == *g)
        {
            if let Some((_, highest_count)) = best {
                if highest_count <= *count {
                    best = Some((*minute, *count));
                }
            } else {
                best = Some((*minute, *count));
            }
        }
        best
    }
}

impl TryFrom<&[Event]> for EventHistory {
    type Error = Fail;
    fn try_from(events: &[Event]) -> Result<EventHistory, Fail> {
        events.iter().try_fold(EventHistory::new(), update_history)
    }
}

fn update_history(mut history: EventHistory, event: &Event) -> Result<EventHistory, Fail> {
    //dbg!(&history);
    //dbg!(&event);
    match event.kind {
        EventType::ShiftStart(guard) => {
            if history.asleep_since.is_some() {
                Err(Fail(
                    "next shift begins but previous guard is still asleep".to_string(),
                ))
            } else {
                Ok(EventHistory {
                    current_guard: Some(guard),
                    asleep_since: None,
                    sleeps: history.sleeps,
                    asleep_during_minute: history.asleep_during_minute,
                })
            }
        }
        EventType::Wakes => match (history.current_guard, history.asleep_since) {
            (Some(guard), Some(begin)) => match event.minute.checked_sub(begin) {
                None => Err(Fail("underflow in time subtraction".to_string())),
                Some(n) => {
                    for m in (begin.0)..(event.minute.0) {
                        history
                            .asleep_during_minute
                            .entry((guard, Minutes(m)))
                            .and_modify(|n| *n += 1)
                            .or_insert(1);
                    }

                    let tot = history.sleeps.get(&guard).copied().unwrap_or(Minutes(0));
                    match tot.checked_add(n) {
                        Some(tot) => {
                            history.sleeps.insert(guard, tot);
                            Ok(EventHistory {
                                current_guard: Some(guard),
                                asleep_since: None,
                                sleeps: history.sleeps,
                                asleep_during_minute: history.asleep_during_minute,
                            })
                        }
                        None => Err(Fail("overflow in time addition".to_string())),
                    }
                }
            },
            (_, None) => Err(Fail(
                "input lines out of order (guard woke but was not asleep)".to_string(),
            )),
            (None, _) => Err(Fail(
                "input lines out of order (guard was asleep but was not on duty)".to_string(),
            )),
        },
        EventType::FallsAsleep => {
            if history.asleep_since.is_some() {
                Err(Fail(
                    "fell asleep twice without intervening wake-up".to_string(),
                ))
            } else {
                history.asleep_since = Some(event.minute);
                Ok(history)
            }
        }
    }
}

fn invert_map<K, V>(input: &HashMap<K, V>) -> BTreeMap<V, K>
where
    K: Copy + Hash + Ord + PartialOrd,
    V: Copy + Ord + PartialOrd,
{
    input.iter().map(|(k, v)| (*v, *k)).collect()
}

fn solve_part1(history: &EventHistory) -> u32 {
    match history.biggest_sleeper() {
        Ok(guard) => {
            let (when, _) = history
                .minute_during_which_sleeps_most(&guard)
                .expect("oops");
            when.0 * guard.0
        }
        Err(e) => {
            panic!("{e}");
        }
    }
}

#[test]
fn test_biggest_sleeper() {
    const EXAMPLE: &str = concat!(
        "[1518-11-01 00:00] Guard #10 begins shift\n",
        "[1518-11-01 00:05] falls asleep\n",
        "[1518-11-01 00:25] wakes up\n",
        "[1518-11-01 00:30] falls asleep\n",
        "[1518-11-01 00:55] wakes up\n",
        "[1518-11-01 23:58] Guard #99 begins shift\n",
        "[1518-11-02 00:40] falls asleep\n",
        "[1518-11-02 00:50] wakes up\n",
        "[1518-11-03 00:05] Guard #10 begins shift\n",
        "[1518-11-03 00:24] falls asleep\n",
        "[1518-11-03 00:29] wakes up\n",
        "[1518-11-04 00:02] Guard #99 begins shift\n",
        "[1518-11-04 00:36] falls asleep\n",
        "[1518-11-04 00:46] wakes up\n",
        "[1518-11-05 00:03] Guard #99 begins shift\n",
        "[1518-11-05 00:45] falls asleep\n",
        "[1518-11-05 00:55] wakes up    \n"
    );
    let events = parse_events(EXAMPLE).expect("test input format should be correct");
    let history: EventHistory = EventHistory::try_from(events.as_slice()).unwrap();
    match history.biggest_sleeper() {
        Ok(guard) => {
            assert_eq!(guard, Guard(10));
            assert_eq!(solve_part1(&history), 240);
        }
        Err(e) => {
            panic!("biggest_sleeper failed: {e}");
        }
    }
}

fn parse_events(s: &str) -> Result<Vec<Event>, Fail> {
    let mut lines: Vec<&str> = s.split_terminator('\n').collect();
    lines.sort();
    lines
        .into_iter()
        .map(|s| s.trim())
        .enumerate()
        .map(|(n, line)| parse_event(line, n))
        .collect::<Result<Vec<Event>, _>>()
}

fn main() {
    let input = str::from_utf8(include_bytes!("input.txt"))
        .expect("input file should be correctly encoded");
    let events = parse_events(input).expect("valid input");
    let history = EventHistory::try_from(events.as_slice()).expect("should not fail");

    let solution = solve_part1(&history);
    println!("Day 04 part 1: {}", solution);
    //dbg!(&events);
}
