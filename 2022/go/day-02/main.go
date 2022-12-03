package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strings"
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

func processInput(scanner *bufio.Scanner) {
	sumPickScore := 0
	sumPickScoreAdjusted := 0

	for scanner.Scan() {
		line := scanner.Text()
		if err := scanner.Err(); err != nil {
			log.Fatal(err)
		}

		// AX: Rock (1)
		// BY: Paper (2)
		// CZ: Scissors (3)
		// Win: 6
		// Draw: 3
		// Loss: 0
		pickMap := map[string]int{
			"A Y": 2 + 6,
			"A X": 1 + 3,
			"A Z": 3 + 0,
			"B Y": 2 + 3,
			"B X": 1 + 0,
			"B Z": 3 + 6,
			"C Y": 2 + 0,
			"C X": 1 + 6,
			"C Z": 3 + 3,
		}

		// part1
		sumPickScore += pickMap[line]

		// part2
		picks := strings.Split(line, " ")
		wantedResult := getWantedResult(picks)
		sumPickScoreAdjusted += pickMap[fmt.Sprintf("%v %v", picks[0], wantedResult)]
	}

	fmt.Printf("Part-1: Your score in rock, paper, scissors against the elves was %v \n", sumPickScore)
	fmt.Printf("Part-2: Your score in rock, paper, scissors when adjusting your picks based on the secret code was %v \n", sumPickScoreAdjusted)
}

// Picks based on the opponents pick first.
// AX: Rock (1)
// BY: Paper (2)
// CZ: Scissors (3)
// Y: draw
// X: loss
// Z: win
func getWantedResult(picks []string) string {
	wantedResult := picks[1]

	switch picks[0] {
	case "A":
		if wantedResult == "X" {
			return "Z"
		}
		if wantedResult == "Y" {
			return "X"
		}
		if wantedResult == "Z" {
			return "Y"
		}
	case "B":
		if wantedResult == "X" {
			return "X"
		}
		if wantedResult == "Y" {
			return "Y"
		}
		if wantedResult == "Z" {
			return "Z"
		}
	case "C":
		if wantedResult == "X" {
			return "Y"
		}
		if wantedResult == "Y" {
			return "Z"
		}
		if wantedResult == "Z" {
			return "X"
		}
	}

	return "undefined-case"
}
