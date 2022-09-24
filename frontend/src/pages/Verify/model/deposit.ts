import {CONTRACT_ID} from "shared/config";

export const deposit = (setStatus: React.Dispatch<React.SetStateAction<string>>, coin:string) => async () => {
   let getCountDeposit = localStorage.getItem("need_to_deposit");
   let getCountDepositUi = localStorage.getItem("need_to_deposit_ui");
   const token_id = localStorage.getItem("token_id");

   try {
      localStorage.removeItem("need_to_deposit");
      localStorage.removeItem("need_to_deposit_ui");

      localStorage.setItem("need_to_send", String(getCountDeposit));
      localStorage.setItem("need_to_send_ui", String(getCountDepositUi));

      if (token_id === 'NEAR') {
         let contract = window.contract;
         await contract.deposit_near({}, window.gas, getCountDeposit)
      } else {
         let contractFT = window.contractFT;

         console.log(getCountDeposit)
         await contractFT(coin).ft_transfer_call({
            receiver_id: CONTRACT_ID,
            amount: getCountDeposit,
            msg:"deposit"
         }, window.gas, "1")
      }

   } catch (er) {
      console.log(er)

      localStorage.removeItem("need_to_deposit");
      localStorage.removeItem("need_to_deposit_ui");
   }
}