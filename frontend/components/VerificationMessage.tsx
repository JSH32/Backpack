import { EmailIcon } from "@chakra-ui/icons"
import * as React from "react"

import {
  Box,
  Flex,
  Heading,
  Link,
  Text,
  useToast
} from "@chakra-ui/react"
import { Page } from "layouts/Page"
import api from "helpers/api"

export const VerificationMessage: React.FC<{ email: string }> = ({ email }) => {
  const toast = useToast()

  const resendEmail = React.useCallback(() => {
    api.user.resendVerify()
      .then(res =>
        toast({
          title: "Email resent",
          description: res.message,
          status: "success",
          duration: 5000,
          isClosable: true
        })
      )
      .catch(error =>
        toast({
          title: "Error",
          description: error.body.message,
          status: "error",
          duration: 5000,
          isClosable: true
        })
      )
  }, [])

  return (
    <Page>
      <Flex minH="100vh" align="center" justify="center">
        <Box py={10} px={6} textAlign="center">
          <EmailIcon boxSize={"50px"} color={"primary.500"} />
          <Heading as="h2" size="xl" mt={6} mb={2}>
            Verify your email
          </Heading>
          <Box color="gray.500">
            <Text>
              An email was sent to
              <Text as="span" fontWeight="bold"> {email}</Text>. Please click
              the link to verify and activate your account
            </Text>
            <Text>
              If you did not get a link please click
              <Link onClick={resendEmail} color="primary.300" m={1}>
                here
              </Link>
              to resend
            </Text>
          </Box>
        </Box>
      </Flex>
    </Page>
  )
}
