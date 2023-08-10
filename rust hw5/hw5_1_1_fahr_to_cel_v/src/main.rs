pub fn fahr_to_cel_v(v: &[f64]) -> Vec<f64>{
    let mut result_vector = Vec::new();
    let mut cel = 0.0;
    let mut cel_rounded = 0.0;
    for temp in v{
        cel = (temp - 32.0)*(5.0/9.0);
        cel_rounded = (cel * 100.0).round() / 100.0; 
        result_vector.push(cel_rounded);
        };
    result_vector
}

fn main() {
    let input: Vec<f64> = vec![45.0, 31.0, -12.0, -89.0];
    print!("{:?}", fahr_to_cel_v(&input));
}

#[test]

fn test1(){
    let input: Vec<f64> = vec![45.0, 31.0, -12.0, -89.0];
    assert_eq!(fahr_to_cel_v(&input), [7.22, -0.56, -24.44, -67.22])
}