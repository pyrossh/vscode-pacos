package main

func data() {
}

func main() {
	list := []string{}
	st := "123123"
	for i := range list {
		println(i)
	}
	data();
}

type Data struct {
	Age int
}

func (a *Data) Get(i int, f float32, s string) int  {
	return a.Age
}