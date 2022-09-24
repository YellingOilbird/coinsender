import {Account} from 'near-api-js'

export async function accountExists(accountId:string) {

   try {
      await new Account(window.near.connection, accountId).state();
      return true;
   } catch (error) {
      console.log(error)
      return false;
   }
}