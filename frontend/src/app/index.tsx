import { withProviders } from "./providers";
import {Router} from "../pages";
import './index.css';

function App() {
   return (
      <div className="App">
         <Router />
      </div>
   );
}

export default withProviders(App);
