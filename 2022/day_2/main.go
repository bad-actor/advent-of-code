package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strings"
)

var scores map[string]int = map[string]int {
	"A": 1,
	"B": 2,
	"C": 3,
	"X": 1,
	"Y": 2,
	"Z": 3,
}

func p1() {
	file, err := os.Open("data.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)
	score := 0
	for scanner.Scan() {
		line := scanner.Text()
		moves := strings.Fields(line)
		o := scores[moves[0]]
		i := scores[moves[1]]
		var w int
		if (i-o % 3 == 1) {
			w = 6;
		} else if (i == o) {
			w = 3;
		} else {
			w = 0;
		}
		score += i + w;
	}
	fmt.Println(score)
}

func p2() {
 file, err := os.Open("data.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)
	score := 0
	for scanner.Scan() {
		line := scanner.Text()
		moves := strings.Fields(line)
		o := scores[moves[0]]
		var i, w int
		switch moves[1] {
		case "X":
			w = 0
			i = (o + 1) % 3 + 1
		case "Y":
			w = 3
			i = o
		case "Z":
			w = 6
			i = o % 3 + 1
		}
		score += i + w;
	}
	fmt.Println(score)
}

func main() {
	p1()
	p2()
}