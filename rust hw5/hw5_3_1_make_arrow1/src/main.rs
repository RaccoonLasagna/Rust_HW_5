use assert_cmd::Command;

pub fn make_arrow1(input: Vec<i64>){
    for length in input{
        for outside_loops1 in 1..length+1 {
            for inside_loops1 in 1..outside_loops1+1{
                print!("{}", "*")
            }
            print!("{}", "\n")
        }
        for outside_loops2 in (1..length).rev() {
            for inside_loops2 in 1..outside_loops2+1{
                print!("{}", "*")
            }
            print!("{}", "\n")
        }
    }

}

fn main() {
    let length_array: Vec<i64> = [5, 3, 8].to_vec();
    make_arrow1(length_array)
}

#[test]
fn test1() {
    let mut cmd = Command::cargo_bin("hw5_3_1_make_arrow1").unwrap();
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
");
}