
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
