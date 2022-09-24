import { connect, WalletConnection, Contract } from 'near-api-js'
import {nearConfig} from './utils'
import {Near, } from "near-api-js/lib/near";
import {CONTRACT_ID} from "shared/config";

declare global {
   interface Window {
      walletConnection:any;
      accountId:string | undefined;
      nearInitPromise:Promise<void>;
      near:Near;
      contract:any;
      contractFT:any;
      gas:number;
   }
}

export async function initContract() {
   window.near = await connect(nearConfig)

   window.walletConnection = new WalletConnection(window.near, 'coinSender')

   window.accountId = window.walletConnection.getAccountId();

   window.contract = new Contract(window.walletConnection.account(), CONTRACT_ID, {
      viewMethods: ['get_whitelisted_tokens','get_user_vault', 'get_user_deposit_by_token', 'get_donate_destinations'],
      changeMethods: ['multi_storage_deposit','send_from_balance_unsafe','send_from_balance','deposit_near', 'withdraw_all', 'transfer_near_to_contract'],
   })

   window.contractFT = (token_id:string) => {
      return new Contract(window.walletConnection.account(), token_id, {
         viewMethods: ['storage_balance_of', 'get_user_deposit_by_token', 'get_deposit_by_token'],
         changeMethods: ['storage_deposit', 'ft_transfer_call']
      })
   }

   // helpers

   window.gas =300000000000000;
}