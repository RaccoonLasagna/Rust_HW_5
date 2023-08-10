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

// print spaces n times
pub fn repeat_print_space(times: i64){
    if times == 0{
        return;
    }
    print!("{}", " ");
    repeat_print_space(times-1);
}

// first half (5, 1)
pub fn pattern_print1(lines: i64, start: i64){
    let mut counting_array:Vec<String> = Vec::new();
    if lines == 0{
        return;
    }
    repeat_print_space(lines-1);
    repeat_print1(start);
    pattern_print1(lines - 1, start + 1);
}

// second half (5, 1)
pub fn pattern_print1_reverse(lines: i64, start: i64){
    let mut counting_array:Vec<String> = Vec::new();
    if lines == 0{
        return;
    }
    repeat_print_space(start);    
    repeat_print1(lines-1);
    pattern_print1_reverse(lines-1, start +1)
}

pub fn make_arrow1_recur(array: Vec<i64>, start: usize){
    if array.is_empty() || start == array.len(){
        return;
    }
    pattern_print1(array[start], 1);
    pattern_print1_reverse(array[start], 1);
    make_arrow1_recur(array, start+1)
}

fn main() {
    let test_vec: Vec<i64> = [5, 8, 3].to_vec();
    make_arrow1_recur(test_vec, 0)
}

#[test]
fn test1() {
    let mut cmd = Command::cargo_bin("hw5_3_3_make_arrow2_recur").unwrap();
    cmd.assert().success().stdout("    *
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