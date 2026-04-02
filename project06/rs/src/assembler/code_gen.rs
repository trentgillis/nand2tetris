use std::{collections::HashMap, sync::LazyLock};

static COMP_CODES: LazyLock<HashMap<&str, &str>> = LazyLock::new(|| {
    HashMap::from([
        ("0", "0101010"),
        ("1", "0111111"),
        ("-1", "0111010"),
        ("D", "0001100"),
        ("A", "0110000"),
        ("!D", "0001101"),
        ("!A", "0110001"),
        ("-D", "0001111"),
        ("-A", "0110011"),
        ("D+1", "0011111"),
        ("A+1", "0110111"),
        ("D-1", "0001110"),
        ("A-1", "0110010"),
        ("D+A", "0000010"),
        ("D-A", "0010011"),
        ("A-D", "0000111"),
        ("D&A", "0000000"),
        ("D|A", "0010101"),
        ("M", "1110000"),
        ("!M", "1110001"),
        ("-M", "1110011"),
        ("M+1", "1110111"),
        ("M-1", "1110010"),
        ("D+M", "1000010"),
        ("D-M", "1010011"),
        ("M-D", "1000111"),
        ("D&M", "1000000"),
        ("D|M", "1010101"),
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

pub fn dest(dest: Option<&str>) -> String {
    if dest.is_none() {
        return String::from("000");
    }

    let dest = dest.unwrap();
    let mut dest_parts = ["0", "0", "0"];
    if dest.contains("A") {
        dest_parts[0] = "1";
    }
    if dest.contains("D") {
        dest_parts[1] = "1";
    }
    if dest.contains("M") {
        dest_parts[2] = "1";
    }

    dest_parts.join("")
}

pub fn comp(comp: &str) -> &str {
    COMP_CODES.get(comp).unwrap()
}

pub fn jump(jump: Option<&str>) -> &str {
    match jump {
        Some(jump) => JUMP_CODES.get(jump).unwrap(),
        None => "000",
    }
}

#[cfg(test)]
mod tests {}
