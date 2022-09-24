import getConfig from './config'
import {CONTRACT_ID, ENVIRONTMENT} from "../../shared/config";

export const nearConfig:any = getConfig(ENVIRONTMENT, CONTRACT_ID)
export function logout() {
   window.walletConnection.signOut()
   window.location.replace(window.location.origin + window.location.pathname)
}

export function login() {
   window.walletConnection.requestSignIn(nearConfig)
}