import {Title, Description, Wrapper, Btns} from 'shared/ui/Layout/style'
import {useLocation} from "react-router";
import {send_unsafe} from "./model/send";
import {withdraw} from "./model/withdraw";
import {useEffect, useState} from "react";
import {Donate} from "../../features/nextSender";

const Send = () => {
   const {pathname} = useLocation()
   const [counetr_accs, setCounetr_accs] = useState(0)
   const [counetr_accs_ui, setCounetr_accs_ui] = useState("")
   const [sendStatus, setSendStatus] = useState("")
   const [currentHash, setCurrentHash] = useState("")


   useEffect(() => {
      const operations = localStorage.getItem("operations")
      const accounts = JSON.parse(operations ? operations : '[]');
      let accountsRaw = Object.keys(accounts);
      let remaining_accounts = Number(localStorage.getItem("remaining_counter"))
      
      if (typeof window !== 'undefined') {
         const params = new URLSearchParams(window.location.search);
         const transitionHashes = params.get("transactionHashes");
         if (transitionHashes) {
            console.log(transitionHashes);
            console.log(params);
            setCurrentHash('https://explorer.testnet.near.org/transactions/'+transitionHashes);
            setSendStatus('PARTIAL_DONE');
         }
         if (transitionHashes && !remaining_accounts) {
            console.log("done");
            setSendStatus('DONE');
         }
      }

      if(!remaining_accounts) {
         setCounetr_accs(accountsRaw.length);
         setCounetr_accs_ui(accountsRaw.length.toString())
      } else if(remaining_accounts < 25) {
         setCounetr_accs(remaining_accounts);
         setCounetr_accs_ui('next ' + remaining_accounts.toString())
      } else {
         setCounetr_accs(25);
         setCounetr_accs_ui("next 25")
      }
   }, [])

   return(
      
      <Wrapper>
         <Title>
         SEND {localStorage.getItem("token_ticker")} TOKEN
         </Title>
         {sendStatus === 'PARTIAL_DONE' && <>
         <Title style={{color:'red'}}>
         DON'T LEAVE THIS PAGE UNTIL THE PROCESS WILL BE DONE!
         </Title>
            <Description mt="20px" style={{fontSize: "18px"}}>
               Number of remaining recepients: {localStorage.getItem("remaining_counter")}
            </Description>

            <Description mt="20px">
               <div style={{fontSize: "12px", width:'50%', textAlign:'center', margin:'auto'}}>
                  <a href={currentHash} style={{color:"blue", width:'50%', textAlign:'center', margin:'auto'}}>
                     Click to see the progress on approval transaction page! (please do it with new TAB)
                  </a>
               </div>
            </Description>

            <Btns mt="20px" className='nes-btn is-warning' onClick={send_unsafe(pathname)}>send to {counetr_accs_ui} accounts</Btns>
         </> }
         {sendStatus === 'DONE' && <>
            <Description mt="20px">
               DONE!
            </Description>
            <Description mt="20px">
               <br></br>
               <a href='https://explorer.mainnet.near.org/accounts/coinsender.near' style={{color:"blue"}}>
                  Click to check last send transactions
               </a>
            </Description>
            <div style={{textAlign:"center", backgroundImage: 'linear-gradient( 135deg, #70F570 10%, #49C628 100%)', display: 'block', width: '100%', marginTop:'0', margin: '30px', padding: '10px'}}>
               donate to project
               <Donate />
            </div>
         </> }
         {sendStatus === '' && <>
         <Title style={{color:'red'}}>
         DON'T LEAVE THIS PAGE UNTIL THE PROCESS WILL BE DONE!
         </Title>
            <Description mt="20px">
               Number of remaining recepients: {counetr_accs_ui}
            </Description>

            <Description mt="24px">
               Click one more time on SEND for send to next account group (25 accounts). Check progress on redirected approval transaction page!
            </Description>

            <Btns mt="20px" className='nes-btn is-warning' onClick={send_unsafe(pathname)}>send to {counetr_accs_ui} accounts</Btns>
         </> }
         <Description mt="5px">
            <br></br>
            Withdraw available amount of {localStorage.getItem("token_ticker")}
         </Description>

         <Btns mt="20px" className='nes-btn is-success' onClick={withdraw}>withdraw {localStorage.getItem("token_ticker")} </Btns>

      </Wrapper>
   )
}

export default Send;