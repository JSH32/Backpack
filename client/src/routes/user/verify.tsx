import * as React from "react"
import { verify } from "api"
import { useLocation } from "react-router-dom"

import store from "../../store"
import { toJS } from "mobx"
import { Box, Flex, Heading, Text } from "@chakra-ui/react"
import { CheckCircleIcon, CloseIcon } from "@chakra-ui/icons"
import { Page } from "components/page"

export const UserVerify: React.FC = () => {
    const verificationCode = new URLSearchParams(useLocation().search).get("code")

    const [verifySuccess, setVerifySuccess] = React.useState(null)

    React.useEffect(() => {
        verify(verificationCode)
            .then(() => {
                setVerifySuccess(true)

                const userData = toJS(store.userData)
                if (userData)
                    store.setAppInfo({ ...userData, verified: true })
            })
            .catch(() => setVerifySuccess(false))
    }, [])

    // While verification is pending
    if (verifySuccess == null) return <></>

    return <Page>
        <Flex minH="100vh"
                align="center"
                justify="center">
            <Box py={10} px={6} textAlign="center">
                { verifySuccess ? <>
                    <CheckCircleIcon boxSize="50px" color="green.500"/>
                    <Heading as="h2" size="xl" mt={6} mb={2}>Account verified</Heading>
                    <Text>Your account was verified. { toJS(store.userData) ? "You may now access your account" : "Please login to access your account" }</Text>
                </> : <>
                    <Box display="inline-block">
                        <Flex
                            flexDirection="column"
                            justifyContent="center"
                            alignItems="center"
                            bg="red.500"
                            rounded="50px"
                            w="55px"
                            h="55px"
                            textAlign="center">
                            <CloseIcon boxSize="20px" color="white"/>
                        </Flex>
                    </Box>
                    <Heading as="h2" size="xl" mt={6} mb={2}>Invalid verification code</Heading>
                    <Text>Invalid or expired verification code was provided</Text>
                </>}
            </Box>
        </Flex>
    </Page>
}