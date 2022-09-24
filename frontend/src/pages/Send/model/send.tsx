import {ConvertToYocto} from "shared/lib/helpers/convert";
import bigInt from "big-integer";

export const send_unsafe = (coinPathname:string) => async () => {
   const coin = coinPathname.split('/')[3]

   let contract = window.contract;
   let gas = window.gas;

   // get token data
   let token_id = localStorage.getItem("token_id");
   let decimals = localStorage.getItem("token_decimals");
   // chunks because of limit per transaction - 300 Tgas (~ 30 transfers)
   let chunkSize = 25;

   let remaining = localStorage.getItem("remaining");
   let operations = localStorage.getItem("operations");

   try {
      let remaining_accounts = JSON.parse(remaining ? remaining : '[]');
      let preparedAccounts:any[] = [];
      if (remaining_accounts.length === 0) {
         // get account-amount recipients
         let accountsRaw = JSON.parse(operations ? operations : '[]');
         //collect JSON to Array
         for (let key in accountsRaw) {
            const amount = ConvertToYocto(accountsRaw[key], Number(decimals))
            console.log(amount)
            preparedAccounts.push({account_id: key, amount: bigInt(amount ? amount : '0')!.toString() });
         }
      } else {
         preparedAccounts = remaining_accounts;
      }

      // chunk Number(chunkSize) accounts from remaining recipients
      const chunks = preparedAccounts.reduce(function (result:any, value:any, index:number, array:any[]) {
         if (index % chunkSize === 0) {
            const max_slice = Math.min(index + chunkSize, preparedAccounts.length);
            result.push(array.slice(index, max_slice));
         }

         return result;
      }, []);

      await (chunks).reduce(
         async(promise:any, chunk:{[key: string]:string, amount:string}[], index:number) => {
            return promise.then(async (last:any) => {
               const ret = last + 100;
               const max_slice = Math.min((index + 1) * chunkSize, preparedAccounts.length);
               const remaining_accounts = preparedAccounts.slice(max_slice);
               localStorage.setItem("remaining_counter", remaining_accounts.length.toString());
               localStorage.setItem("remaining", remaining_accounts ? JSON.stringify(remaining_accounts) : "{}");
               localStorage.setItem("chunk", JSON.stringify(chunk));

               if (token_id === 'NEAR') {
                  await new Promise(async (res, rej) => {
                     console.log(chunk)
                     await contract.send_from_balance_unsafe({
                        accounts: chunk
                     }, gas, "1");
                     return setTimeout(res, 1000);
                  });
               } else {
                  await new Promise(async (res, rej) => {
                     await contract.send_from_balance_unsafe({
                        accounts: chunk,
                        token_id: token_id
                     }, gas, "1");
                     return setTimeout(res, 1000);
                  });
               }

               return ret;
            })
         }, Promise.resolve(0)).then(() => {
         alert(
            'Send to 25 accounts complete! \n' +
            'See transaction in explorer: https://explorer.mainnet.near.org/accounts/coinsender.near  \n' +
            'Click OK to continue.\n'
         );
         localStorage.setItem("vault", 'true');

         // window.location.href = `https://${contractName}.page/processing/finality/${wallet.getAccountId()}`;
      });
   } catch (e) {
      alert(
         'Something went wrong! \n' +
         'Check your browser console for more info.\n' +
         (e as Error).toString()
      )
      throw e
   } finally {
      alert(
         "SEND COMPLETE! ENJOY STATS! Click OK"
      );

      localStorage.setItem("vault", 'true');
   }
}