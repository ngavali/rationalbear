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

type ETLConfig struct {
	Source      ConfigStruct `json:"source"`
	Destination ConfigStruct `json:"destination"`
}

type ConfigStruct struct {
	Name   string `json:"name"`
	Dbname string `json:"dbname"`
	Opts   string `json:"opts"`
	Table  string `json:"table"`
}

type DataSource struct {
	Name   string `json:"name"`
	User   string `json:"user"`
	Pass   string `json:"pass"`
	Host   string `json:"host"`
	Port   int    `json:"port"`
	Driver string `json:"driver"`
}

func GetSource(name string) (DataSource, string) {
	for _, v := range datasource["db"] {
		if v.Name == name {
			return v, "db"
		}
	}
	return DataSource{}, ""
}
