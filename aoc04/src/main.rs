/// Two adjacent are the same

fn adjacent_rule(candidate: String) -> bool {
    let mut prev = candidate.chars().next().unwrap();
    for c in candidate[1..].chars() {
        if prev == c {
            return true;
        }
        prev = c;
    }
    false
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
    }

    fn increase_rule() {
        assert!(super::increase_rule("123345".to_string()));
        assert!(!super::increase_rule("123234".to_string()));
    }
}