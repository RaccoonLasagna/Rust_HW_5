use assert_cmd::Command;

// print * n times
pub fn repeat_print1(times: i64){
    if times == 0{
        print!("{}", "\n");
        return;
    }
    print!("{}", "*");
    repeat_print1(times-1);
}

pub fn pattern_print1(lines: i64, start: i64){
    if lines == 0{
        return;
    }
    repeat_print1(start);
    pattern_print1(lines - 1, start + 1);
}

pub fn pattern_print1_reverse(lines: i64){
    if lines == 0{
        return;
    }
    repeat_print1(lines-1);
    pattern_print1_reverse(lines-1)
}

pub fn make_arrow1_recur(array: Vec<i64>, start: usize){
    if array.is_empty() || start == array.len(){
        return;
    }
    pattern_print1(array[start], 1);
    pattern_print1_reverse(array[start]);
    make_arrow1_recur(array, start+1)
}

fn main() {
    let test_vec: Vec<i64> = [5, 8, 3].to_vec();
    make_arrow1_recur(test_vec, 0)
}

#[test]
fn test1() {
    let mut cmd = Command::cargo_bin("hw5_3_3_make_arrow1_recur").unwrap();
    cmd.assert().success().stdout("*
**
***
****
*****
****
***
**
*

*
**
***
****
*****
******
*******
********
*******
******
*****
****
***
**
*

*
**
***
**
*

");
}