use std::{collections::HashMap, sync::LazyLock};

static COMP_CODES: LazyLock<HashMap<&str, &str>> = LazyLock::new(|| {
    HashMap::from([
        ("0", "101010"),
        ("1", "111111"),
        ("-1", "111010"),
        ("D", "001100"),
        ("A", "110000"),
        ("!D", "001101"),
        ("!A", "110001"),
        ("0", "101010"),
        ("-D", "001111"),
        ("-A", "110011"),
        ("D+1", "011111"),
        ("A+1", "110111"),
        ("D-1", "001110"),
        ("A-1", "110010"),
        ("D+A", "000010"),
        ("D-A", "010011"),
        ("A-D", "000111"),
        ("D&A", "000000"),
        ("D|A", "010101"),
        ("M", "110000"),
        ("!M", "110001"),
        ("-M", "110011"),
        ("M+1", "110111"),
        ("M-1", "110010"),
        ("D+M", "000010"),
        ("D-M", "010011"),
        ("M-D", "111000"),
        ("D&M", "000000"),
        ("D|M", "010101"),
    ])
});

static JUMP_CODES: LazyLock<HashMap<&str, &str>> = LazyLock::new(|| {
    HashMap::from([
        ("JGT", "001"),
        ("JEQ", "010"),
        ("JGE", "011"),
        ("JLT", "100"),
        ("JNE", "101"),
        ("JLE", "110"),
        ("JMP", "111"),
    ])
});

pub fn comp(comp: Option<&str>) -> &str {
    match comp {
        Some(jump) => COMP_CODES.get(jump).unwrap(),
        None => "000",
    }
}

pub fn jump(jump: Option<&str>) -> &str {
    match jump {
        Some(jump) => JUMP_CODES.get(jump).unwrap(),
        None => "000",
    }
}

#[cfg(test)]
mod tests {}
