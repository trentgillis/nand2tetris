package jackcompiler

import (
	"fmt"
	"os"
)

type segment string

const (
	ARGUMENT segment = "argument"
	LOCAL    segment = "local"
	STATIC   segment = "static"
	THIS     segment = "this"
	THAT     segment = "that"
	POINTER  segment = "pointer"
	TEMP     segment = "temp"
	CONSTANT segment = "constant"
)

type arithmeticCommand string

const (
	ADD arithmeticCommand = "add"
	SUB arithmeticCommand = "sub"
	NEG arithmeticCommand = "neg"
	EQ  arithmeticCommand = "eq"
	GT  arithmeticCommand = "gt"
	LT  arithmeticCommand = "lt"
	AND arithmeticCommand = "and"
	OR  arithmeticCommand = "or"
	NOT arithmeticCommand = "not"
)

type vmWriter struct {
	outf *os.File
}

func newVmWriter(outf *os.File) vmWriter {
	return vmWriter{
		outf: outf,
	}
}

func (vw *vmWriter) writePush(s segment, i int) {
	fmt.Fprintf(vw.outf, "push %v %d\n", s, i)
}

func (vw *vmWriter) writePop(s segment, i int) {
	fmt.Fprintf(vw.outf, "pop %v %d\n", s, i)
}

func (vw *vmWriter) writeArithmetic(command arithmeticCommand) {
	fmt.Fprintf(vw.outf, "%s\n", command)
}

func (vw *vmWriter) writeLabel(label string) {
	fmt.Fprintf(vw.outf, "label %s\n", label)
}

func (vw *vmWriter) writeGoto(label string) {
	fmt.Fprintf(vw.outf, "goto %s\n", label)
}

func (vw *vmWriter) writeIf(label string) {
	fmt.Fprintf(vw.outf, "if-goto %s\n", label)
}

func (vw *vmWriter) writeFunction(className string, subroutineName string, nVars int) {
	fmt.Fprintf(vw.outf, "function %s.%s %d\n", className, subroutineName, nVars)
}

func (vw *vmWriter) writeCall(className string, subroutineName string, nVars int) {
	fmt.Fprintf(vw.outf, "call %s.%s %d\n", className, subroutineName, nVars)
}

func (vw *vmWriter) writeReturn() {
	fmt.Fprintf(vw.outf, "return\n")
}
