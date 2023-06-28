package main

import (
	"context"
	"fmt"
	"math/big"
	"strconv"

	rpc "github.com/centrifuge/go-substrate-rpc-client/v4/gethrpc"

	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/common/hexutil"
	ethtypes "github.com/ethereum/go-ethereum/core/types"
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

func (EthService) GetBlockByNumber(_blockNumber string, _fullTx bool) (*ethtypes.Block, error) {
	return &ethtypes.Block{}, nil
}
func (EthService) EstimateGas(ctx context.Context, args map[string]interface{}, blockNrOrHash *rpc.BlockNumberOrHash) (hexutil.Uint64, error) {
	return hexutil.Uint64(0), nil
}
func (EthService) GasPrice() (*hexutil.Big, error) {
	return &hexutil.Big{}, nil
}
func (EthService) GetTransactionCount(_address common.Address, _blockNrOrHash rpc.BlockNumberOrHash) (*hexutil.Uint64, error) {
	return nil, nil
}
func (EthService) SendRawTransaction(_input hexutil.Bytes) (common.Hash, error) {
	return common.Hash{}, nil
}
func (EthService) GetTransactionReceipt(ctx context.Context, hash common.Hash) (map[string]interface{}, error) {
	return nil, nil
}

type Transaction struct {
	Data string
	To   string
}

func (EthService) Call(t Transaction, blockNumber string) (hexutil.Bytes, error) {
	fmt.Println(t.Data)

	return hexutil.Bytes{}, nil
}

type NetService struct{}

func (NetService) Version() string {
	return strconv.Itoa(int(0))
}

var apis = []rpc.API{
	{Namespace: "eth", Version: "", Service: new(EthService), Public: true},
	{Namespace: "net", Version: "", Service: new(NetService), Public: true},
}
