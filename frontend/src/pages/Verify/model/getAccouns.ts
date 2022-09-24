import bigInt from "big-integer";
import {ConvertToYocto} from "../../../shared/lib/helpers/convert";
import {accountExists} from "../../../shared/lib/helpers/accountExits";
import {getDeposits} from "../../../shared/lib/helpers/getDeposits";

interface returnFn {
   total:number;
   accounts:Record<string, number>
}

export const verifyAcoounts = (
   value: string | undefined,
   setStatus: React.Dispatch<React.SetStateAction<string>>,
   SetAccounts: React.Dispatch<React.SetStateAction<string>>,
   setError: React.Dispatch<React.SetStateAction<string>>,
   coin:string,
   isStorage:boolean = false,
) => async ():Promise<returnFn | false> => {
   console.log('start_verify');
   let decimals = localStorage.getItem("token_decimals");
   

   let accounts:Record<string, number> = {};
   let result;
   let total = 0;

   if(value !== '' && value) {
      let input = value;
      const pattern = RegExp(/^([0-9a-zA-Z][\_\-0-9a-zA-Z.]*)[\t,|\||=| ]([0-9\.\,]+$)/, 'gm');

      while ((result = pattern.exec(input)) !== null) {
         const account_name = result[1].toLowerCase();
         const amount = parseFloat(result[2].replace(',', '.').replace(' ', ''))
         const isAcc = await accountExists(account_name);

         if (result[1] && amount && isAcc) {
            if (accounts.hasOwnProperty(account_name)) {
               accounts[account_name] += amount;
            } else
               accounts[account_name] = amount;
               total += amount;
         }
      }

      const deposit = await getDeposits(coin);
      total += 0.01;
      const totalConvert = ConvertToYocto(total, Number(decimals))
      const totalBigAmount = bigInt(totalConvert ? totalConvert : '0').toString();

      if(Number(deposit) >= Number(totalBigAmount) ) {
         setStatus('SEND');
      } else {
         setStatus('DEPOSIT');
         
         console.log(totalBigAmount + ' big amount');

         const need_to_deposit = totalBigAmount;

         console.log(need_to_deposit + ' deposit')
         console.log(total.toString() + ' deposit_ui')

         localStorage.setItem('need_to_deposit', need_to_deposit ? need_to_deposit : '')
         localStorage.setItem('need_to_deposit_ui', total.toString())
      }

      const operations = JSON.stringify(accounts);
      localStorage.setItem('operations',operations)
      localStorage.setItem('operations_number', Object.keys(accounts).length.toString())

      if(coin !== 'NEAR' && isStorage) {
         setStatus('STORAGE')
      }
   } else setError('There are no accounts')
   // set this to input with SetAccounts
   if (accounts.length > 0) {
      console.log(accounts);
      SetAccounts(JSON.stringify(accounts));
   }
   return {total, accounts}
}