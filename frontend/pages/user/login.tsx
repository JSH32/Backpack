import { VerificationMessage } from "components/VerificationMessage"
import type { NextPage } from "next"

import * as React from "react"

import { 
    Box, 
    Button, 
    Center, 
    chakra,
    Divider, 
    Flex, 
    FormControl, 
    FormLabel, 
    Heading, 
    Icon,
    Input, 
    Link, 
    Stack, 
    Text, 
    useColorModeValue, 
    useToast 
} from "@chakra-ui/react"

import { Page } from "layouts/Page"
import { useRouter } from "next/router"
import { useForm } from "react-hook-form"
import { default as RouterLink } from "next/link"

import GoogleSVG from "assets/icons/google.svg"
import GithubSVG from "assets/icons/github.svg"
import DiscordSVG from "assets/icons/discord.svg"

import styles from "styles/login.module.scss"
import { BasicAuthForm } from "@/client"
import api from "helpers/api"
import getConfig from "next/config"
import { useAppInfo } from "helpers/info"
import { useStore } from "helpers/store"

const Login: NextPage = () => {
    const [postLoginUnverifiedEmail, setPostLoginUnverifiedEmail] = React.useState<string | null>(null)
    const router = useRouter()
    const appInfo = useAppInfo()
    const { publicRuntimeConfig } = getConfig()

    const { token, fail } = router.query

    const { register, handleSubmit } = useForm()
    const toast = useToast()
    const store = useStore()

    React.useEffect(() => {
        if (fail) {
            toast({
                title: "Authentication Error",
                description: fail,
                status: "error",
                duration: 5000,
                isClosable: true
            })
        } else if (token != null) {
            tokenLogin(token as string)
        } else if (store?.userData != null) {
            router.replace("/user/uploads")
        }
    }, [])

    const tokenLogin = React.useCallback((token: string) => {
        localStorage.setItem("token", token)

        api.user.info().then(userInfo => {
            store?.setUserInfo(userInfo)
            userInfo.verified 
                ? router.replace("/user/uploads") 
                : setPostLoginUnverifiedEmail(userInfo.email)

            toast({
                title: "Logged in",
                description: `Welcome ${userInfo.username}`,
                status: "success",
                duration: 5000,
                isClosable: true
            })
        })
    }, [])

    const formSubmit = (data: BasicAuthForm) => {
        api.authentication.basic(data)
            .then(tokenRes => {
                tokenLogin(tokenRes.token)
            })
            .catch(error => {
                toast({
                    title: "Authentication Error",
                    description: error.body.message,
                    status: "error",
                    duration: 5000,
                    isClosable: true
                })
            })
    }

    const oauthSignIn = React.useCallback((provider: string) => {
        window.location.replace(`${publicRuntimeConfig.apiRoot}/api/auth/${provider}/login`)
    }, [])

    if (postLoginUnverifiedEmail != null)
        return <VerificationMessage email={postLoginUnverifiedEmail} />

        return <Page title="Login">
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
                        { appInfo?.oauthProviders.google && 
                            <Button 
                                w="full" 
                                variant="outline" 
                                leftIcon={<Icon as={GoogleSVG} mt="2px"/>}
                                onClick={() => oauthSignIn("google")}
                            >
                                <Center>
                                    <Text>Sign in with Google</Text>
                                </Center>
                            </Button> 
                        }
                        { appInfo?.oauthProviders.github && 
                            <Button 
                                w="full" 
                                colorScheme="blackAlpha" 
                                color="white" 
                                bg="black" 
                                variant="solid" 
                                leftIcon={<Icon as={GithubSVG} mt="2px"/>} 
                                onClick={() => oauthSignIn("github")}
                            >
                                <Center>
                                    <Text>Sign in with Github</Text>
                                </Center>
                            </Button>
                        }
                        { appInfo?.oauthProviders.discord && 
                            <Button 
                                w="full" 
                                colorScheme="purple" 
                                color="white" 
                                bg="#404EED" 
                                leftIcon={<Icon as={DiscordSVG} mt="3px" fontSize="xl"/>} 
                                onClick={() => oauthSignIn("discord")}
                                _hover={{ bg: "#5865F2" }}
                            >
                                <Center>
                                    <Text>Sign in with Discord</Text>
                                </Center>
                            </Button>
                        }
                    </Stack>
                    { Object.values(appInfo?.oauthProviders as any).some(v => v === true) && 
                        <Box className={styles.separator}>
                            <Divider borderColor="white.500" />
                            <chakra.span>or</chakra.span>
                            <Divider borderColor="white.500" />
                        </Box> 
                    }
                    <form onSubmit={handleSubmit(formSubmit as any)}>
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
                                Dont have an account? <RouterLink href="/user/create"><Link color="primary.300">Sign up</Link></RouterLink>
                            </Text>
                        </Stack>
                    </form>
                </Box>
            </Stack>
        </Flex>
    </Page>
}

export default Login
