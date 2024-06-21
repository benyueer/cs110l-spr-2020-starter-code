use grid::Grid; // For lcs()
use std::env;
use std::fs::File; // For read_file_lines()
use std::io::{self, BufRead}; // For read_file_lines()
use std::process;

pub mod grid;

/// Reads the file at the supplied path, and returns a vector of strings.
fn read_file_lines(filename: &String) -> Result<Vec<String>, io::Error> {
    let file = File::open(filename)?;

    let mut lines = Vec::new();
    for line in io::BufReader::new(file).lines() {
        lines.push(line?);
    }

    Ok(lines)
}


fn lcs(seq1: &Vec<String>, seq2: &Vec<String>) -> Grid {
    let m = seq1.len();
    let n = seq2.len();
    let mut lcs_table = Grid::new(m + 1, n + 1);

    for i in 1..m + 1 {
        for j in 1..n + 1 {
            if seq1[i - 1] == seq2[j - 1] {
                lcs_table.set(i, j, lcs_table.get(i - 1, j - 1).unwrap() + 1);
            } else {
                lcs_table.set(i, j, std::cmp::max(lcs_table.get(i, j - 1).unwrap(), lcs_table.get(i - 1, j).unwrap()));
            }
        }
    }

    lcs_table
}


fn print_diff(lcs_table: &Grid, lines1: &Vec<String>, lines2: &Vec<String>, mut i: usize, mut j: usize) {
    let mut rets = Vec::new();

    while i > 0 && j > 0 {
        if lines1[i - 1] == lines2[j - 1] {
            rets.insert(0, "  ".to_string() + &lines1[i - 1].clone());
            i -= 1;
            j -= 1;
        } else if lcs_table.get(i, j - 1).unwrap() < lcs_table.get(i - 1, j).unwrap() {
            rets.insert(0, "- ".to_string() + &lines1[i - 1].clone());
            i -= 1;
        } else if lcs_table.get(i, j - 1).unwrap() >= lcs_table.get(i - 1, j).unwrap() {
            rets.insert(0, "+ ".to_string() + &lines2[j - 1].clone());
            j -= 1;
        }
    }

    while i > 0 {
        rets.insert(0, "- ".to_string() + &lines1[i - 1].clone());
        i -= 1;
    }

    while j > 0 {
        rets.insert(0, "+ ".to_string() + &lines2[j - 1].clone());
        j -= 1;
    }

    println!("{}", rets.join("\n"));
}


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Too few arguments.");
        process::exit(1);
    }
    let filename1 = &args[1];
    let filename2 = &args[2];

    let lines1_result = read_file_lines(&filename1).unwrap();
    let lines2_result = read_file_lines(&filename2).unwrap();

    let lcs_table = lcs(&lines1_result, &lines2_result);

    print_diff(&lcs_table, &lines1_result, &lines2_result, lines1_result.len(), lines2_result.len());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_read_file_lines() {
        let lines_result = read_file_lines(&String::from("handout-a.txt"));
        assert!(lines_result.is_ok());
        let lines = lines_result.unwrap();
        assert_eq!(lines.len(), 8);
        assert_eq!(
            lines[0],
            "This week's exercises will continue easing you into Rust and will feature some"
        );
    }

    #[test]
    fn test_lcs() {
        let mut expected = Grid::new(5, 4);
        expected.set(1, 1, 1).unwrap();
        expected.set(1, 2, 1).unwrap();
        expected.set(1, 3, 1).unwrap();
        expected.set(2, 1, 1).unwrap();
        expected.set(2, 2, 1).unwrap();
        expected.set(2, 3, 2).unwrap();
        expected.set(3, 1, 1).unwrap();
        expected.set(3, 2, 1).unwrap();
        expected.set(3, 3, 2).unwrap();
        expected.set(4, 1, 1).unwrap();
        expected.set(4, 2, 2).unwrap();
        expected.set(4, 3, 2).unwrap();

        println!("Expected:");
        expected.display();
        let result = lcs(
            &"abcdefgh".chars().map(|c| c.to_string()).collect(),
            &"batdefyyh".chars().map(|c| c.to_string()).collect(),
        );
        println!("Got:");
        result.display();
        assert_eq!(result.size(), expected.size());
        for row in 0..expected.size().0 {
            for col in 0..expected.size().1 {
                assert_eq!(result.get(row, col), expected.get(row, col));
            }
        }
    }
}
