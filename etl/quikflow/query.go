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

import (
	"fmt"
	"strconv"
	"strings"

	"github.com/lib/pq"
)

var insertQueryFunc = make(map[string]func(string, []string) string)

func ValueString(driver string, numArgs int) string {
	switch driver {
	case "mysql":
		return MySQLValueString(numArgs)
	}
	return ""
}

func MySQLValueString(numArgs int) string {
	var valueHolder = make([]string, numArgs)
	for i := range valueHolder {
		valueHolder[i] = "?"
	}
	/*
		var valueHolder = "?"
		for i := 0; i < numArgs-1; i++ {
			valueHolder += ",?"
		}
		return "(" + valueHolder + ")"
	*/
	return fmt.Sprintf("( %s )", strings.Join(valueHolder, ","))
	/*
		return "(?" + strings.Repeat(",?", numArgs-1) + ")"
	*/
}

func PgSQLValueString(numArgs int) string {
	var valueHolder = make([]string, numArgs)
	for k := range valueHolder {
		valueHolder[k] = "$" + strconv.Itoa(k+1)
	}
	return fmt.Sprintf("( %s )", strings.Join(valueHolder, ","))
}

func InsertQuery(driver, table string, fields []string) (insertQuery string) {
	f, ok := insertQueryFunc[driver]
	if ok {
		return f(table, fields)
	}
	return ""
	/*
		switch driver {
		case "mysql":
			return MySQLInsertQuery(table, fields)
		case "postgres":
			return PgSQLInsertQuery(table, fields)
		}
		return ""
		/*
			return insertQuery[driver](table, fields)
	*/
}

func MySQLInsertQuery(table string, fields []string) (insertQuery string) {
	insertQuery = fmt.Sprintf("insert into %s ( %s ) values %s", table, strings.Join(fields, ","), MySQLValueString(len(fields)))
	//insertQuery = "insert into " + table + " ( " + strings.Join(fields, ",") + " ) values " + "(?" + strings.Repeat(",?", len(fields)-1) + ")"
	//MySQLValueString(len(fields))
	return
}

func PgSQLInsertQuery(table string, fields []string) (insertQuery string) {
	insertQuery = pq.CopyIn(table, fields...)
	return
}

func RegisterInsertQueryFunc() {
	insertQueryFunc["mysql"] = MySQLInsertQuery
	insertQueryFunc["postgres"] = PgSQLInsertQuery
}

func init() {
	RegisterInsertQueryFunc()
}
