import {ConnectNear, Info, Title, TitleSign, Wrapper} from "./connectWallet.styled";
import {GoldCoin} from "../../shared/ui/styled-components";
import {login} from "../../app/near/utils";

export const ConnectWallet = () => {
   return(
      <div className='container'>
         <Wrapper>
            <GoldCoin style={{marginBottom:'10px'}} className="nes-icon coin" />
            <TitleSign>COINSENDER</TitleSign>
            <Info>This is pre-audited BETA version. Use at your own risk!</Info>
            <Title>Sign with NEAR to continue {'->'}</Title>
            <ConnectNear className='nes-btn' onClick={login}>Sign In</ConnectNear>
         </Wrapper>
      </div>
   )
}