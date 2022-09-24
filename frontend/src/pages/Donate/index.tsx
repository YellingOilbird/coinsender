import {useEffect, useState} from "react";
import {utils} from 'near-api-js';
import {Back, Title} from "./Donate.styled";
import { ConvertToYocto } from "shared/lib/helpers/convert";


const Donate = () => {
   const [donated, setDonated] = useState('')
   const [destinations, setDestinations] = useState<string[] | false>(false);  
   const [currentHash, setCurrentHash] = useState("")

   const depositDonate = async() => {
      var parsed_donate = parseFloat(donated.replace(',', '.').replace(' ', ''))
      var deposit = ConvertToYocto(parsed_donate, 24);
      await window.contract.transfer_near_to_contract(
         {}, window.gas, deposit
      );
   }

   useEffect(() => {
      async function getDonateDestinations() {
         let destinations = await window.contract.get_donate_destinations();
         setDestinations(destinations)
      }

      let txBlock = document.getElementById('tx');
      if ( txBlock !== null ) {
         txBlock.style.display = 'none';
      }     
      if (typeof window !== 'undefined') {
         const params = new URLSearchParams(window.location.search);
         const transitionHashes = params.get("transactionHashes");
         if (transitionHashes) {
            console.log(transitionHashes);
            console.log(params);
            setCurrentHash('https://explorer.mainnet.near.org/transactions/'+transitionHashes);
            let txBlock = document.getElementById('tx');
            if ( txBlock !== null ) {
               txBlock.style.display = 'block';
            }         
         }
      }
      getDonateDestinations();
   }, []);

   return(
      <>
         <Title className='nes-container' style={{marginTop:'150px', textAlign:'center', border:'0'}}>DONATE <a>Ⓝ</a></Title>
         <div id='tx' style={{display:'none', width: '50%', margin:'auto', padding:'10px', textAlign:'center'}}> 
               <p>Donation complete!</p> 
               <br></br>
               <a href={currentHash} style={{color:"blue"}}>
                  Click to check your transaction
               </a>
         </div>
         <Back>
            <textarea id="number"
               style={{width:'50%', alignItems: "center", height: '50px', textAlign:'center'}}
               className='nes-textarea'               
               onChange={(evt:React.ChangeEvent<HTMLTextAreaElement>) => { setDonated(evt.target.value); }} 
            />
            <button
               className='nes-btn is-success' 
               style={{width:'50%', alignItems: "center", height: '50px'}} 
               onClick={() => depositDonate()} 
            >
            Donate {donated}<a style={{fontWeight:'bold', fontSize:'18px'}}>Ⓝ</a>
            </button>

         </Back>
         {destinations && 
            <div style={{width:'50%', alignItems: "center", textAlign:'center', margin:'auto', padding:'20px'}}>
               Donation destinations:
               <p style={{marginTop: '10px'}}>{destinations.map((token:string) => token+'\n')}</p>
            </div>
         }
      </>
   )
}

export default Donate;