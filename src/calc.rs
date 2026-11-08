use meval::eval_str;

pub struct MathHandler;

impl crate::InputHandler for MathHandler {
    fn get_canidates(&self, input: &str) -> Vec<String> {
        match eval_str(input) {
            Ok(a) => {
                vec![format!("{} = {}", input, a)]
            }
            Err(_) => vec![],
        }
    }

    fn execute(&self, action: String) -> Option<(String, i32)> {
        match eval_str(action) {
            Ok(a) => {
                println!(" {}", a);
                Some((a.to_string(), 0))
            }
            Err(_) => None,
        }
    }
}

/*
pub fn string_to_calc(string: String) -> Option<f64> {
    // split the string into charaters
    // then split it into tokens
    // evaluate all of the funtions fist (only make root for now)
    // evaluate all of the mutiplicaions and devisons
    // make numbers inot positive and negative
    //let r: Vec<char> = string.chars().collect();
    //let r = evaluate_braket(r, 0);

    None
}

fn evaluate_braket(vec: &Vec<char>, poss: usize) -> Option<f64> {
    let mut resutls: Vec<f64> = vec![0.0];
    let mut runnig = true;
    let mut poss = poss;
    let mut add_nex = true;
    while runnig {
        let c = vec[poss];
        if c.is_numeric() {
            todo!()
        }
    }

    None
}
*/
