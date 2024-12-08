const { SigningCosmWasmClient } = require('@cosmjs/cosmwasm-stargate');
const { DirectSecp256k1HdWallet } = require('@cosmjs/proto-signing');
const { GasPrice } = require('@cosmjs/stargate');

// Parse command line arguments
const args = process.argv.slice(2).reduce((acc, arg) => {
  const [key, value] = arg.replace('--', '').split('=');
  acc[key] = value;
  return acc;
}, {});

const MNEMONIC = process.env.TESTNET_MNEMONIC;
const RPC_ENDPOINT = args.rpc || process.env.STARGAZE_TESTNET_RPC;
const CHAIN_ID = args['chain-id'] || process.env.STARGAZE_TESTNET_CHAIN_ID;
const CONTRACT_CODE_ID = args['contract-id'] || process.env.CONTRACT_CODE_ID;

if (!MNEMONIC) throw new Error('TESTNET_MNEMONIC is required');
if (!RPC_ENDPOINT) throw new Error('RPC endpoint is required');
if (!CHAIN_ID) throw new Error('Chain ID is required');
if (!CONTRACT_CODE_ID) throw new Error('Contract code ID is required');

async function main() {
  try {
    console.log('Starting integration tests with:');
    console.log('- RPC:', RPC_ENDPOINT);
    console.log('- Chain ID:', CHAIN_ID);
    console.log('- Contract Code ID:', CONTRACT_CODE_ID);

    // Setup wallet
    const wallet = await DirectSecp256k1HdWallet.fromMnemonic(MNEMONIC, {
      prefix: 'stars',
    });
    const [account] = await wallet.getAccounts();
    console.log('Using account:', account.address);

    // Setup client
    const client = await SigningCosmWasmClient.connectWithSigner(
      RPC_ENDPOINT,
      wallet,
      {
        gasPrice: GasPrice.fromString('0.025ustars'),
      }
    );

    // Instantiate contract
    console.log('\nInstantiating contract...');
    const { contractAddress } = await client.instantiate(
      account.address,
      parseInt(CONTRACT_CODE_ID),
      {
        owner: account.address,
        canvas_size: 100,
        pixel_price: '1000000', // 1 STARS
      },
      'Pixel Canvas Test Instance',
      'auto'
    );
    console.log('Contract instantiated at:', contractAddress);

    // Test pixel purchase
    console.log('\nTesting pixel purchase...');
    const buyResult = await client.execute(
      account.address,
      contractAddress,
      {
        buy_pixel: {
          x: 0,
          y: 0,
        },
      },
      'auto',
      'Buy pixel test',
      [{ denom: 'ustars', amount: '1000000' }]
    );
    console.log('Pixel purchase result:', buyResult.transactionHash);

    // Test pixel color update
    console.log('\nTesting pixel color update...');
    const updateResult = await client.execute(
      account.address,
      contractAddress,
      {
        set_pixel_color: {
          x: 0,
          y: 0,
          color: '#FF0000',
        },
      },
      'auto'
    );
    console.log('Color update result:', updateResult.transactionHash);

    // Query pixel state
    console.log('\nQuerying pixel state...');
    const pixel = await client.queryContractSmart(contractAddress, {
      get_pixel: {
        x: 0,
        y: 0,
      },
    });
    console.log('Pixel state:', pixel);

    // Verify pixel state
    if (pixel.owner !== account.address || pixel.color !== '#FF0000') {
      throw new Error('Pixel state verification failed');
    }

    console.log('\n✅ All integration tests passed! 🎉');
    process.exit(0);
  } catch (error) {
    console.error('\n❌ Integration tests failed:', error);
    process.exit(1);
  }
}

main(); 