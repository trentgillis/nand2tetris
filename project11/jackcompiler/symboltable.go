package jackcompiler

type stEntry struct {
	name     string
	dataType string
	kind     segment
	index    int
}

type symbolTable struct {
	table       map[string]stEntry
	staticCount int
	fieldCount  int
	argCount    int
	localCount  int
}

// type symbolTable map[string]stEntry

func newSymbolTable() symbolTable {
	return symbolTable{
		table:       make(map[string]stEntry),
		staticCount: 0,
		fieldCount:  0,
		argCount:    0,
		localCount:  0,
	}
}

func (st *symbolTable) Add(entry stEntry) {
	st.table[entry.name] = entry
}

func (st *symbolTable) Lookup(name string) (stEntry, bool) {
	entry, ok := st.table[name]
	return entry, ok
}
