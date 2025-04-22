use std::io::stdin;

fn main() {
    loop {
        let mut input: String = String::new();
        
        stdin()
            .read_line(&mut input)
            .expect("Error while reading line");
            
        command_handler(&input);
        
        let cleaned_input = input.trim().to_string();
    
        if cleaned_input == "exit" || cleaned_input == "quit" {
            break;
        }
    }
}

fn command_handler(input: &String) {
    let token_array: Vec<String> = input
        .split_whitespace()
        .map(|token| token.to_string())
        .collect();
}