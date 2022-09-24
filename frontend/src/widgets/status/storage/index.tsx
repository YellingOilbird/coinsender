import {Smails, WrapperStatus} from "../index.styled";
import {Description} from 'shared/ui/Layout/style';

export const StorageStatus = () => (
   <WrapperStatus>
      <Description>Every fungible token except USN needs registered storage balance in token contract (0.00125 NEAR). When you are click at 'CHECK STORAGE' button we check this for all recipients and redirect you to approve storage registration for non-registered receiver(s)</Description>
      <Smails>💾</Smails>
   </WrapperStatus>
)