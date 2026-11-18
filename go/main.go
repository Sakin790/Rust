package main

import "fmt"

func main() {

	score := []int{1000, 2000, 3000, 4000, 5000}

	for index, value := range score {

		fmt.Println(index, value)
	}

	fmt.Println("Out of Loops")
}
