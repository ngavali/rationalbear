/* The MIT License

   Copyright (c) 2008, 2009 by gavali.nilesh80186 <gavali.nilesh80186@gmail.com>

   Permission is hereby granted, free of charge, to any person obtaining
   a copy of this software and associated documentation files (the
   "Software"), to deal in the Software without restriction, including
   without limitation the rights to use, copy, modify, merge, publish,
   distribute, sublicense, and/or sell copies of the Software, and to
   permit persons to whom the Software is furnished to do so, subject to
   the following conditions:

   The above copyright notice and this permission notice shall be
   included in all copies or substantial portions of the Software.

   THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
   EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
   MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
   NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS
   BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
   ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
   CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
   SOFTWARE.
*/

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
