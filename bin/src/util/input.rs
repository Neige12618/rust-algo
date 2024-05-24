pub fn get_correct_number(max: usize) -> usize {
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        if input.trim().is_empty() {
            return 0;
        } else {
            let i = input.trim().parse::<usize>().unwrap();
            if i >= max {
                continue;
            }
            return i;
        }
    }
}
