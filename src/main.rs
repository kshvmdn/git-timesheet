use std::collections::HashMap;
use std::fmt;
use std::process::Command;
extern crate time;

struct Commit {
    author: String,
    tm: time::Tm
}

impl fmt::Debug for Commit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Commit: {{ author: {}, tm: {:?} }}", self.author, self.tm)
    }
}

fn get_log() -> String {
    let output = Command::new("git")
                         .arg("log")
                         .arg("--pretty=%ae %at")
                         .output()
                         .expect("failed to `git log`");
    assert!(output.status.success(), "{}",
        String::from_utf8_lossy(&output.stderr));
    String::from_utf8_lossy(&output.stdout).into_owned()
}

fn parse_commits(lines: Vec<&str>) -> Vec<Commit> {
    let mut commits: Vec<Commit> = Vec::new();
    for line in lines {
        let split = line.split(" ").collect::<Vec<&str>>();
        if split.len() < 2 {
            continue;
        }

        // TODO: Filter by email if provided (as command line arg).
        commits.push(Commit {
            author: String::from(split[0]),
            tm: time::strptime(split[1], "%s").unwrap(),
        })
    }
    commits.reverse();
    commits
}

fn print_summary(commits: &Vec<Commit>) {
    println!("SUMMARY\n-------");
    println!("{:?} commits", commits.len());

    let initial = commits[0].tm;
    let latest = commits[commits.len()-1].tm;
    let difference = latest - initial;

    println!("{:?} day span ({} to {})",
        difference.num_days(),
        time::strftime("%F", &initial).unwrap(),
        time::strftime("%F", &latest).unwrap(),
    );
}

fn print_hrs(commits: &Vec<Commit>) {
    let mut counts: Vec<i32> = vec![0; 24];
    for commit in commits {
        counts[commit.tm.tm_hour as usize] += 1;
    }

    println!("BY HOUR\n-------");
    for (i, count) in counts.iter().enumerate() {
        let ratio = (*count as f32 / commits.len() as f32) * 100.0;
        println!("{:02}: {:>4} {}", i, count, str::repeat("=", ratio as usize));
    }
}

fn print_days(commits: &Vec<Commit>) {
    let mut counts: Vec<i32> = vec![0; 7];
    for commit in commits {
        counts[commit.tm.tm_wday as usize] += 1;
    }

    let days: Vec<&str> = vec!["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];

    println!("BY DAY\n------");
    for (i, count) in counts.iter().enumerate() {
        let ratio = (*count as f32 / commits.len() as f32) * 100.0;
        println!("{}: {:>4} {}", days[i], count, str::repeat("=", ratio as usize));
    }
}

fn print_months(commits: &Vec<Commit>) {
    let mut counts: HashMap<String, i32> = HashMap::new();
    for commit in commits {
        let k = time::strftime("%Y-%m", &commit.tm).unwrap();
        let v = counts.entry(k).or_insert(0);
        *v += 1;
    }

    // We want to output months in order, so we manually sort keys here (stored
    // in arbitrary order in the HashMap).
    let mut keys: Vec<&String> = Vec::new();
    for key in counts.keys() {
        keys.push(key)
    }
    keys.sort();

    println!("BY MONTH / YEAR\n---------------");
    for key in keys {
        let count = counts[key];
        let ratio = (count as f32 / commits.len() as f32) * 100.0;
        println!("{}: {:>4} {}", key, count, str::repeat("=", ratio as usize));
    }
}

fn main() {
    let log = get_log();
    let mut lines = log.split("\n").collect::<Vec<&str>>();

    // Truncate trailing whitespace (since git-log outputs trailing newline).
    if lines.len() > 0 && lines[lines.len()-1] == "" {
        lines.pop();
    }

    let commits = parse_commits(lines);
    print_summary(&commits);
    print!("\n");
    for func in [print_hrs, print_days, print_months].iter() {
        func(&commits);
        print!("\n");
    }
}
