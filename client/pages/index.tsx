import { VerificationMessage } from 'components/VerificationMessage'
import type { NextPage } from 'next'

const Home: NextPage = () => {
  return (
    <>
    <VerificationMessage email={'kakarot.joel@gmail.com'} />
    </>
  )
}

export default Home
