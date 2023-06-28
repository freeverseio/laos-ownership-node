import Web3 from 'web3';
import HDWalletProvider from '@truffle/hdwallet-provider';

// Private key of the sender account
const privateKey = '0xa98c8730d71a46bcc40fb06fc68142edbc2fdf17b89197db0fbe41d35718d5fc';
// const privateKey = '0x5fb92d6e98884f76de468fa3f6278f8807c48bebc13595d45af5bdc4da702133';

// Recipient's Ethereum address
const recipientAddress = '0xf24FF3a9CF04c71Dbc94D0b566f7A27B94566cac';

// Connection URL of your Ethereum node
const providerURL = 'http://127.0.0.1:8545';

const provider = new HDWalletProvider({
    privateKeys: [privateKey],
    providerOrUrl: providerURL
});

const web3 = new Web3(provider);

async function transferEth(senderAddress, recipientAddress, amountEth) {
    const amountWei = web3.utils.toWei(amountEth, 'ether');

    const transactionParameters = {
        to: recipientAddress,
        from: senderAddress,
        value: amountWei
    };

    // Estimate gas limit
    const gas = await web3.eth.estimateGas(transactionParameters);
    transactionParameters['gas'] = gas;

    // Send transaction
    const tx = await web3.eth.sendTransaction(transactionParameters);
    return tx;
}

const senderAddress = provider.getAddress(0); // Address of the first account derived from the private key
const amountEth = '0.1'; // Amount of Ether to send

transferEth(senderAddress, recipientAddress, amountEth)
    .then(tx => console.log(tx))
    .catch(err => console.error(err))
    .finally(() => provider.engine.stop()); // Stop the provider engine
