package main

import (
	"encoding/binary"
	"fmt"
	"math/big"

	gsrpc "github.com/centrifuge/go-substrate-rpc-client/v4"
	"github.com/centrifuge/go-substrate-rpc-client/v4/hash"
	"github.com/centrifuge/go-substrate-rpc-client/v4/signature"
	"github.com/centrifuge/go-substrate-rpc-client/v4/types"
)

const (
	arrakisOwnershipURL = "wss://arrakis.gorengine.com/own"
)

type substrateClient struct {
	api *gsrpc.SubstrateAPI
}

func NewSubstrateClient(url string) (*substrateClient, error) {
	api, err := gsrpc.NewSubstrateAPI(url)
	if err != nil {
		return nil, err
	}
	return &substrateClient{api: api}, nil
}

func (cli substrateClient) transfer() error {
	meta, err := cli.api.RPC.State.GetMetadataLatest()
	if err != nil {
		return err
	}

	// Create a call, transferring 12345 units to Bob
	bob, err := types.NewMultiAddressFromHexAccountID("0x8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48")
	if err != nil {
		return err
	}

	// 1 unit of transfer
	bal, ok := new(big.Int).SetString("100000000000000", 10) // 100
	if !ok {
		return fmt.Errorf("failed to convert balance")
	}

	c, err := types.NewCall(meta, "Balances.transfer", bob, types.NewUCompact(bal))
	if err != nil {
		return err
	}

	// Create the extrinsic
	ext := types.NewExtrinsic(c)

	genesisHash, err := cli.api.RPC.Chain.GetBlockHash(0)
	if err != nil {
		return err
	}

	rv, err := cli.api.RPC.State.GetRuntimeVersionLatest()
	if err != nil {
		return err
	}

	account, err := types.NewAccountIDFromHexString("0xe659a7a1628cdd93febc04a4e0646ea20e9f5f0ce097d9a05290d4a9e054df4e")
	if err != nil {
		return err
	}

	key, err := types.CreateStorageKey(meta, "System", "Account", account.ToBytes())
	if err != nil {
		return err
	}

	var accountInfo types.AccountInfo
	ok, err = cli.api.RPC.State.GetStorageLatest(key, &accountInfo)
	if err != nil {
		return err
	}

	if !ok {
		return fmt.Errorf("account not found")
	}

	nonce := uint32(accountInfo.Nonce)
	o := types.SignatureOptions{
		BlockHash:          genesisHash,
		Era:                types.ExtrinsicEra{IsMortalEra: false},
		GenesisHash:        genesisHash,
		Nonce:              types.NewUCompactFromUInt(uint64(nonce)),
		SpecVersion:        rv.SpecVersion,
		Tip:                types.NewUCompactFromUInt(100),
		TransactionVersion: rv.TransactionVersion,
	}

	keyringPair, err := signature.KeyringPairFromSecret("bottom drive obey lake curtain smoke basket hold race lonely fit walk//Eve", 2001)
	if err != nil {
		return err
	}

	err = ext.Sign(keyringPair, o)
	if err != nil {
		return err
	}

	// Send the extrinsic
	_, err = cli.api.RPC.Author.SubmitExtrinsic(ext)
	if err != nil {
		return err
	}

	fmt.Printf("Balance transferred from Eve to Bob: %v\n", bal.String())
	return nil
}

func (cli substrateClient) getOwnerOfCollection(collectionId uint64) (*types.AccountID, error) {
	meta, err := cli.api.RPC.State.GetMetadataLatest()
	if err != nil {
		return nil, err
	}

	collectionIdSerialized := make([]byte, 8)
	binary.LittleEndian.PutUint64(collectionIdSerialized, collectionId)

	// Hashing serialized collection id
	collectionIdHasher, err := hash.NewBlake2b128Concat(nil)
	if err != nil {
		return nil, err
	}
	_, err = collectionIdHasher.Write(collectionIdSerialized)
	if err != nil {
		return nil, err
	}

	key, err := types.CreateStorageKey(meta, "LivingassetsOwnership", "OwnerOfCollection", collectionIdSerialized)
	if err != nil {
		return nil, err
	}
	var accountId types.AccountID
	ok, err := cli.api.RPC.State.GetStorageLatest(types.NewStorageKey(key), &accountId)
	if err != nil {
		return nil, err
	}

	if !ok {
		return nil, nil
	}

	return &accountId, nil
}
