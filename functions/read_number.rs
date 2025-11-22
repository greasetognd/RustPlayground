use stdin;
pub fn read_number(prompt: &str) -> f64 {
    println!("{}", prompt);
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwarp()
}