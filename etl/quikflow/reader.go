
package quikflow

func QueryDbReader(d *database, q Query, batchSize int) ResultSet {
	i := d.Query(q)

	resultSet, _err := i.Stream(batchSize)
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
	return resultSet

}
