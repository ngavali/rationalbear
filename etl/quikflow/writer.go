
package quikflow

import (
	"database/sql"
	"strings"
)

func TableDbWriterTx(d *database, t Table, rs ResultSet, batchSize int) {

	var _err error
	var Tx *sql.Tx
	var stmt *sql.Stmt

	Tx, _err = d.Begin()
	if _err != nil {
		panic(_err)
	}
	defer Tx.Rollback()

	stmt, _err = Tx.Prepare("delete from " + t.Name())
	if _err != nil {
		panic(_err)
	}
	_, _err = stmt.Exec()
	if _err != nil {
		panic(_err)
	}

	insertQuery := InsertQuery(d.Driver(), t.Name(), rs.columns)

	if batchSize > 1 && d.canBatch() == true {
		args := make([]interface{}, 0, batchSize)
		vstring := "," + ValueString(d.Driver(), len(rs.columns))

		insertQueryBatch := insertQuery + strings.Repeat(vstring, batchSize-1)
		stmt, _err = Tx.Prepare(insertQueryBatch)
		var b int
		for r := range rs.rows {
			args = append(args, r...)
			b++
			if b == batchSize {
				_, _err = stmt.Exec(args...)
				if _err != nil {
					panic(_err)
				}
				b = 0
				args = args[:0]
			}
		}
		if b > 0 {
			insertQuery += strings.Repeat(vstring, b-1)
			stmt, _err = Tx.Prepare(insertQuery)
			if _err != nil {
				panic(_err)
			}
			_, _err = stmt.Exec(args...)
			if _err != nil {
				panic(_err)
			}
			args = nil //Free memory
		}
	} else {

		stmt, _err = Tx.Prepare(insertQuery)
		if _err != nil {
			panic(_err)
		}
		for r := range rs.rows {
			_, _err = stmt.Exec(r...)
			if _err != nil {
				panic(_err)
			}
		}

		if d.Driver() == "postgres" {
			//flush for copy command
			_, _err = stmt.Exec()
			if _err != nil {
				panic(_err)
			}
		}
	}

	_err = Tx.Commit()
	if _err != nil {
		panic(_err)
	}

}

func TableDbWriter(d *database, t Table, rs ResultSet, batchSize int) {
	/*
		var d = quikflow.NewMySQLDb()
		var q = quikflow.NewQuery("select * from mydb.mytable order by id")

		d.SetSchema("mydb")
		d.Connect()
	*/
	var _err error
	var stmt *sql.Stmt

	insertQuery := InsertQuery(d.Driver(), t.Name(), rs.columns)

	if batchSize > 1 && d.canBatch() == true {
		stmt, _err = d.Prepare("delete from " + t.Name())
		if _err != nil {
			panic(_err)
		}
		_, _err = stmt.Exec()
		if _err != nil {
			panic(_err)
		}
		args := make([]interface{}, 0, batchSize)
		vstring := "," + ValueString(d.Driver(), len(rs.columns))

		insertQueryBatch := insertQuery + strings.Repeat(vstring, batchSize-1)
		stmt, _err = d.Prepare(insertQueryBatch)
		if _err != nil {
			panic(_err)
		}
		var b int
		for r := range rs.rows {
			args = append(args, r...)
			b++
			if b == batchSize {
				_, _err = stmt.Exec(args...)
				if _err != nil {
					panic(_err)
				}
				b = 0
				args = args[:0]
			}
		}
		if b > 0 {
			insertQuery += strings.Repeat(vstring, b-1)
			stmt, _err = d.Prepare(insertQuery)
			if _err != nil {
				panic(_err)
			}
			_, _err = stmt.Exec(args...)
			if _err != nil {
				panic(_err)
			}
			args = nil //Free memory
		}
	} else {
		stmt, _err = d.Prepare("delete from " + t.Name())
		if _err != nil {
			panic(_err)
		}
		_, _err = stmt.Exec()
		if _err != nil {
			panic(_err)
		}
		stmt, _err = d.Prepare(insertQuery)
		if _err != nil {
			panic(_err)
		}
		//	fields = rs.fields.Name()

		for r := range rs.rows {
			/*var valueHolder = make([]string, len(r.Fields()))
			for i := range valueHolder {
				valueHolder[i] = "?"
			}
			*/
			//insertQuery := fmt.Sprintf("insert into %s ( %s ) values %s", t.Name(), strings.Join(fields, ","), d.ValueString(len(fields)))
			//fmt.Println(insertQuery)
			//fmt.Println("%s", r.values)
			//q := NewQuery(insertQuery, r.values)
			_, _err = stmt.Exec(r...)
			if _err != nil {
				panic(_err)
			}
		}

		if d.Driver() == "postgres" {
			//flush for copy command
			_, _err = stmt.Exec()
			if _err != nil {
				panic(_err)
			}
		}
	}

	/*i := d.Query(q)

	fmt.Println(d)
	fmt.Println(q)
	fmt.Println(i)
	rows, _err := i.Stream(batchSize)
	if _err != nil {
		panic(_err)
	}

	/*
		if _err == nil {
			for r := range rows {
				fmt.Println(r)
			}
		}
				for k := 0; k < 5; k++ {
					rows, isDone := i.Rows(1)
					if isDone {
						for _, v := range rows {
							v.Values()
							fmt.Printf("%+v\n", v)
						}
					} else {
						fmt.Println("no more rows to read")
						break
					}
				}
			fmt.Println("Read complete")
	*/

}
