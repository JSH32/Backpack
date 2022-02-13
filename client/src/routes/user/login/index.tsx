import * as React from "react"
import "./style.scss"

import GoogleSVG from "assets/icons/google.svg"
import GithubSVG from "assets/icons/github.svg"
import { Link as RouterLink, useHistory } from "react-router-dom"
import { passwordLogin } from "api"
import store from "../../../store"
import { VerificationMessage } from "components/verificationmessage"
import { Box, Text, Link, useColorModeValue, Flex, Stack, FormControl, FormLabel, Input, Button, Heading, useToast, Divider, chakra, Center, Icon } from "@chakra-ui/react"
import { useForm } from "react-hook-form"
import { Page } from "components/page"

export const UserLogin: React.FC = () => {
    const [postLoginUnverifiedEmail, setPostLoginUnverifiedEmail] = React.useState(null)
    const history = useHistory()

    const { register, handleSubmit } = useForm()
    const toast = useToast()

    const formSubmit = (data: any) => {
        passwordLogin(data.auth, data.password)
            .then(userInfo => {
                store.setAppInfo(userInfo)
                userInfo.verified ? history.replace("/user/uploads") : setPostLoginUnverifiedEmail(userInfo.email)
                toast({
                    title: "Logged in",
                    description: `Welcome ${userInfo.username}`,
                    status: "success",
                    duration: 5000,
                    isClosable: true
                })
            })
            .catch(error => {
                toast({
                    title: "Authentication Error",
                    description: error.response.data.message,
                    status: "error",
                    duration: 5000,
                    isClosable: true
                })
            })
    }

    if (postLoginUnverifiedEmail != null)
        return <VerificationMessage email={postLoginUnverifiedEmail} />

    return <Page>
        <Flex
            id="login"
            minH="100vh"
            align="center"
            justify="center">
            <Stack spacing={8} mx="auto" maxW="lg" py={12} px={6}>
                <Stack align="center" textAlign="center">
                    <Heading fontSize="4xl">Sign in to your account</Heading>
                    <Text fontSize="lg" color="gray.600">
                        To access your account and files
                    </Text>
                </Stack>
                <Box
                    rounded="lg"
                    bg={useColorModeValue("white", "gray.700")}
                    boxShadow="lg"
                    p={8}>
                    <Stack spacing={2}>
                        <Button w="full" variant="outline" leftIcon={<Icon as={GoogleSVG} />}>
                            <Center>
                                <Text>Sign in with Google</Text>
                            </Center>
                        </Button>
                        <Button w="full" colorScheme="blackAlpha" color="white" bg="black" variant="solid" leftIcon={<Icon color="white.500" as={GithubSVG} />}>
                            <Center>
                                <Text>Sign in with Github</Text>
                            </Center>
                        </Button>
                    </Stack>
                    <Box className="separator">
                        <Divider borderColor="white.500" />
                        <chakra.span>or</chakra.span>
                        <Divider borderColor="white.500" />
                    </Box>
                    <form onSubmit={handleSubmit(formSubmit)}>
                        <Stack spacing={5}>
                            <Stack spacing={2}>
                                <FormControl>
                                    <FormLabel>Username or Email</FormLabel>
                                    <Input {...register("auth", { required: true })} />
                                </FormControl>
                                <FormControl id="password">
                                    <FormLabel>Password</FormLabel>
                                    <Input {...register("password", { required: true })} type="password" />
                                </FormControl>
                            </Stack>
                            <Button
                                bg="primary.500"
                                type="submit"
                                color="white"
                                _hover={{
                                    bg: "primary.600"
                                }}>
                                Sign in
                            </Button>
                            <Text textAlign="center" >
                                Dont have an account? <RouterLink to="/user/create"><Link color="primary.300">Sign up</Link></RouterLink>
                            </Text>
                        </Stack>
                    </form>
                </Box>
            </Stack>
        </Flex>
    </Page>
}