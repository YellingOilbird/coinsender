import {Coins, CoinsWrap} from "pages/Home/Home.styled";
import {token} from "shared/config/type";
import {useAppSelector} from "app/model/hooks";
import near from 'shared/lib/assets/img/near.svg'

export const WhitedTokens = () => {
   const whited_list_tokens = useAppSelector(state => state.tokens.tokens)

   return(
      <CoinsWrap>
         <Coins>
            <p>NEAR</p>
            <img
               width='40px'
               src={near}
               alt=''
            />
         </Coins>
         {whited_list_tokens.length > 0 &&
            whited_list_tokens.map((token:token) => (
               <Coins key={token[1].ticker}>
                  <p>{token[1].ticker}</p>
                  <img
                     width='40px'
                     src={token[1].icon}
                     alt=''
                  />
               </Coins>
            ))
         }
      </CoinsWrap>
   )
}