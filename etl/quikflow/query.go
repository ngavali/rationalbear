
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
