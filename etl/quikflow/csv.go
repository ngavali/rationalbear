package quikflow

import (
	"encoding/csv"
	"io"
	"os"
)

type csvHandler struct {
	filename  string
	separator rune
	hasHeader bool
	header    []string
}

func NewCSVHandler() *csvHandler {
	return &csvHandler{"file.csv", ',', true, nil}
}

func (h *csvHandler) Read() ResultSet {
	csvFile, _err := os.Open(h.filename)
	if _err != nil {
		panic(_err)
	}
	csvReader := csv.NewReader(csvFile)
	csvReader.Comma = h.separator

	var rows = make(chan row, 100)

	if h.hasHeader {
		cols, _err := csvReader.Read()
		if _err != nil {
			panic(_err)
		}
		h.header = cols
	}

	go func() {
		for {
			record, _err := csvReader.Read()
			if _err != nil {
				if _err == io.EOF {
					break
				}
				panic(_err)
			}
			var r = make(row, len(record))
			for k := range record {
				if record[k] != "NULL" {
					r[k] = record[k]
				}
			}
			rows <- r
		}
		close(rows)
	}()
	return ResultSet{h.header, rows}
}
