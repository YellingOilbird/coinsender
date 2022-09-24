import {useLocation, useNavigate} from "react-router";

export const useGoSend = () => {
   const navigate = useNavigate()
   const {pathname} = useLocation()

   function go() {
      const token = pathname.split('/')[3]
      navigate(`/processing/send/${token}`)
   }

   return go
}