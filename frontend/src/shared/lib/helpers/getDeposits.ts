export const getDeposits = async (coin:string) => {

   if(coin === 'NEAR') {
      const deposit = await window.contract.get_user_deposit_by_token({account_id:window.accountId});

      return (deposit)
   } else {
      const deposit = await window.contract.get_user_deposit_by_token({account_id:window.accountId, token_id:coin});

      return deposit
   }
}