package main

import (
	"context"
	"math/big"
	"strconv"

	rpc "github.com/centrifuge/go-substrate-rpc-client/v4/gethrpc"

	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/common/hexutil"
)

type EthService struct{}

func (EthService) ChainId() *hexutil.Big {
	return (*hexutil.Big)(big.NewInt(int64(2001)))
}

func (EthService) BlockNumber(_ context.Context) (hexutil.Uint64, error) {
	return hexutil.Uint64(0), nil
}

func (EthService) GetBalance(_ common.Address, blockNumber string) (hexutil.Uint64, error) {
	return hexutil.Uint64(0), nil
}

type NetService struct{}

func (NetService) Version() string {
	return strconv.Itoa(int(0))
}

var apis = []rpc.API{
	{Namespace: "eth", Version: "", Service: new(EthService), Public: true},
	{Namespace: "net", Version: "", Service: new(NetService), Public: true},
}
