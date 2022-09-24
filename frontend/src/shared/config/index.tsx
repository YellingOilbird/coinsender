
const getEnvVar = (key: string) => {
   if (process.env[key] === undefined) {
      throw new Error(`Env variable ${key} is required`);
   }
   return process.env[key] || "";
};

export const CONTRACT_ID = getEnvVar("REACT_APP_CONTRACT_ID");
export const ENVIRONTMENT = getEnvVar("REACT_APP_ENVIRONTMENT");

console.log(CONTRACT_ID);
console.log(ENVIRONTMENT);


export const NODE_ENV = getEnvVar("NODE_ENV");

export const isDevEnv = NODE_ENV === "development";

export const isProdEnv = NODE_ENV === "production";