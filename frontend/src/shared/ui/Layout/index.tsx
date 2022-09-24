import { Header } from 'widgets/Header'
import { Outlet } from 'react-router-dom'

export const Layout = () => {
   return (
      <div>
         <Header />
         <main>
            <Outlet />
         </main>
      </div>
   )
}