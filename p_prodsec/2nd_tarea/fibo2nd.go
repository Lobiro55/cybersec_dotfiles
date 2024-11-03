package main

import (
	"fmt"
)

func main() {
	x := 0
	y := 1
	fibo(x, y)
}

func fibo(x, y int) {
	var z int8
	fmt.Print("inserte el numero de iteraciones: ")
	fmt.Scanln(&z)
	fmt.Println(z)
	fibo_calc(x, y, z)
}
func fibo_calc(x, y int, z int8) {
	var xy int
	for i := 0; i < int(z); i++ {
		fmt.Println(x)
		xy = x
		x = y
		y = xy + x
	}
}
