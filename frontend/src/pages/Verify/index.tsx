import {Wrapper, Btns, Error} from 'shared/ui/Layout/style'
import {useEffect, useState} from "react";
import {verifyAcoounts} from "./model/getAccouns";
import {Title, Description} from 'shared/ui/Layout/style';
import {VerifyStatus} from "widgets/status/verify";
import {deposit} from "./model/deposit";
import {DepositStatus} from "widgets/status/deposit";
import {useGoSend} from "shared/lib/helpers/goSend";
import {Textarea} from "features/textarea";
import {useLocation} from "react-router";
import {checkStorage} from "./model/checkStorage";
import {StorageStatus} from "../../widgets/status/storage";

const Verify = () => {
   const {pathname} = useLocation()

   const [error, setError] = useState('')
   const [status, setStatus] = useState('VERIFY')
   const [accounts, setAccounts] = useState('')

   const go = useGoSend();
   const clearAcoounts = () => {
      setAccounts('')
   }

   const coin = pathname.split('/')[3];

   const prettyAccounts = (accounts: string | undefined) => {
      var re = /[`~!@#$%^&*()|+\=?;:'"<>\{\}\[\]\\\/]/g;
      var pretty = JSON.stringify(accounts, null, "\t");
      let parsed = JSON.parse(pretty).replace(re,"").replace(/,/g, '\n');
      //let res = parsed.replace(/testnet/g, 'testnet ');
      return parsed;
   }

   const prettyAccountsNew = (accounts: string | undefined) => {
      let parsed = prettyAccounts(accounts);
      let res = parsed.replace(/.near/g, '.near ');
      return res;
   }


   useEffect(() => {
      const need_to_deposit = localStorage.getItem('need_to_deposit');
      const need_to_send = localStorage.getItem('need_to_send');

      const operations = localStorage.getItem('operations');

      console.log(coin);

      if (typeof window !== 'undefined') {
         const params = new URLSearchParams(window.location.search);
         const transitionHashes = params.get("transactionHashes");
         if (transitionHashes) {
            console.log(transitionHashes);
            console.log(params);
            setStatus('DEPOSIT');
            setAccounts(operations ? operations : '')
         }
      }

      if (
         need_to_send !== null &&
         operations !== null &&
         need_to_deposit === null
      ) {
         setStatus('SEND');
         setAccounts(operations ? operations : '')
      }
   }, [])

   return(
      <Wrapper>
         {status === 'VERIFY' && <>
            <Title>VERIFY AND CHECK ACCOUNTS</Title>
             <VerifyStatus />
             <Description>
               <p>...wait a little bit after push 'VERIFY'</p>
            </Description>
             <Btns id='verify_button' className='nes-btn' onClick={verifyAcoounts(accounts, setStatus, setAccounts, setError, coin, coin !== 'NEAR')}>VERIFY</Btns>
             <Btns className='nes-btn is-warning' onClick={clearAcoounts}>CLEAR ACCOUNTS</Btns>
             <br />
            <Title style={{marginBottom:'5px'}}>EXAMPLES:</Title>
            <Description>
               <p>hello.near 1</p>
               <p>world.near 1</p>
            </Description>
            <Textarea accounts={prettyAccounts(accounts)} setAccounts={setAccounts} />
            </> 
         }

         {status === 'DEPOSIT' && <>
            <Title>DEPOSIT REQUIRED AMOUNT</Title>
            <Description>
               <p>we are taking</p> <p style={{color:"red"}}>0.01 {localStorage.getItem("token_ticker")}</p> <p> more</p> <p> to prevent exceeded amount issues while send</p>
            </Description>
             <DepositStatus />
             <Btns className='nes-btn is-success' onClick={deposit(setStatus, coin)}>Deposit {Number(localStorage.getItem('need_to_deposit_ui')).toFixed(2)} {localStorage.getItem("token_ticker")}</Btns>
             <Textarea accounts={prettyAccounts(accounts)} setAccounts={setAccounts} />
         </> }

         {status === 'SEND' && <>
            <Title>GO TO SEND PAGE</Title>
             <Btns className='nes-btn is-warning' onClick={go}>GO TO SEND!</Btns>
             <Textarea accounts={prettyAccountsNew(accounts)} setAccounts={setAccounts} />
         </>
         }

         {status === 'STORAGE' && <>
             <StorageStatus />
             <Btns id='storage_button' className='nes-btn' onClick={checkStorage(accounts, setStatus, setAccounts, coin)}>CHECK STORAGE</Btns>
             <Textarea accounts={prettyAccounts(accounts)} setAccounts={setAccounts} />
         </>
         }

         <Error>{error}</Error>
      </Wrapper>
   )
}

export default Verify;