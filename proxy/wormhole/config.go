package wormhole

import (
	"encoding/json"
	"io/ioutil"

	"github.com/pkg/errors"
)

var destinationConfig map[string]destinationConf
var wormholeConfig map[string]wormholeConf

type destinationConf struct {
	Host  string `json:"host"`
	Port  uint   `json:"port"`
	Proxy bool   `json:"proxy"`
}

type wormholeConf struct {
	Name   string   `json:"name"`
	Port   uint     `json:"port"`
	BeList []string `json:"destination"`
	Algo   int      `json:"algo"`
	Check  int      `json:"check"`
}

func loadServerFromConfig(configFile string) map[string]destinationConf {
	configFileContent, _err := ioutil.ReadFile(configFile)
	if _err != nil {
		panic(errors.Errorf("Error while opening config file. Error => %s\n", _err))
	}
	config := make(map[string]destinationConf)
	_err = json.Unmarshal(configFileContent, &config)
	if _err != nil {
		panic(errors.Errorf("Error while reading the config. Error => %s\n", _err))
	}
	return config
}

func loadWormholeFromConfig(configFile string) map[string]wormholeConf {
	configFileContent, _err := ioutil.ReadFile(configFile)
	if _err != nil {
		panic(errors.Errorf("Error while opening config file. Error => %s\n", _err))
	}
	config := make(map[string]wormholeConf)
	_err = json.Unmarshal(configFileContent, &config)
	if _err != nil {
		panic(errors.Errorf("Error while reading the config. Error => %s\n", _err))
	}
	return config
}
