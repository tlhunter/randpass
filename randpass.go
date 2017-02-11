package main

import (
	"fmt"
	"math/rand"
	"os"
	"strconv"
	"time"
)

func main() {
	rand.Seed(time.Now().UnixNano())

	firstArgument := "12"

	if len(os.Args) > 1 {
		firstArgument = os.Args[1]
	}

	length, err := strconv.Atoi(firstArgument)

	if err != nil || length < 1 || length > 9999 {
		length = 12
	}

	pool := "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ!@#$%^&*()"

	for i := 0; i < length; i++ {
		fmt.Printf("%c", pool[rand.Intn(len(pool))])
	}

	fmt.Println()
}
