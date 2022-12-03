package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"sort"
	"strconv"
)

func main() {
	file, reader := readFile("input.txt")
	defer file.Close()

	processInput(reader)
}

func readFile(inputFile string) (*os.File, *bufio.Scanner) {
	file, err := os.Open(inputFile)
	if err != nil {
		log.Fatal(err)
	}
	return file, bufio.NewScanner(file)
}

func getElves(scanner *bufio.Scanner) map[int]int {
	elveIndex := 0
	elves := map[int]int{}

	for scanner.Scan() {
		line := scanner.Text()
		if err := scanner.Err(); err != nil {
			log.Fatal(err)
		}

		_, ok := elves[elveIndex]
		if !ok {
			elves[elveIndex] = 0
		}

		if len(line) == 0 {
			elveIndex++
			continue
		}

		calories, _ := strconv.Atoi(line)
		elves[elveIndex] += calories
	}
	return elves
}

func processInput(scanner *bufio.Scanner) {
	elves := getElves(scanner)

	keys := make([]int, len(elves))
	for index, calories := range elves {
		keys[index] = calories
	}
	sort.Slice(keys, func(a, b int) bool {
		return a > b
	})

	fmt.Printf("Part-1: The top elve carried %v calories\n", keys[0])

	totalSum := keys[0] + keys[1] + keys[2]
	fmt.Printf("Part-2: the top three elves carried a sum of %v calories\n", totalSum)
}
