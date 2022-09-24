import {useAppDispatch} from "app/model/hooks";
import {useEffect} from "react";
import {setWhitedListToken} from "../../app/model/slice/tokens";

export const getWhitedList = (component: () => React.ReactNode) => () => {
   const dispatch = useAppDispatch()
   useEffect(() => {
      async function white_list_tokens () {
         const whiteList = await window.contract.get_whitelisted_tokens();
         dispatch(setWhitedListToken(whiteList))
      }

      white_list_tokens()
   }, [dispatch])

   return <>{component()}</>
}