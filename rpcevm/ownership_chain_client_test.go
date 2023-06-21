package main_test

import (
	"encoding/binary"
	"fmt"
	"testing"

	gsrpc "github.com/centrifuge/go-substrate-rpc-client/v4"
	"github.com/centrifuge/go-substrate-rpc-client/v4/hash"
	"github.com/centrifuge/go-substrate-rpc-client/v4/types"
	"gotest.tools/assert"
)

func TestOwnershipChainStorage(t *testing.T) {
	api, err := gsrpc.NewSubstrateAPI("wss://arrakis.gorengine.com/own")
	assert.NilError(t, err)

	meta, err := api.RPC.State.GetMetadataLatest()
	assert.NilError(t, err)

	t.Run("key generated with centrifuge client is the same as the one provided by the ownership node", func(t *testing.T) {
		keyFromPolkadotJS := "0x7422b9c0c7299b691c37b32a5f37c45be1721fc45417f59f7c0445592822e4379ea2d098b5f70192f96c06f38d3fbc970100000000000000"

		// Serialize collection id
		var collectionId uint64 = 1
		collectionIdSerialized := make([]byte, 8)
		binary.LittleEndian.PutUint64(collectionIdSerialized, collectionId)

		// Hashing serialized collection id
		collectionIdHasher, err := hash.NewBlake2b128Concat(nil)
		assert.NilError(t, err)
		_, err = collectionIdHasher.Write(collectionIdSerialized)
		assert.NilError(t, err)

		assert.Equal(t, "0x0100000000000000", fmt.Sprintf("%#x", collectionIdSerialized))

		key, err := types.CreateStorageKey(meta, "LivingassetsOwnership", "OwnerOfCollection", collectionIdSerialized)
		assert.NilError(t, err)
		assert.Equal(t, keyFromPolkadotJS, key.Hex())
	})
}
