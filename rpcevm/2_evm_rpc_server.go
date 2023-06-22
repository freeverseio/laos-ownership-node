package main

import (
	"fmt"
	"os"
	"os/signal"
	"syscall"

	rpc "github.com/centrifuge/go-substrate-rpc-client/v4/gethrpc"

	"github.com/ethereum/go-ethereum/log"
)

func runServer() error {
	// Define other parameters
	endpoint := ":8545"
	modules := []string{}
	cors := []string{"*"}
	vhosts := []string{"localhost"}
	timeouts := rpc.DefaultHTTPTimeouts

	log.Root().SetHandler(log.LvlFilterHandler(log.LvlDebug, log.StreamHandler(os.Stderr, log.TerminalFormat(true))))
	// Start the HTTP endpoint
	listener, handler, err := rpc.StartHTTPEndpoint(endpoint, apis, modules, cors, vhosts, timeouts)
	if err != nil {
		return fmt.Errorf("Error starting HTTP endpoint: %v", err)
	}

	// Log that the server is running
	log.Info("HTTP RPC server running", "endpoint", endpoint)

	// Wait for an interrupt signal to gracefully shutdown the server
	sigs := make(chan os.Signal, 1)
	signal.Notify(sigs, syscall.SIGINT, syscall.SIGTERM)
	<-sigs

	// Close the listener
	if err := listener.Close(); err != nil {
		return fmt.Errorf("Error closing listener: %v", err)
	}
	// Stop server
	handler.Stop()

	fmt.Println("HTTP RPC server shut down gracefully")
	return nil
}
