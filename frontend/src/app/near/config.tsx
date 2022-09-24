import { keyStores } from 'near-api-js'

function getConfig(env:string = 'mainnet', contract:string) {

   switch (env) {

      case 'production':
      case 'mainnet':
         return {
            networkId: 'mainnet',
            nodeUrl: 'https://rpc.mainnet.near.org',
            contractName: contract,
            walletUrl: 'https://wallet.near.org',
            helperUrl: 'https://helper.mainnet.near.org',
            explorerUrl: 'https://explorer.mainnet.near.org',
            keyStore: new keyStores.BrowserLocalStorageKeyStore()
         }
      case 'development':
      case 'testnet':
         return {
            contractName: contract,
            networkId: 'testnet',
            keyStore: new keyStores.BrowserLocalStorageKeyStore(),
            nodeUrl: "https://rpc.testnet.near.org",
            walletUrl: "https://wallet.testnet.near.org",
            helperUrl: "https://helper.testnet.near.org",
            explorerUrl: "https://explorer.testnet.near.org",
         }
      default:
         throw Error(`Unconfigured environment '${env}'. Can be configured in src/config.js.`)
   }
}

export default getConfig
