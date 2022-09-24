import styled from "styled-components";

export const Wrapper = styled.div`
  padding-top: 125px;
  margin: 0 auto;
  display: flex;
  flex-direction: column;
  align-items: center;
`

export const Title = styled.h2`
  text-align: center;
  margin-bottom: 30px;
`

export const Description = styled.p.attrs((props: {mt:string}) => props)`
  text-align: center;
  font-size: 14px;
  margin-top: ${props => props.mt && props.mt};
`

export const Btns = styled.button.attrs((props: {mt:string}) => props)`
  margin: auto;
  width: 384px;
  margin:15px 0;
  margin-top: ${props => props.mt && props.mt};
  
  @media(max-width:400px) {
    width: 300px;
  }

  @media(max-width:320px) {
    width: 250px;
  }
`

export const Error = styled.p`
  color: #b23434;
  margin-top: 20px;
  margin-bottom: -10px;
`