package main

import (
	"bufio"
	"log"
	"os"
	"sort"
	"strconv"
)

func p1() {
	file, err := os.Open("data.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)
	count := 0
	max := 0
	for scanner.Scan() {
		line := scanner.Text()
		calories, err := strconv.Atoi(line)
		if err != nil {
			count = 0
			continue
		}
		count += calories
		if count > max {
			max = count
		}
	}
	println(max)
}

func p2() {
	file, err := os.Open("data.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)
	totals := []int { 0 }
	index := 0
	for scanner.Scan() {
		line := scanner.Text()
		calories, err := strconv.Atoi(line)
		if err != nil {
			totals = append(totals, 0)
			index = len(totals) - 1
			continue
		}
		totals[index] += calories
	}
	sort.Ints(totals)
	total := 0
	for i := len(totals) - 3; i < len(totals); i++ {
		total += totals[i]
	}
	println(total)
}

func main() {
	p1()
	p2()
}
