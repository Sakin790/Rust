package main

import (
	"fmt"
)

func main() {

	score := []int{1000, 2000, 3000, 4000, 5000}

	for index, value := range score {

		fmt.Println(index, value)
	}

	fmt.Println("Out of Loops")

	price := map[string]int{

		"apple":  21,
		"banana": 32,
		"guava":  32,
	}

	for key, value := range price {

		fmt.Println(key, value)
	}

	_, exist := price["manngo"]
	if exist {
		fmt.Println("Key exist bro")
	} else {
		fmt.Println("Key dose not exist")
	}

}
