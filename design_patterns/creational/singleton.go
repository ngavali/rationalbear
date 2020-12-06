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

package main

import (
	"database/sql"
	"fmt"
	"sync"

	_ "github.com/go-sql-driver/mysql"
)

type myDB struct {
	*sql.DB
	once sync.Once
}

var myDBinstance *myDB

func GetDBinstance() *myDB {
	if myDBinstance == nil {
		myDBinstance = new(myDB)
		myDBinstance.once.Do(func() {
			db, _err := sql.Open("mysql", "ngavali:mypass@tcp(localhost:3306)/alexandria?columnsWithAlias=true")
			if _err != nil {
				panic(_err)
			}
			myDBinstance.DB = db
			fmt.Println("Creating new DB connection...")
		})
	}
	return myDBinstance
}

var myQuery = func(wg *sync.WaitGroup, id int, table string) {

	db := GetDBinstance()

	stmt, _err := db.Prepare("select * from " + table)
	if _err != nil {
		panic(_err)
	}
	rows, _err := stmt.Query()
	if _err != nil {
		panic(_err)
	}
	defer rows.Close()
	columns, _err := rows.Columns()
	if _err != nil {
		panic(_err)
	}
	scanInterface := make([]interface{}, len(columns))
	for rows.Next() {
		row := make([]string, len(columns))
		for i := range row {
			scanInterface[i] = &row[i]
		}
		rows.Scan(scanInterface...)
		fmt.Printf("goroutine:%2d:%s=column%v:row%+v\n", id, table, columns, row)
	}

	fmt.Printf("DB connection stats:[db.Stats=%+v]\n", db.Stats())
	wg.Done()

}

func main() {

	wg := sync.WaitGroup{}
	wg.Add(1)
	go myQuery(&wg, 1, "seq_1_to_3")
	wg.Add(1)
	go myQuery(&wg, 2, "seq_4_to_6")
	wg.Add(1)
	go myQuery(&wg, 3, "seq_7_to_9")

	wg.Wait()

}
