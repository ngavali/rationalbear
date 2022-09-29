
package quikflow

import "fmt"

func MakeDSN(d *database) (dsn string) {
	switch d.Driver() {
	case "mysql":
		dsn = fmt.Sprintf("%s:%s@tcp(%s:%d)/%s?%s", d.user, d.pass, d.host, d.port, d.dbname, d.opts)
	case "postgres":
		dsn = fmt.Sprintf("postgres://%s:%s@%s:%d/%s?%s", d.user, d.pass, d.host, d.port, d.dbname, d.opts)
	case "oracle":
		dsn = "oracleDSN"
	}
	return
}
