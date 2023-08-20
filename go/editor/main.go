package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

type element struct {
	data byte
	prev *element
	next *element
}

func (e *element) GetFirst() *element {
	if e.prev == nil {
		return e
	}
	return e.prev.GetFirst()
}

func (e *element) Print() {
	var output strings.Builder

	first := e.GetFirst()

	for first != nil {
		output.WriteByte(first.data)
		first = first.next
	}
	fmt.Println(output.String())
}

func main() {
	var commandCount int
	fmt.Scan(&commandCount)

	reader := bufio.NewReader(os.Stdin)

	cursor := &element{data: '|'}

	for i := 0; i < commandCount; i++ {
		line, _ := reader.ReadString('\n')
		line = strings.Trim(line, "\n")
		fields := strings.Fields(line)

		switch fields[0] {
		case "INSERT":
			newElement := &element{data: fields[1][0]}
			if cursor.prev != nil {
				cursor.prev.next = newElement
				newElement.prev = cursor.prev
			}
			newElement.next = cursor
			cursor.prev = newElement
		case "LEFT":
			if cursor.prev != nil {
				cursor.prev.data, cursor.data = cursor.data, cursor.prev.data
				cursor = cursor.prev
			}
		case "RIGHT":
			if cursor.next != nil {
				cursor.next.data, cursor.data = cursor.data, cursor.next.data
				cursor = cursor.next
			}
		case "BACKSPACE":
			if cursor.prev != nil {
				if cursor.prev.prev != nil {
					cursor.prev.prev.next = cursor
				}
				cursor.prev = cursor.prev.prev
			}
		default:
			panic("Invalid command provided: " + line)
		}
	}
	cursor.Print()
}
