package main

import (
	"os"

	"github.com/ethereum/go-ethereum/log"
)

func main() {
	if err := runServer(); err != nil {
		log.Error("Error running server", "error", err)
		os.Exit(1)
	}
}
