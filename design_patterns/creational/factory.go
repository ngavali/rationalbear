
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
