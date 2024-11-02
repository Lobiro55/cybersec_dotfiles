package main

import (
	"fmt"
	//"math"
)

func main() {
	/*x := 0
	y := 1
	ty := 2
	types(x, y, ty)*/
	veces()
}

func types(ty, x, y int) {
	if ty == 0 {
		fibo(x, y)
	} else {
		veces()
	}
}

func veces() {
	var try int16
	fmt.Println("Introduce el numero de veces a iterar")
	fmt.Scanln(&try)
	tester_veces(try)

}
func tester_veces(try int16) {
	if try == 0 {
		err := fmt.Errorf("El numero excede el limite permitido por la funcion")
		fmt.Println(err)
	} else {
		fmt.Println(try)
	}
}
func fibo(x, y int) {

}

func binnet(x, y int) {
	fmt.Println(y)
	fmt.Println(x)
}
