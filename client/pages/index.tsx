import { VerificationMessage } from 'components/VerificationMessage'
import type { NextPage } from 'next'
import { Icon } from '@chakra-ui/react'
import { SearchIcon } from "@chakra-ui/icons"

const Home: NextPage = () => {
  return (
    <>
    <h1>Hi</h1><Icon as={SearchIcon}/>
    </>
  )
}

export default Home
