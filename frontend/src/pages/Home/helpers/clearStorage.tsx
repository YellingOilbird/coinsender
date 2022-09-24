import {useEffect} from "react";

export const useClearStorage = () => {
   useEffect(() => {
      localStorage.removeItem("vault");
      localStorage.removeItem("token_id");
      localStorage.removeItem("coin");
      localStorage.removeItem("token_ticker");
      localStorage.removeItem("token_decimals");
      localStorage.removeItem("need_to_fund");
      localStorage.removeItem("chunkProcessed");
      localStorage.removeItem("chunkProcessedIndex");
      localStorage.removeItem("remaining_counter");
      localStorage.removeItem("transition");

      // localStorage.removeItem("need_to_deposit_ui");
      localStorage.removeItem("operations");
      localStorage.removeItem("total");
      localStorage.removeItem("remaining");
      // localStorage.removeItem("need_to_send");
      // localStorage.removeItem("need_to_deposit");
   }, []);
}