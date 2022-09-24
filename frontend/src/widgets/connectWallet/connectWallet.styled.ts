import styled from 'styled-components'

export const Wrapper = styled.div`
  padding-top: 100px;
  margin: 0 auto;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: space-between;
`

export const Title = styled.h2`
  margin: 20px 0;
`

export const TitleSign = styled.h1`
  font-size: 35px;
  margin-top: 40px;
  margin-bottom: 20px;
  text-align: center;
  
  @media(max-width: 1350px) {
    font-size: 30px;
  }

  @media(max-width: 1005px) {
    font-size: 25px;
  }

  @media(max-width: 805px) {
    font-size: 20px;
  }
`

export const Info = styled.h2`
  font-size: 20px;
  text-align: center;

  @media(max-width: 780px) {
    font-size: 16px;
  }
`

export const ConnectNear = styled.button`
  position: sticky;

  @media(max-width: 780px) {
    width: 50%;
  }
`