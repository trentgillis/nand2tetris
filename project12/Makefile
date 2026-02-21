.DEFAULT_GOAL := compile

.PHONY:compile

compile:
	cp ./Math.jack test/MathTest
	./tools/JackCompiler.sh ./test/MathTest
	cp ./Memory.jack test/MemoryTest
	./tools/JackCompiler.sh ./test/MemoryTest
