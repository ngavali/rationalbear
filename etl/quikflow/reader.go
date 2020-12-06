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
