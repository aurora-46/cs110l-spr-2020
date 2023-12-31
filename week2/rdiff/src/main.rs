use grid::Grid; // For lcs()
use std::env;
use std::fs::File; // For read_file_lines()
use std::io::{self, BufRead}; // For read_file_lines()
use std::process;

pub mod grid;

/// Reads the file at the supplied path, and returns a vector of strings.
//#[allow(unused)] // TODO: delete this line when you implement this function
fn read_file_lines(filename: &String) -> Result<Vec<String>, io::Error> {
    //unimplemented!();
    // Be sure to delete the #[allow(unused)] line above
    let file = File::open(filename)?;
    let mut ans=Vec::new();
    for line in io::BufReader::new(file).lines() {
        let line_str = line?;
        ans.push(line_str);
        // do something with line_str
    }
    return Ok(ans);
}

//#[allow(unused)] // TODO: delete this line when you implement this function
fn lcs(seq1: &Vec<String>, seq2: &Vec<String>) -> Grid {
    // Note: Feel free to use unwrap() in this code, as long as you're basically certain it'll
    // never happen. Conceptually, unwrap() is justified here, because there's not really any error
    // condition you're watching out for (i.e. as long as your code is written correctly, nothing
    // external can go wrong that we would want to handle in higher-level functions). The unwrap()
    // calls act like having asserts in C code, i.e. as guards against programming error.
    //unimplemented!();
    // Be sure to delete the #[allow(unused)] line above
    let m = seq1.len();
    let n =seq2.len();
    let mut c = Grid::new(m+1, n+1);
// here, the .., like in Rust, is inclusive on the left bound and exclusive on
// the right bound
    for i in 0..m+1{
        c.set(i, 0, 0).unwrap();
    }
    for j in 0..n+1{
        c.set(0, j, 0).unwrap();
    }
    for i in 0..m{
        for j in 0..n{
            if seq1[i]==seq2[j]{
                let val =match c.get(i, j){
                    Some(k)=>k+1,
                    None=>0,
                };
                c.set(i+1, j+1, val).unwrap();
            }
            else{
                let val1 =match c.get(i+1, j){
                    Some(k)=>k,
                    None=>0,
                };
                let val2 =match c.get(i, j+1){
                    Some(k)=>k,
                    None=>0,
                };
                if val1>val2{
                    c.set(i+1, j+1, val1).unwrap();
                }else {
                    c.set(i+1, j+1, val2).unwrap();
                }
            
            }
        }
    }

return c;
    


}

//#[allow(unused)] // TODO: delete this line when you implement this function
fn print_diff(lcs_table: &Grid, lines1: &Vec<String>, lines2: &Vec<String>, i: usize, j: usize) {
    //unimplemented!();
    // Be sure to delete the #[allow(unused)] line above
    let mut ans=Vec::new();
    let mut x=i;
    let mut y=j;
    loop{
        if x>0 && y>0 && lines1[x-1] ==lines2[y-1]{
            ans.insert(0,format!("  {}",lines1[x-1]));
            x=x-1;
            y=y-1;
        }
        else if y>0 &&(x==0||lcs_table.get(x, y-1)>=lcs_table.get(x-1, y)){
            ans.insert(0,format!("> {}",lines2[y-1]));
            y=y-1;
        }
        else if x>0 &&(y==0||lcs_table.get(x, y-1)<lcs_table.get(x-1, y)){
            ans.insert(0,format!("< {}",lines1[x-1]));
            x=x-1;
        }
        else {
            break;
        }
    }
    for x in ans{
        println!("{}",x);
    }
}

#[allow(unused)] // TODO: delete this line when you implement this function
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Too few arguments.");
        process::exit(1);
    }
    let filename1 = &args[1];
    let filename2 = &args[2];

    let  seq1=read_file_lines(filename1).unwrap();
    let  seq2=read_file_lines(filename2).unwrap();

    let table=lcs(&seq1, &seq2);
    print_diff(&table, &seq1, &seq2, seq1.len(), seq2.len());
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
            &"abcd".chars().map(|c| c.to_string()).collect(),
            &"adb".chars().map(|c| c.to_string()).collect(),
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
