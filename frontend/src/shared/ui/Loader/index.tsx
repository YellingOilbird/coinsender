import { Spinner } from '@chakra-ui/react'
import { Heading, Center } from '@chakra-ui/react'

export const Loader = () => {
   return(
      <div style={{
         display:"flex",
         flexDirection:'column',
         alignItems:"center",
         justifySelf:"center",
         marginTop:'20px'
      }} >
         <Heading
            as='h3'
            size='2xl'
            noOfLines={1}
            overflow="none"
         >Loading. . .</Heading >

         <Spinner
            thickness='4px'
            speed='0.65s'
            emptyColor='gray.200'
            color='blue.500'
            size='xl'
            mt={5}
         />
      </div>
   );
};