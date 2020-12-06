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

import "fmt"

type Vehicle interface {
	drive()
}

const (
	CAR = iota
	BIKE
)

type Car struct{}

func NewCar() *Car {
	return new(Car)
}

func (c *Car) start() {
	fmt.Println("car started.")
}
func (c *Car) drive() {
	c.start()
	fmt.Println("enjoy your car ride.")
	c.stop()
}
func (c *Car) stop() {
	fmt.Println("car stopped.")
}

type Bike struct{}

func NewBike() *Bike {
	return new(Bike)
}

func (b *Bike) start() {
	fmt.Println("bike started.")
}
func (b *Bike) drive() {
	b.start()
	fmt.Println("enjoy your bike ride.")
	b.stop()
}
func (b *Bike) stop() {
	fmt.Println("bike stopped.")
}

type VehicleFactory struct{}

func NewVehicleFactory() *VehicleFactory {
	return new(VehicleFactory)
}

func (vf *VehicleFactory) GetVehicle(vehicleType int) Vehicle {
	switch vehicleType {
	case CAR:
		return NewCar()
	case BIKE:
		return NewBike()
	}
	return nil
}

func main() {
	var v Vehicle
	vf := NewVehicleFactory()
	fmt.Println(":Let's drive a Car:")
	v = vf.GetVehicle(CAR)
	v.drive()
	fmt.Println(":Let's drive a Bike:")
	v = vf.GetVehicle(BIKE)
	v.drive()
}
