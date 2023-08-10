pub fn grade(array: Vec<f64>) -> Vec<String>{
    let input = array;
    let mut output:Vec<String> = Vec::new();
    fn grade_inside(array: &Vec<f64>, new_array: &mut Vec<String>){
        if array.is_empty(){
            return;
        }
        else{
            if array[0] > 100.0 {
                new_array.push("Invalid score".to_string())
            } else if array[0] >= 95.0 {
                new_array.push("Excellent with A+".to_string())
            } else if array[0] >= 81.0 {
                new_array.push("A".to_string())
            } else if array[0] >= 71.0 {
                new_array.push("B".to_string())
            } else if array[0] >= 61.0 {
                new_array.push("C".to_string())
            } else if array[0] >= 50.0 {
                new_array.push("D".to_string())
            } else if array[0] >= 0.0 {
                new_array.push("Failed with F".to_string())
            } else {
                new_array.push("Invalid score".to_string())
            }
            grade_inside(&array[1..].to_vec(), new_array);
        }
    }

    grade_inside(&input, &mut output);
    return output;
}

fn main() {
    let grades: Vec<f64> = [100., -100., 101., 50., 80., 90., 40.].to_vec();
    print!("{:?}", grade(grades))
}

#[test]

fn test1(){
    let grades: Vec<f64> = [100., -100., 101., 50., 80., 90., 40.].to_vec();
    assert_eq!(grade(grades), ["Excellent with A+", "Invalid score", "Invalid score", "D", "B", "A", "Failed with F"])
}