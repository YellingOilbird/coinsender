import { configureStore, ThunkAction, Action } from '@reduxjs/toolkit';
import tokensSlice from "./slice/tokens";

export const store = configureStore({
   reducer: {
      tokens:tokensSlice
   },
   middleware: (getDefaultMiddleware) => getDefaultMiddleware({
      serializableCheck: false
   })
})

export type AppDispatch = typeof store.dispatch;
export type RootState = ReturnType<typeof store.getState>;
export type AppThunk<ReturnType = void> = ThunkAction<
   ReturnType,
   RootState,
   unknown,
   Action<string>
   >;