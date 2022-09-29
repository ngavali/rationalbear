
package quikflow

import (
	"database/sql"

	_ "github.com/go-sql-driver/mysql"
	_ "github.com/lib/pq"
)

type row []interface{}

type ResultSet struct {
	columns []string
	rows    <-chan row
}

func Transform(r ResultSet, filter func(interface{}) interface{}) ResultSet {
	if filter != nil {
		var rows = make(chan row, cap(r.rows))
		go func() {
			for r := range r.rows {
				//for k := range r.values {
				for k := 0; k < len(r); k++ {
					r[k] = filter(r[k])
				}
				rows <- r
			}
			close(rows)
		}()
		return ResultSet{r.columns, rows}
	}
	return r
}

type database struct {
	user    string
	pass    string
	host    string
	port    int
	dbname  string
	opts    string
	driver  string
	isBatch bool
	*sql.DB
}

func NewDatabase(config DataSource) *database {
	var isBatch bool
	if config.Driver == "mysql" {
		isBatch = true
	}
	return &database{
		config.User,
		config.Pass,
		config.Host,
		config.Port,
		"",
		"",
		config.Driver,
		isBatch,
		nil,
	}
}

type Table struct {
	name string
}

func NewTable(n string) Table {
	return Table{n}
}

func (t Table) Name() string {
	return t.name
}

type Query struct {
	SQL  string
	Args []interface{}
}

func NewQuery(sql string, args []interface{}) Query {
	return Query{sql, args}
}

type RowIterator struct {
	result *sql.Rows
}

func (I *RowIterator) Stream(bufferSize int) (ResultSet, error) {
	var rows = make(chan row, bufferSize)
	var columns, _ = I.result.Columns()
	var numCols = len(columns)
	go func() {
		var scanInterface = make([]interface{}, numCols)
		for I.result.Next() {
			var r = make(row, numCols)
			for i := 0; i < numCols; i++ {
				scanInterface[i] = &r[i]
			}
			_err := I.result.Scan(scanInterface...)
			if _err != nil {
				panic(_err)
			}
			rows <- r
		}
		close(rows)
	}()
	return ResultSet{columns, rows}, nil
}

func (d *database) Query(q Query) *RowIterator {
	stmt, _err := d.DB.Prepare(q.SQL)
	if _err != nil {
		panic(_err)
	}
	result, _err := stmt.Query(q.Args...)
	if _err != nil {
		panic(_err)
	}
	return &RowIterator{result}
}

func (d *database) Driver() string {
	return d.driver
}

func (d *database) Connect(dsn string) {
	db, _err := sql.Open(d.driver, dsn)
	if _err != nil {
		panic(_err)
	}
	_err = db.Ping()
	if _err != nil {
		panic(_err)
	}
	d.DB = db
}

func (d *database) canBatch() bool {
	return d.isBatch
}

func (d *database) ExecuteStmt(stmt *sql.Stmt, args ...interface{}) {

	_, _err := stmt.Exec(args...)
	if _err != nil {
		panic(_err)
	}
	/*
	   li, _err := result.LastInsertId()
	   ra, _err := result.RowsAffected()
	   fmt.Printf("%d %d\n", li, ra)
	*/
}
