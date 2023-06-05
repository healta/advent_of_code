use std::fs;

fn main() {
    let file_path = r"E:\Programming\AdventOfCode\AdventOfCode\2015\day_1\input.txt";
    let content = fs::read_to_string(file_path).expect("Failed to read file");
    let values = content.chars();
    let mut result = 0;
    let mut running_count = 0;
    let mut answer_check = false;
    for c in values {
        if c == '('{
            result+=1;
        }
        else{
            result-=1;
        }
        running_count+=1;
        if result == -1 && answer_check == false{
            answer_check = true;
            println!("{}", running_count);
        }
    }
    println!("{}", result);
}