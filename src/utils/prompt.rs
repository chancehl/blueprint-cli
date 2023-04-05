pub fn prompt_for_value(prompt: String) -> String {
    let mut line = String::new();

    println!("{}", prompt);

    std::io::stdin().read_line(&mut line).unwrap();

    line.strip_suffix('\n')
        .unwrap_or(&line.to_string())
        .to_owned()
}
