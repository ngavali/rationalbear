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
