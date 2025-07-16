
package main

import (
	"ETL/quikflow"
	"encoding/json"
	"io/ioutil"
)

func main() {

	etlConfig := quikflow.ETLConfig{}

	configJson, _ := ioutil.ReadFile("./etl.json")

	json.Unmarshal(configJson, &etlConfig)

	quikflow.DataProcessor(etlConfig)

}
