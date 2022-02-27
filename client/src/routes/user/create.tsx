import * as React from "react"
import { userCreate } from "api"
import { Link as RouterLink } from "react-router-dom"
import { useForm } from "react-hook-form"
import { Page } from "components/page"

import { 
    ViewIcon, 
    ViewOffIcon
} from "@chakra-ui/icons"

import { 
    Flex, 
    Link, 
    Heading, 
    Stack, 
    useToast,
    Text,
    Box, 
    useColorModeValue, 
    FormControl, 
    FormLabel, 
    Input, 
    InputGroup, 
    InputRightElement, 
    Button
} from "@chakra-ui/react"

import { VerificationMessage } from "components/verificationmessage"

export const UserCreate: React.FC = () => {
    const [emailPostSignup, setEmailPostSignup] = React.useState(null)
    const [showPassword, setShowPassword] = React.useState(false)

    const { register, handleSubmit } = useForm()
    const toast = useToast()
    
    const formSubmit = (data: any) => {
        userCreate(data.username, data.email, data.password)
            .then(() => {
                setEmailPostSignup(data.email)
                toast({
                    title: "Account created",
                    description: "Your account has been created",
                    status: "success",
                    duration: 5000,
                    isClosable: true
                })
            })
            .catch(error => toast({
                title: "Error",
                description: error.response.data.message,
                status: "error",
                duration: 5000,
                isClosable: true
            }))
    }
    
    if (emailPostSignup !== null)
        return <VerificationMessage email={emailPostSignup}/>

    return <Page>
        <Flex
            minH="100vh"
            align="center"
            justify="center">
            <Stack spacing={8} mx="auto" maxW="lg" py={12} px={6}>
                <Stack align="center" textAlign="center">
                    <Heading fontSize="4xl">Create an account</Heading>
                    <Text fontSize="lg" color="gray.600">
                        To upload your files
                    </Text>
                </Stack>
                <Box 
                    rounded="lg"
                    bg={useColorModeValue("white", "gray.700")}
                    boxShadow="lg"
                    w={["full", 400]}
                    p={8}>
                    <form onSubmit={handleSubmit(formSubmit)}>
                        <Stack spacing={5}>
                            <Stack spacing={2}>
                                <FormControl isRequired>
                                    <FormLabel>Username</FormLabel>
                                    <Input {...register("username")} />
                                </FormControl>
                                <FormControl isRequired>
                                    <FormLabel>Email</FormLabel>
                                    <Input {...register("email")} />
                                </FormControl>
                                <FormControl isRequired>
                                    <FormLabel>Password</FormLabel>
                                    <InputGroup>
                                        <Input type={showPassword ? "text" : "password"} {...register("password")}/>
                                        <InputRightElement h="full">
                                        <Button
                                            variant="ghost"
                                            onClick={() => setShowPassword((showPassword) => !showPassword) }>
                                            {showPassword ? <ViewIcon/> : <ViewOffIcon/>}
                                        </Button>
                                        </InputRightElement>
                                    </InputGroup>
                                </FormControl>
                            </Stack>
                            <Button
                                bg="primary.500"
                                type="submit"
                                color="white"
                                _hover={{
                                    bg: "primary.600"
                                }}>
                                Sign up
                            </Button>
                            <Text textAlign="center" >
                                Already have an account? <RouterLink to="/user/login"><Link color="primary.300">Sign in</Link></RouterLink>
                            </Text>
                        </Stack>
                    </form>
                </Box>
            </Stack>
        </Flex>
    </Page>
}