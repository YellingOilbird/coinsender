import {Deposit, DepositWrap, HeaderWrap, LinksBtn, LinksWrap, MenuMobail, WrapLink} from "./header.styled";
import {Links} from "./types";
import {useEffect, useState} from "react";
import {useLocation} from "react-router";
import {getDeposits} from "shared/lib/helpers/getDeposits";
import {
   Menu,
   MenuButton,
   MenuList,
} from '@chakra-ui/react'
import {utils} from 'near-api-js'
import { withdraw } from "pages/Send/model/withdraw";

const links:Links[] = [
   {title:'Home', href:'/', className:'nes-btn is-error'},
   {title:'Vault', href:'/vault', className:'nes-btn'},
]

export const Header = () => {
   const {pathname} = useLocation()
   const [balance, setBalance] = useState('0');
   const coin = pathname.split('/')[3];
   const coinUi = localStorage.getItem("token_ticker");

   const balanceUi = Number(utils.format.formatNearAmount(
      balance,
      Number(localStorage.getItem('token_decimals')
   )))
   .toFixed(2);

   useEffect(() => {
      async function setDeposits() {
         setBalance(await getDeposits(coin))
      }

      setDeposits()
   }, [pathname])

   return(
      <HeaderWrap>
        <div className='container'>
           <LinksWrap>
              {links.map((link:Links) => (
                 <WrapLink key={link.title} to={link.href}>
                    <LinksBtn className={link.className}>{link.title}</LinksBtn>
                 </WrapLink>
              ))}
           </LinksWrap>

        <LinksWrap>
            <DepositWrap style={{marginLeft:'10px', margin:'0px', color: "red"}}>
               <Deposit className='is-success'>
                  {coinUi}
                  {'    '}
                  {
                     Number(utils.format.formatNearAmount(
                        balance,
                        Number(localStorage.getItem('token_decimals')
                        ))).toFixed(2)
                  }
               </Deposit>  
            </DepositWrap>
        </LinksWrap>

        <MenuMobail>
           <Menu>
              <MenuButton color='white' border='2px solid white' padding="10px" >
                 Open
              </MenuButton>

              <MenuList>
                 {links.map((link:Links) => (
                    <WrapLink key={link.title} to={link.href}>
                       <LinksBtn className={link.className}>{link.title}</LinksBtn>
                    </WrapLink>
                 ))}

                 <DepositWrap>
                    <Deposit className='is-success'>
                       {coinUi}
                       {' '}
                       {balanceUi}
                    </Deposit>
                 </DepositWrap>
              </MenuList>
           </Menu>
        </MenuMobail>
      </div>
      </HeaderWrap>
   )
}