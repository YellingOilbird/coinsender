function trimLeadingZeroes(value:string) {
   value = value.replace(/^0+/, '');
   if (value === '') {
      return '0';
   }
   return value;
}

export function ConvertToYocto(amt:number=0, dec:number=0){
   if (!amt) {
      return null;
   }

   dec = Number(dec);
   let convertAmt = String(amt);
   convertAmt = convertAmt.replace(/,/g, '').trim();
   const split = convertAmt.split('.');
   const wholePart = split[0];
   const fracPart = split[1] || '';
   // if (split.length > 2 || fracPart.length > dec) {
   //    throw new Error(`Cannot parse '${convertAmt}' as '${localStorage.getItem("token_ticker")} amount`);
   // }

   return (trimLeadingZeroes(wholePart + fracPart.padEnd(dec, '0')));
}