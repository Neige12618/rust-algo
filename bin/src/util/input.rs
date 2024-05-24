pub fn get_correct_number(max: usize) -> usize {
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        if input.trim().is_empty() {
            return 0;
        } else {
            let i = input.trim().parse::<usize>();
            if let Ok(i) = i {
                if i >= max {
                    println!("请输入小于{}的数。", max);
                    continue;
                } else {
                    return i;
                }
            } else {
                println!("请输入数字。");
                continue;
            }
        }
    }
}
