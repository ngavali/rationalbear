
package quikflow

type dbHandler struct {
	config ConfigStruct
	*database
}

func NewDBHandler(source DataSource, config ConfigStruct) *dbHandler {
	return &dbHandler{config, NewDatabase(source)}
}

func (d *dbHandler) Write(rs ResultSet) {

	desDb := d.database

	desDb.dbname = d.config.Dbname
	desDb.opts = d.config.Opts

	dsn := MakeDSN(desDb)
	desDb.Connect(dsn)

	defer desDb.Close()

	t := NewTable(d.config.Table)

	//Rinse data if required
	//Postgres requires special handling
	if desDb.driver == "postgres" {
		rs = Transform(rs, RinseData)
	}

	TableDbWriterTx(desDb, t, rs, 100)

}
func (d *dbHandler) Read() ResultSet {

	srcDb := d.database

	var q = NewQuery("select * from "+d.config.Table, nil)

	srcDb.dbname = d.config.Dbname
	srcDb.opts = d.config.Opts

	var dsn = MakeDSN(srcDb)

	srcDb.Connect(dsn)

	defer srcDb.Close()

	return QueryDbReader(srcDb, q, 100)

}
