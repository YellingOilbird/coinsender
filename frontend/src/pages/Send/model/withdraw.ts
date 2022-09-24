export const withdraw = async () => {
   const contract = window.contract;
   const gas = window.gas;
   const token_id = localStorage.getItem("token_id");

   try {
      if (token_id === 'NEAR') {
         await contract.withdraw_all({
         }, gas, "1");
      } else {
         await contract.withdraw_all({
            token_id: token_id
         }, gas, "1");
      }

   } catch(e) {
      alert(
         "ERR_CATHING_WITHDRAW_ACTION \n" +
         "check your browser console for more info!"
      );
      console.log(e)
   } finally {
      alert(
         "WITHDRAW COMPLETE! ENJOY STATS!\n" +
         "click OK"
      )

      localStorage.setItem("vault", 'true');
   }
}
