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

//rinse data for postgres
func RinseData(v interface{}) interface{} {
	switch v.(type) {
	case []uint8:
		v = string(v.([]uint8))
	}
	return v
}

/*
import (
	"fmt"
	"strconv"
	"strings"

	"github.com/lib/pq"
	_ "github.com/lib/pq"
)

type PgSQLDb struct {
	*database
	schema string
}

func NewPgSQLDb() *PgSQLDb {
	return &PgSQLDb{&database{"ngavali", "mypass", "localhost", 5432, "sslmode=disable", "postgres", nil}, "mydb"}
}

func (d *PgSQLDb) SetSchema(s string) {
	d.schema = s
}

func (d *PgSQLDb) dsn() (dsn string) {
	dsn = fmt.Sprintf("postgres://%s:%s@%s:%d/%s?%s", d.user, d.pass, d.host, d.port, d.schema, d.opts)
	fmt.Println(dsn)
	return
}

func (d *PgSQLDb) Connect() {
	//	d.database.Connect(dsnMaker(m))
	d.database.Connect(d.dsn())
}

func (d *PgSQLDb) ValueString(numArgs int) string {
	var valueHolder = make([]string, numArgs)
	for k := range valueHolder {
		valueHolder[k] = "$" + strconv.Itoa(k+1)
	}
	return fmt.Sprintf("( %s )", strings.Join(valueHolder, ","))
}

func (d *PgSQLDb) InsertQuery(table string, fields []string) (insertQuery string) {
	insertQuery = pq.CopyIn(table, fields...)
	return
}

func (d *PgSQLDb) Transaction() bool {
	return true
}

func (d *PgSQLDb) canBatch() bool {
	return false
}
*/
