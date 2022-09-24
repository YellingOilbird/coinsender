import {useAppDispatch, useAppSelector} from "app/model/hooks";
import {token} from "shared/config/type";
import {setToken} from "app/model/slice/tokens";

export const ChangeToken = () => {
   const {tokens} = useAppSelector(state => state.tokens)
   const dispatch = useAppDispatch()

   const onSetCoin = (ev:any) => {
      dispatch(setToken(ev.target.value.split(':')));
   }

   return(
      <select id="token_select" onChange={onSetCoin}>
         <option
            value="NEAR:24:NEAR"
         >
            NEAR
         </option>
         {tokens.length > 0 &&
            tokens.map((token:token) => (
               <option
                  key={token[0]}
                  value={`${token[1].contract_id}:${token[1].decimals}:${token[1].ticker}`}
               >
                  {token[1].ticker}
               </option>
            ))
         }
      </select>
   )
}