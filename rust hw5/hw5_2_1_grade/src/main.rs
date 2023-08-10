pub fn grade(input: Vec<f64>) -> Vec<String>{
    let mut output = Vec::new();
    for grade in input{
        if grade > 100.0 {
            output.push("Invalid score".to_string())
        } else if grade >= 95.0 {
            output.push("Excellent with A+".to_string())
        } else if grade >= 81.0 {
            output.push("A".to_string())
        } else if grade >= 71.0 {
            output.push("B".to_string())
        } else if grade >= 61.0 {
            output.push("C".to_string())
        } else if grade >= 50.0 {
            output.push("D".to_string())
        } else if grade >= 0.0 {
            output.push("Failed with F".to_string())
        } else {
            output.push("Invalid score".to_string())
        }
    }
    output
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