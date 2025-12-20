use std::collections::HashMap;

pub fn build_bracket_map(source: &str) -> Result<HashMap<usize, usize>, String> {
    let mut stk = Vec::new();
    let mut bracket_map = HashMap::new();

    for (idx, c) in source.chars().enumerate() {
        match c {
            '[' => stk.push(idx),
            ']' => {
                let open_idx = stk
                    .pop()
                    .ok_or_else(|| format!("Unmatched bracket at position {idx}"))?;

                bracket_map.insert(open_idx, idx);
                bracket_map.insert(idx, open_idx);
            }
            _ => {}
        }
    }

    if let Some(lst) = stk.last() {
        return Err(format!("Unmatched Bracket at position {lst}"));
    }

    Ok(bracket_map)
}
