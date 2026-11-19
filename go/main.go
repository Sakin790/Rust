package main

import "fmt"

type User struct {
	Id   int
	Name string
}

type Namer interface {
	chnageName(name string)
	getName() string
}

func (u *User) changeName(nm string) {
	u.Name = nm
}

func main() {
	p1 := User{
		Id:   1,
		Name: "Sakin",
	}
	fmt.Println(p1)

	p1.changeName("Mahid")
	fmt.Println(p1)
}
