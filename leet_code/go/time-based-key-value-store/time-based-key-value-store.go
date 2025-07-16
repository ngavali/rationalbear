package main

import "fmt"

type TimeMap struct {

}


func Constructor() TimeMap {
    return TimeMap{}
}


func (this *TimeMap) Set(key string, value string, timestamp int)  {

}


func (this *TimeMap) Get(key string, timestamp int) (value string) {
    return 
}

func main() {
  obj := Constructor();
  obj.Set("boy","vihaan",1);
  param_2 := obj.Get("boy",1);
  fmt.Println(param_2)
 }
