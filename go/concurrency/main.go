package main

import (
	"bufio"
	"fmt"
	"net/http"
	"os"
	"sync"
	"time"
)

func main() {

	args := os.Args[1:]

	start := time.Now()
	var wg sync.WaitGroup
	const workerCount = 3
	jobs := make(chan string, workerCount)
	for i := 0; i < workerCount; i++ {
		go CheckWebsite(jobs, &wg, i)
	}

	for _, file := range args {

		reader, err := os.Open(file)
		if err != nil {
			panic("failed to read file: " + file)
		}

		scanner := bufio.NewScanner(reader)
		for scanner.Scan() {
			url := scanner.Text()
			wg.Add(1)
			jobs <- url
		}
	}
	wg.Wait()
	fmt.Println("Total time: ", time.Since(start))

}

func CheckWebsite(jobs chan string, wg *sync.WaitGroup, workerId int) {
	for url := range jobs {
		start := time.Now()
		if res, err := http.Get(url); err != nil {
			fmt.Printf("[%d]: [ERR]: '%s' is down - elapsed: %v\n", workerId, url, time.Since(start))
		} else {
			fmt.Printf("[%d]: [%d]: '%s' is up - duration: %v\n", workerId, res.StatusCode, url, time.Since(start))
		}
		wg.Done()
	}
}
