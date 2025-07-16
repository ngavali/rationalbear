package quikflow

import (
	"encoding/json"
	"io/ioutil"
)

var datasource = map[string][]DataSource{}

func init() {
	dataSourceJson, _ := ioutil.ReadFile("./datasource.json")
	json.Unmarshal(dataSourceJson, &datasource)
}

type SourceReader interface {
	Read() ResultSet
}

type SourceWriter interface {
	Write(ResultSet)
}

func NewDataSourceReader(config ETLConfig) SourceReader {

	source, class := GetSource(config.Source.Name)

	switch class {
	case "db":
		return NewDBHandler(source, config.Source)
	}
	return nil
}

func NewDataSourceWriter(config ETLConfig) SourceWriter {

	source, class := GetSource(config.Destination.Name)

	switch class {
	case "db":
		return NewDBHandler(source, config.Destination)
	}
	return nil
}
