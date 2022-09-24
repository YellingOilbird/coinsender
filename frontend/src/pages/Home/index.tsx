import {
   Back,
   Description,
   DescriptionText,
   GoldCoin,
   MainMenu,
   Powered,
   Title,
   TitleWrap
} from "./Home.styled";
import {ConnectWallet} from "widgets/connectWallet";
import {useClearStorage} from "./helpers/clearStorage";
import {ChangeToken} from "features/changeToken";
import {WhitedTokens} from "../../widgets/whitedTokens";
import {NextSender, Donate} from "../../features/nextSender";
import near from 'shared/lib/assets/img/near.svg'

const Home = () => {
   useClearStorage()

   return(
      <div>
         {window.accountId !== '' ?
            <Back style={{width:'100%'}}>
               <div style={{backgroundColor: 'orange', width: '100%', alignItems:'center'}}>
                  <Powered style={{width: '25%'}}>POWERED BY LNC &amp; <img src={near} width='40px' alt=''></img></Powered>
               </div>
               <section className="nes-container" style={{backgroundImage: 'linear-gradient( 135deg, #3C8CE7 10%, #00EAFF 100%)', display: 'block', margin: 'auto', width: '50%', padding: '10px'}}>
                  <p style={{textAlign:"center", margin:'auto', padding: '10px'}}>CHOOSE TOKEN FOR SEND</p>
                  <section className="nes-select" style={{margin:'auto', padding: '10px'}}>
                     <ChangeToken />
                  </section>
               </section>

               <Description id="description" className="data"
                    style={{display: 'block', margin: 'auto', width: '60%', padding: '10px'}}>
                  <section className="topic">
                     <MainMenu id="main" className="nes-container with-title is-centered" style={{top: '10px', border:'0'}}>
                        <TitleWrap>
                           <GoldCoin className="nes-icon coin"/>
                           <Title >
                              COINSENDER
                           </Title>
                           <GoldCoin className="nes-icon coin"/>
                        </TitleWrap>

                        <DescriptionText>
                           <strong>Coinsender </strong>
                           allows send tokens to many accounts using one
                           single transaction
                        </DescriptionText>

                        <NextSender />
                     </MainMenu>
                  </section>
               </Description>
               <div style={{textAlign:"center", backgroundImage: 'linear-gradient( 135deg, #70F570 10%, #49C628 100%)', display: 'block', marginTop:'0', marginBottom: '30px', padding: '10px'}}>
                  donate to project
                  <Donate />
               </div>
               <div id="supported" className="nes-table-responsive" style={{textAlign:'center', display:'block'}}>
                  <h1 style={{marginBottom: '20px'}}>supported tokens</h1>
                  <table style={{margin: 'auto', top:'10px', width: '30%', border: '0'}}>
                     <WhitedTokens />
                  </table>
               </div>
            </Back>
            :
            <ConnectWallet />
         }
      </div>
   )
}

export default Home;