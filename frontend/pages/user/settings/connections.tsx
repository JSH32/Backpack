import { Button, ModalOverlay, Text, HStack, Icon, Stack, Box, useColorModeValue, VStack, Flex, Spacer, useDisclosure, Modal, ModalContent, ModalHeader, ModalCloseButton, ModalBody, ModalFooter, Input, FormControl, FormLabel, useToast } from "@chakra-ui/react"
import { useStore } from "helpers/store"
import { SettingsLayout, ConnectionsTab } from "layouts/SettingsLayout"
import { observer } from "mobx-react-lite"
import { NextPage } from "next"

import DiscordSVG from "assets/icons/discord.svg"
import GoogleSVG from "assets/icons/google.svg"
import GithubSVG from "assets/icons/github.svg"
import { useAppInfo } from "helpers/info"
import { OAuthProvider } from "@/client"
import api from "helpers/api"
import React from "react"
import { useForm } from "react-hook-form"

const SocialItem: React.FC<{
	provider: OAuthProvider,
	icon: React.FC,
	username?: string,
	color?: string,
	enabled?: boolean
}> = ({ provider, username, color, icon, enabled }) => {
	const passwordForm = useForm()
	const { isOpen, onOpen, onClose } = useDisclosure()
	const store = useStore()
	const toast = useToast()

	const disconnect = React.useCallback((password?: string) => {
		api.authentication.unlinkMethod({
			method: provider,
			password: password
		})
		.then(authMethods => store?.setMethods(authMethods))
		.catch(error => {
			toast({
				title: "Error",
				description: error.body.message,
				status: "error",
				duration: 5000,
				isClosable: true
			})
		})
		.finally(() => closeForm())
	}, [])

	const closeForm = React.useCallback(() => {
        passwordForm.reset({
			password: ""
		})
        onClose()
    }, [passwordForm])

	const Inner = () => {
		return <>
			<Modal isOpen={isOpen} onClose={closeForm}>
				<ModalOverlay/>
				<ModalContent>
					<form onSubmit={passwordForm.handleSubmit(form => {
						disconnect(form?.password)
					})}>
						<ModalHeader>Password required</ModalHeader>
                        <ModalCloseButton />
						<ModalBody>
							<FormControl isRequired>
								<FormLabel>Password</FormLabel>
								<Input type="password" {...passwordForm.register("password")} />
							</FormControl>
						</ModalBody>
						<ModalFooter>
                            <Button variant="ghost" mr={3} onClick={closeForm}>Close</Button>
                            <Button colorScheme="primary" type="submit">Disconnect</Button>
                        </ModalFooter>
					</form>
				</ModalContent>
			</Modal>
			<HStack spacing={5}>
				<Icon fontSize="3xl" as={icon} color={color}/>
				{ enabled ? <>
					<VStack spacing={0} align="left">
						<Text fontSize="xl" fontWeight="bold">{username}</Text>
						<Text as="sub" fontSize="md" pb={5}>{provider[0].toUpperCase() + provider.substring(1)}</Text>
					</VStack>
				</> : <>
					<Text fontSize="xl" fontWeight="bold">{provider[0].toUpperCase() + provider.substring(1)}</Text>
				</> }
			</HStack>
			<Spacer />
			<Box width={{ base: "full", md: "120px" }}>
				{ enabled ? <>
					<Button colorScheme="primary" w="full" onClick={() => {
						if (store?.authMethods?.password)
							onOpen()
						else
							disconnect()
					}}>Disconnect</Button>
				</> : <>
					<Button colorScheme="primary" w="full">Connect</Button>
				</> }
			</Box>
		</>
	}

	return <>
		<Box
			rounded="lg"
			p={2}
			bg={useColorModeValue("gray.200", "gray.800")}
		>
			<HStack ml={2} mr={2} display={{ base: "none", md: "flex" }}>
				<Inner/>
			</HStack>
			<VStack m={2} spacing={3} display={{ base: "row", md: "none" }}>
				<Inner/>
			</VStack>
		</Box>
	</>
}

const Connections: NextPage = observer(() => {
	const store = useStore()
	const appInfo = useAppInfo()
	
	return <SettingsLayout tab={ConnectionsTab}>
		<Stack spacing={2}>
			{ appInfo?.oauthProviders.google && <SocialItem provider={OAuthProvider.GOOGLE} 
				username={store?.authMethods?.google}
				icon={GoogleSVG} 
				enabled={store?.authMethods?.google !== null}
			/> }
			{ appInfo?.oauthProviders.google && <SocialItem provider={OAuthProvider.GITHUB} 
				username={store?.authMethods?.github}
				icon={GithubSVG} 
				enabled={store?.authMethods?.github !== null}
			/> }
			{ appInfo?.oauthProviders.google && <SocialItem provider={OAuthProvider.DISCORD} 
				username={store?.authMethods?.discord}
				icon={DiscordSVG}
				color="#404EED"
				enabled={store?.authMethods?.discord !== null}
			/> }
		</Stack>
	</SettingsLayout>
})

export default Connections