import styled from "styled-components";

export const Back = styled.div`
background-image: radial-gradient( circle farthest-corner at 50.7% 54%,  rgba(244,254,252,1) 0%, rgba(249,253,255,1) 92.4% );
`

export const TitleWrap = styled.div`
  width: 70%;
  margin: 0 auto;
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 10px;
  
  @media(max-width:779px) {
    flex-direction: column;
  }
`

export const GoldCoin = styled.i`
  margin-bottom: 0;
  margin-right: 0;

  @media(max-width:779px) {
    display: none;
  }
`

export const Description = styled.section`
  @media(max-width:480px) {
    width: 100% !important;
  }
`

export const MainMenu = styled.section`
  @media(max-width:480px) {
    width: 100% !important;
  }
`

export const Title = styled.h2`
  font-size: 32px;
  animation:blink 3s ease-in-out;
  margin: 0 5px;
  margin-top:10px;

  @media(max-width:770px) {
    font-size: 27px;
  }

  @media(max-width:480px) {
    font-size: 20px;
  }

  @media(max-width:350px) {
    font-size: 25px;
  }

  @media(max-width:320px) {
    font-size: 20px;
  }
`

export const DescriptionText = styled.p`
  @media(max-width:320px) {
    font-size: 12px;
    margin: 15px 0;
    margin-bottom:20px;
  }
`

export const CoinsWrap = styled.div`
  display: block;
`

export const Coins = styled.div`
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom:20px;
`

export const Powered = styled.div`
  display: flex;
  justify-content: space-between;
  margin: auto;
  align-items: center;
  margin-bottom:20px;
`

export const Next = styled.a`
  display: block;
  margin: 0 auto;
  margin-top: 20px;
  font-size: 17px;
  
  @media(max-width: 320px) {
    width: 100%;
    font-size: 14px;
  }
`