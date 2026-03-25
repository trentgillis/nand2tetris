use std::fs;

use hack_assembler::assembler;

#[test]
fn test_assembler() {
    let asm_paths = vec![
        "asm/add/Add.asm",
        "asm/max/MaxL.asm",
        "asm/max/Max.asm",
        "asm/pong/PongL.asm",
        "asm/pong/Pong.asm",
        "asm/rect/RectL.asm",
        "asm/rect/Rect.asm",
    ];

    for path in asm_paths {
        let args = vec![String::from(""), String::from(path)];
        let config = assembler::cli_config::CliConfig::build(args.into_iter()).unwrap();
        assembler::assemble(config).unwrap();

        let output_path = format!(
            "{}/{}",
            env!("CARGO_MANIFEST_DIR"),
            path.replace(".asm", ".hack")
        );
        let output = std::fs::read_to_string(&output_path).unwrap();

        let cmp_path = format!(
            "{}/{}",
            env!("CARGO_MANIFEST_DIR"),
            path.replace(".asm", "Cmp.hack")
        );
        let cmp = std::fs::read_to_string(&cmp_path).unwrap();

        assert_eq!(output, cmp);

        fs::remove_file(output_path).expect("Failed to delete output file after test.");
    }
}
