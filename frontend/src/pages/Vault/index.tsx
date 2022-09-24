import {useEffect, useState} from "react";
import {background, Heading} from "@chakra-ui/react";
import {
   Table,
   Tbody,
   Tr,
   Th,
   Td,
   TableContainer,
} from '@chakra-ui/react'
import {utils} from 'near-api-js'
import bigInt from "big-integer";

const Vault = () => {
   const [user_vault, setsVault] = useState<Record<string, any> | false>(false);
   const [token_tickers, setTickers] = useState<Record<string, string> | false>(false);
   const [token_decimals, setDecimals] = useState<Record<string, string> | false>(false);

   const [storageSpended, setStorageSpended] = useState('');
   const [nearDeposit, setNearDeposit] = useState('');


   function getTicker(token: string) {
      if (token_tickers) {
         return token_tickers[token];
      }
   }

   function getDecimals(token: string) {
      if (token_decimals) {
         return token_decimals[token];
      }
   }

   function getConvertedAmount(token: string, amount: string) {
      let decimals = getDecimals(token);
      if (typeof decimals !== undefined) {
         let decimals_number = Number(decimals);
         if (decimals_number >= 6) {
            console.log(amount);
            let amount_converted = parseFloat(bigInt(amount).divide(bigInt(Math.pow(10, (decimals_number - 4)))).toString());
            return (amount_converted / 10000).toFixed(2)
         }
      }
   }

   useEffect(() => {
      async function getVault() {
         const vault = await window.contract.get_user_vault({account_id:window.accountId});
         const wl_tokens = await window.contract.get_whitelisted_tokens();

         const tickers = wl_tokens.reduce(function(map: Record<string, string>, obj: any) {
            map[obj[0]] = obj[1].ticker;
            return map;
         }, {});

         const decimals = wl_tokens.reduce(function(map: Record<string, string>, obj: any) {
            map[obj[0]] = obj[1].decimals;
            return map;
         }, {});

         let storageDepositedTotal = utils.format.formatNearAmount(vault.spended_storage);
         setStorageSpended(storageDepositedTotal);

         let nearDepositedParsed = utils.format.formatNearAmount(vault.total_near_amount_sent);
         setNearDeposit(nearDepositedParsed);

         setsVault(vault)
         setTickers(tickers)
         setDecimals(decimals)
      }

      getVault()

   }, [])
   return(
      <div style={{marginTop:"150px", backgroundColor:'fbfbfe'}}>
         <h1 style={{fontSize:'32px', textAlign:'center'}}>🏧    VAULT   🏧</h1>

         <TableContainer mt={'40px'}>
            <Table variant='unstyled'>
               <Tbody>
                  {user_vault &&
                  <>
                     <Tr>Ⓝ NEAR</Tr>
                     <Tr>
                        <Th>
                           Number of <a>Ⓝ</a> sends
                           <br/>
                           (25 accounts per time)
                        </Th>
                        <Td>{user_vault?.near_sends_num}</Td>
                     </Tr>
                     <Tr>
                        <Th>total <a>Ⓝ</a> sended</Th>
                        <Td>{nearDeposit}</Td>
                     </Tr>
                     <Tr> 
                        <Th isNumeric style={{textAlign:"left", color:"#d44f4f"}}>Spended <a>Ⓝ</a> for recipients storage</Th>  
                        <Td style={{color:"#d44f4f"}}>{
                           storageSpended
                        }</Td>
                     </Tr>
                     <br></br>
                     <Tr><i className="nes-icon coin is-small"/> FUNGIBLE TOKENS</Tr>
                     <br></br>
                     <Tr>
                        <Th>
                           Number of token sends
                           <br/>
                           (25 accounts per time)
                        </Th>
                        <Td>{user_vault?.ft_sends_num}</Td>
                     </Tr>
                     {/* <Tr>   
                        <Th>tokens used</Th>
                        <Td>{user_vault.tokens_used.map((token:string) => token)}</Td>
                     </Tr> */}
                     <Tr>
                        <Th isNumeric style={{textAlign:"left"}}>total tokens sended</Th>
                        <Td>{user_vault.total_ft_amount_sent.map((tokens:string[], index:number) => (
                              <>
                                 {getTicker(user_vault.total_ft_amount_sent[index][0])}: 
                                 {getConvertedAmount(user_vault.total_ft_amount_sent[index][0], user_vault.total_ft_amount_sent[index][1])}
                                 <br></br>
                              </>
                           ))}</Td>
                     </Tr>
                     <Tr>
                        <Th isNumeric style={{textAlign:"left"}}>current tokens balances</Th>
                        <Td>{user_vault.token_deposits.map((tokens:string[], index:number) => (
                              <>
                                 {getTicker(user_vault.token_deposits[index][0])}: 
                                 {getConvertedAmount(user_vault.token_deposits[index][0], user_vault.token_deposits[index][1])}
                                 <br></br>
                              </>
                           ))}</Td>
                     </Tr>
                  </>
                  }
               </Tbody>
            </Table>
         </TableContainer>
      </div>
   )
}

export default Vault;