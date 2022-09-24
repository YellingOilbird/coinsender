import { createSlice } from '@reduxjs/toolkit'
import type { PayloadAction } from '@reduxjs/toolkit'
import {token} from "shared/config/type";

interface initialState {
   tokens:token[] | never[],
   activeToken:string | string[];
}

const initialState:initialState = {
   tokens:[],
   activeToken:['NEAR', '24', 'NEAR']
}

export const tokensSlice = createSlice({
   name: 'tokens',
   initialState,
   reducers: {
      setToken:(state, action: PayloadAction<string>) => {
         state.activeToken = action.payload;
      },

      setWhitedListToken:(state, action: PayloadAction<token[]>) => {
         state.tokens = action.payload
      }
   },
})

export const { setToken, setWhitedListToken } = tokensSlice.actions

export default tokensSlice.reducer