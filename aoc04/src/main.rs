/// Two adjacent are the same
/// But not part of a larger match
fn adjacent_rule(candidate: String) -> bool {
    let mut prev = candidate.chars().next().unwrap();
    let mut preliminary = false;
    let mut needs_diff = false;
    for c in candidate[1..].chars() {
        // println!("{}, {}, prel: {}, needs_diff: {}", prev, c, preliminary, needs_diff);
        if needs_diff && prev == c {
            prev = c;
            continue;
        } else if needs_diff {
            needs_diff = false;
        }
        if preliminary && prev != c {
            return true;
        } else if preliminary {
            preliminary = false;
            needs_diff = true;
            prev = c;
            continue;
        }
        if prev == c {
            preliminary = true;
        }
        prev = c;
    }
    if preliminary {
        true
    } else {
        false
    }
}

fn increase_rule(candidate: String) -> bool {
    let mut prev = candidate.chars().next().unwrap();
    for c in candidate[1..].chars() {
        if c < prev {
            return false
        }
        prev = c;
    }
    true
}

fn main() {
    let min = 172930;
    let max = 683082;
    println!("Hello, world!");
    println!("{}", adjacent_rule("122345".to_string()));
    let mut matching = 0;
    for x in min..max {
        if adjacent_rule(x.to_string()) && increase_rule(x.to_string()) {
            println!("Match: {}", x);
            matching += 1;
        }
    }
    println!("{} passwords match", matching);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    fn adjacent_rule() {
        assert!(super::adjacent_rule("122345".to_string()));
        assert!(super::adjacent_rule("111144".to_string()));
        assert!(!super::adjacent_rule("111111".to_string()));
        assert!(!super::adjacent_rule("111444".to_string()));
        assert!(!super::adjacent_rule("111123".to_string()));
    }

    fn increase_rule() {
        assert!(super::increase_rule("123345".to_string()));
        assert!(!super::increase_rule("123234".to_string()));
    }
}