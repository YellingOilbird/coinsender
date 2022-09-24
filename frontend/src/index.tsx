import React from 'react';
import ReactDOM from 'react-dom/client';
import App from './app';
import reportWebVitals from './reportWebVitals';
import { Provider } from 'react-redux'
import {store} from './app/model'
import {Buffer} from 'buffer';
import {BrowserRouter} from "react-router-dom";
import {initContract} from "./app/near";
window.Buffer = Buffer

const root = ReactDOM.createRoot(
  document.getElementById('root') as HTMLElement
);
root.render(
   <Provider store={store}>
      <App />
   </Provider>
);

window.nearInitPromise = initContract()
   .then(() => {

      const root = ReactDOM.createRoot(
         document.getElementById('root') as HTMLElement
      );

      root.render(
         <Provider store={store}>
            <App />
         </Provider>
      );
   })
   .catch(console.error)

reportWebVitals();
