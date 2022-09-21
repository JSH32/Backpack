import * as React from "react"

import {
  Box,
  Button,
  Flex,
  FormControl,
  FormLabel,
  Heading,
  Input,
  Stack,
  Text,
  useColorModeValue,
  useToast
} from "@chakra-ui/react"
import { Page } from "layouts/Page"
import api from "helpers/api"
import { useForm } from "react-hook-form"
import { useStore } from "helpers/store"

export const RegisterPrompt: React.FC = () => {
	const toast = useToast()
	const store = useStore()
	const { register, handleSubmit } = useForm()

	const sendCode = (form: any) => {
		api.user.registerKey(form?.registrationCode)
			.then(userInfo => {
				store?.setUserInfo(userInfo)

				toast({
					title: "Successfully Registered",
					description: `Welcome ${userInfo.username}`,
					status: "success",
					duration: 5000,
					isClosable: true
				})
			})
			.catch(error => {
				toast({
					title: "Registration Error",
					description: error.body.message,
					status: "error",
					duration: 5000,
					isClosable: true
				})
			})
	}

	return <Page>
		<Flex
            minH="100vh"
            align="center"
            justify="center">
			<Stack spacing={8} mx="auto" maxW="lg" py={12} px={6}>
				<Stack align="center" textAlign="center">
					<Heading fontSize="4xl">Complete Registration</Heading>
					<Text fontSize="lg" color="gray.600">
						Service is running in <i>invite only</i> mode.
						Please enter your registration key.
					</Text>
				</Stack>
				<Box
					rounded="lg"
					bg={useColorModeValue("white", "gray.700")}
					boxShadow="lg"
					p={8}>
					<form onSubmit={handleSubmit(sendCode)}>
						<Stack spacing={2}>
							<FormControl>
								<FormLabel>Registration Code</FormLabel>
								<Input {...register("registrationCode", { required: true })} />
							</FormControl>
							<Button type="submit">
								Register
							</Button>
						</Stack>
					</form>
				</Box>
			</Stack>
		</Flex>
	</Page>
}