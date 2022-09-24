import {InfoTitle, Input, WrapperTextArea} from "./index.style";

interface Props{
   accounts:string;
   setAccounts:(value:string) => void;
}

export const Textarea = ({accounts, setAccounts}: Props) => {
   return(
      <WrapperTextArea className='nes-container with-title' >
         <InfoTitle>account, amount</InfoTitle>
         <Input
            className='nes-textarea'
            value={accounts}
            onChange={(e:React.ChangeEvent<HTMLTextAreaElement>) => setAccounts(e.target.value)}
         />
      </WrapperTextArea>
   )
}