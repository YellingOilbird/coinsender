import styled from 'styled-components';

export const Back = styled.div`
width: 70%;
margin: 0 auto;
display: flex;
justify-content: space-between;
align-items: center;
margin-bottom: 10px;
margin-top:10px;
@media(max-width:779px) {
  flex-direction: column;
}
`
export const Input = styled.textarea`
  width: 80%;
  height: 170px;
  font-family: system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Oxygen, Ubuntu, Cantarell, "Open Sans", "Helvetica Neue", sans-serif;
  outline: none;
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