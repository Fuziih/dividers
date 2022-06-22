use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    for i in &args[1..] {
        let arg = match i.parse::<i64>() {
            Ok(val) => val,
            Err(e) => {
                println!("Unable to parse number from argument: {}", e);
                return;
            }
        };
        let mut res: Vec<String> = Vec::new();
        for n in 1..arg {
            if arg % n == 0 {
                res.push(n.to_string());
            }
        };
        res.push(arg.to_string());
        let output = res.join(", ");
        println!("{}", output);
    }
}
