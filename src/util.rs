pub fn text_to_bf(input: &str) -> String {
    let mut bf = String::new();

    for c in input.chars() {
        let n = c as u32;
        let sq_low = n.isqrt();
        let sq_high = n / sq_low;
        let rem = n % (sq_low * sq_high);

        let answer = "[-]>[-]<".to_string()
            + "+".repeat(sq_low as usize).as_str()
            + "[>"
            + "+".repeat(sq_high as usize).as_str()
            + "<-]>"
            + "+".repeat(rem as usize).as_str()
            + ".[-]<";
        bf.push_str(answer.as_str());
    }

    bf
}
