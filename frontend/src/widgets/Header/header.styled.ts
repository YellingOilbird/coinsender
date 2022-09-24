import styled from 'styled-components'
import {Link} from "react-router-dom";

export const HeaderWrap = styled.header`
  height: 100px;
  background: black;
  display: flex;
  align-items: center;
  justify-content: space-between;
  transition: all 0.2s ease;
  padding: 0 15px;

`

export const LinksWrap = styled.div`
  display: flex;
  flex-wrap: wrap;
  max-width: 500px;
  align-items: center;
  justify-content: space-between;
  
  @media(max-width: 800px) {
    display: none;
  }
`

export const MenuMobail = styled.div`
  @media(min-width: 800px) {
    display: none;
  }
`

export const WrapLink = styled(Link)`
  text-decoration: none;
  
  @media(max-width: 800px) {
    margin: 10px 0;
  }
`

export const LinksBtn = styled.button`
  margin: 0 15px;
  display: block;
  text-decoration: none;

  @media(max-width: 800px) {
    margin: 20px 15px;
  }
`

export const DepositWrap = styled.div`
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: space-between;
  background: white;
  border-radius: 10px;
`

export const Deposit = styled.span`
  position: sticky !important;
  font-size: 10px !important;
  color:black !important;
  display: flex !important;
  align-items: center !important;
  justify-content: center !important;
  padding: 10px !important;
`