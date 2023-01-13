package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"unicode"
)

var priorities map[rune]int = make(map[rune]int)

func p1() {
	file, err := os.Open("data_1.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)
	total := 0
	for scanner.Scan() {
		line := scanner.Text()
		n := len(line) / 2
		c1 := line[:n]
		c2 := line[n:]
		
		out:
		for _, i1 := range c1 {
			for _, i2 := range c2 {
				if i1 == i2 {
					total += int(priorities[i1])
					break out
				}
			}
		}
	}
	fmt.Println(total)
}

func p2() {
	file, err := os.Open("data_2.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	reader := bufio.NewReader(file)
	total := 0

	read:
	for {
		lines := []string {}
		for i := 0; i < 3; i++ {
			line, _, err := reader.ReadLine()
			if err != nil || len(line) == 0 {
				break read
			}
			lines = append(lines, string(line))
		}

		out:
		for _, i1 := range lines[0] {
			for _, i2 := range lines[1] {
				if i1 == i2 {
					for _, i3 := range lines[2] {
						if i2 == i3 {
							total += int(priorities[i1])
							break out
						}
					}
				}
			}
		}
	}
	fmt.Println(total)
}

func main() {
	for i, r := 1, 'a'; r <= 'z'; i, r = i+1, r+1 {
		priorities[r] = i
		priorities[unicode.ToUpper(r)] = i + 26
	}
	p1()
	p2()
}