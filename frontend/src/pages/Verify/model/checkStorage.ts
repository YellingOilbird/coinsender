import bigInt from "big-integer";
import {verifyAcoounts} from "./getAccouns";

export const checkStorage = (
   accounts: string | undefined,
   setStatus: React.Dispatch<React.SetStateAction<string>>,
   setAccounts: React.Dispatch<React.SetStateAction<string>>,
   coin:string
) => async () => {

   let contract = window.contract;
   let contractFT = window.contractFT;
   const gas = window.gas;
   const operations = localStorage.getItem("operations")

   let operationsArr = JSON.parse(operations ? operations : '[]');
   let checkedAccounts = Object.keys(operationsArr);
   const token_id = localStorage.getItem('token_id');
   // get massive of chunked accounts

   let chunksAccountsFunded = async() => {

      if (checkedAccounts.length > 0) {
         const groupSize = 20;
         let groupIndex = -1;
         let accountGroups:any[] = [];
         let total_already_registered = 0;
         let fundedAccounts:any[] = [];
         for (let i = 0; i < checkedAccounts.length; i++) {
            if (i % groupSize === 0) {
               groupIndex++;
               accountGroups[groupIndex] = [];
            }
            accountGroups[groupIndex].push(checkedAccounts[i])
         }
         let index = 0;
         // StorageBalance retrieve
         while (index < accountGroups.length) {
            let storageCheckAccountGroup = async () => {
               return await Promise.all(accountGroups[index].map(async (account:any) => {
                  let registered = await contractFT(coin).storage_balance_of({account_id: account});
                  console.log(registered)
                  if (registered) {
                     total_already_registered += 1;
                     console.log("Registered account: " + account);
                  } else {
                     return account
                  }
               }));
            }
            await storageCheckAccountGroup().then((nonFundedAccounts) => {
               Object.values(nonFundedAccounts).map((account) => {
                  if (account) {
                     console.log(account);
                     fundedAccounts.push(account);
                  }
               });
               index++;
            })
         }//end of while loop
         // If we got some non funded accounts collect them
         localStorage.setItem("need_to_fund", String(fundedAccounts.length));
         let fundedAccountsChunks = [];

         if (fundedAccounts.length > 0) {
            if (fundedAccounts.length > groupSize) {
               let fundedAccountsSlice = fundedAccounts.slice(0, groupSize);
               fundedAccountsChunks.push(fundedAccountsSlice);
               console.log("sliced+: "+fundedAccountsSlice.length+" accs : ",fundedAccountsChunks);
            } else {
               fundedAccountsChunks.push(fundedAccounts);
               console.log("sliced-: "+fundedAccounts.length+" accs : ",fundedAccountsChunks);
            }
            // Go to deposit if no more accounts to fund
         } else {
            fundedAccountsChunks = [];
         }

         return fundedAccountsChunks;
      }//if we have any accounts

   }// end chunkSlicer fn

   try {
      // TODO wrap in promise - await
      // processing slices from chunk
      await chunksAccountsFunded().then((async (fundedAccountsChunks) => {
            //unneccesary double checks for a while
            const i = 0;
   
            if (fundedAccountsChunks!.length > 0) {
               console.log("______accs_____",fundedAccountsChunks![i].length);
               console.log("[]...[]length :",fundedAccountsChunks!.length);
               const groupSize = 20;
               // N accounts
   
               // here comes [] massive[accounts](N)
               // need to fund = N (groupSize or groupSize >= last chunk >=0)
               if (fundedAccountsChunks![i].length <= groupSize) {
                  let total_funded_required = bigInt(fundedAccountsChunks![i].length).multiply(bigInt("1250000000000000000000")).toString();
                  await contract.multi_storage_deposit({
                           accounts: fundedAccountsChunks![i],
                           token_id: token_id
                        }, gas, total_funded_required
                  );
                  console.log("processed"+ fundedAccountsChunks![i] +"accounts");
                  let ntf = Number(localStorage.getItem("need_to_fund"));
                  localStorage.setItem("need_to_fund", (ntf - fundedAccountsChunks![i]!.length).toString());
   
                  if (Number(ntf - fundedAccountsChunks![i].length) === 0) {
                     localStorage.setItem("transition", '1');
                     console.log("fin");
                  }
                  setStatus('DEPOSIT');
               } else {
                  console.log("SOMETHING WRONG WITH ACCOUNT SLICES! see slice in console! \n");
               }
            } else {
               console.log("set");
               verifyAcoounts(
                  accounts,
                  setStatus,
                  setAccounts,
                  () => {},
                  coin
               )()
            }
         }));
   } catch (e) {
      setStatus('VERIFY')
      throw e
   } finally {
      setStatus('DEPOSIT');
   }
}

/*

*/
