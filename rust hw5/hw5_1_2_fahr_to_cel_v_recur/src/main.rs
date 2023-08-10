pub fn fahr_to_cel_v(array: Vec<f64>) -> Vec<f64>{
    let input = array;
    let mut output:Vec<f64> = Vec::new();
    fn fahr_to_cel_v_inside(array: &Vec<f64>, new_array: &mut Vec<f64>){
        if array.is_empty(){
            return;
        }
        else{
            let c: f64 = (array[0] - 32.0)*(5.0/9.0);
            let c_rounded = (c * 100.0).round() / 100.0; 
            new_array.push(c_rounded);
            fahr_to_cel_v_inside(&array[1..].to_vec(), new_array);
        }
    }

    fahr_to_cel_v_inside(&input, &mut output);
    return output;
}

fn main() {
    let input: Vec<f64> = vec![45.0, 31.0, -12.0, -89.0];
    
    print!("{:?}", fahr_to_cel_v(input));
}

#[test]

fn test1(){
    let input: Vec<f64> = vec![45.0, 31.0, -12.0, -89.0];
    assert_eq!(fahr_to_cel_v(input), [7.22, -0.56, -24.44, -67.22])
}