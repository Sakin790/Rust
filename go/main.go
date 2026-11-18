package main

import (
	"fmt"
)

func main() {

	player := map[string]int{
		"Messi":   10,
		"Rolando": 7,
		"Neymar":  10,
	}

	fmt.Println(player)

	for key, value := range player {

		fmt.Println(key, value)
	}
	fmt.Println(len(player))

	_, exist := player["mBappay"]

	if exist {
		fmt.Println("Key exist")
	} else {
		fmt.Println("Key dose not exist")
	}

}
