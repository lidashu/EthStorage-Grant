package main

func fib(iter int) int {
	var a0, a1, maxc int
	a0 = 1
	a1 = 1
	maxc = 1 << 32
	for i := 2; i <= iter; i++ {
		a0, a1 = a1%maxc, (a0+a1)%maxc
	}
	return a1

}

func main() {
	fib(1000000)
}
