.DEFAULT_GOAL := run

.PHONY:prod

build-prod:
	cargo build --release

test:
	cargo test

run: build-prod
	./target/release/hack_assembler ./asm/add/Add.asm
	./target/release/hack_assembler ./asm/max/Max.asm
	./target/release/hack_assembler ./asm/max/MaxL.asm
	./target/release/hack_assembler ./asm/pong/Pong.asm
	./target/release/hack_assembler ./asm/pong/PongL.asm
	./target/release/hack_assembler ./asm/rect/Rect.asm
	./target/release/hack_assembler ./asm/rect/RectL.asm
