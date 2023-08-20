package main

import (
	"bufio"
	"fmt"
	"net/http"
	"os"
	"time"
)

func main() {

	args := os.Args[1:]

	start := time.Now()
	for _, file := range args {

		reader, err := os.Open(file)
		if err != nil {
			panic("failed to read file: " + file)
		}

		scanner := bufio.NewScanner(reader)
		for scanner.Scan() {
			url := scanner.Text()
			CheckWebsite(url)
		}
	}
	fmt.Println("Total time: ", time.Since(start))

}

func CheckWebsite(url string) {
	if res, err := http.Get(url); err != nil {
		fmt.Printf("[ERR]: '%s' is down\n", url)
	} else {
		fmt.Printf("[%d]: '%s' is up\n", res.StatusCode, url)
	}
}
