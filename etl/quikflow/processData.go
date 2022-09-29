
package quikflow

func DataProcessor(config ETLConfig) {

	//Create reader from datasource
	reader := NewDataSourceReader(config)

	rs := reader.Read()

	//Get writer from datasource
	writer := NewDataSourceWriter(config)

	writer.Write(rs)

}
