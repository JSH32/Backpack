import React from "react"
import type { NextPage } from "next"

import {
    Box,
    Text,
    Stack,
    Button,
    FormControl,
    FormLabel,
    Input,
    useDisclosure,
    useToast,
    Modal,
    ModalOverlay,
    ModalBody,
    ModalCloseButton,
    ModalContent,
    ModalFooter,
    ModalHeader,
    UnorderedList,
    ListItem
} from "@chakra-ui/react"

import { observer } from "mobx-react-lite"
import { useForm } from "react-hook-form"
import { toJS } from "mobx"
import store from "helpers/store"
import { 
    ProfileTab, 
    SettingsLayout 
} from "layouts/SettingsLayout"
import api from "helpers/api"

const Profile: NextPage = observer(() => {
    const changeForm = useForm()
    const finalSubmitForm = useForm()

    const { isOpen, onOpen, onClose } = useDisclosure()
    const toast = useToast()

    const userData = toJS(store.userData)
    const [newData, setNewData] = React.useState<any>({})

    const closeFinalForm = React.useCallback(() => {
        finalSubmitForm.resetField("password")
        onClose()
    }, [finalSubmitForm.reset])

    const watchAllFields = changeForm.watch()
    const fieldsChanged = React.useCallback(() => {
        for (const [name, field] of Object.entries(changeForm.getValues()))
            if (field !== (userData as any)[name])
                return true

        return false
    }, [watchAllFields, userData])

    return <SettingsLayout tab={ProfileTab}>
        {!userData ? <></> : <>
            <Modal isOpen={isOpen} onClose={closeFinalForm}>
                <ModalOverlay />
                <ModalContent>
                    <form onSubmit={finalSubmitForm.handleSubmit(form => {
                        api.user.settings({ ...newData, currentPassword: form.password })
                            .then(newData => {
                                store.setUserInfo(newData)
                                toast({
                                    title: "Settings changed",
                                    description: "Your settings have been changed",
                                    status: "success",
                                    duration: 5000,
                                    isClosable: true
                                })
                            })
                            .catch(error => {
                                toast({
                                    title: "Problem changing settings",
                                    description: error.body.message,
                                    status: "error",
                                    duration: 5000,
                                    isClosable: true
                                })
                            })
                            .finally(() => closeFinalForm())
                    })}>
                        <ModalHeader>Change profile settings</ModalHeader>
                        <ModalCloseButton />
                        <ModalBody>
                            <Text fontWeight="bold">This will change the following</Text>
                            <UnorderedList>
                                {Object.keys(newData).map(field => <ListItem key={field}>{field}</ListItem>)}
                                <FormControl mt={"20px"} isRequired>
                                    <FormLabel>Current password</FormLabel>
                                    <Input id="password" type="password" {...finalSubmitForm.register("password")} />
                                </FormControl>
                            </UnorderedList>
                        </ModalBody>
                        <ModalFooter>
                            <Button variant="ghost" mr={3} onClick={closeFinalForm}>
                                Close
                            </Button>
                            <Button colorScheme="primary" type="submit">Submit</Button>
                        </ModalFooter>
                    </form>
                </ModalContent>
            </Modal>
            <form onSubmit={changeForm.handleSubmit(form => {
                const newData = {}

                // Only store the new values
                for (const [name, field] of Object.entries(form)) {
                    if (field !== (userData as any)[name])
                        (newData as any)[name] = field
                }

                // Only open modal if something actually changed
                if (Object.keys(newData).length > 0) {
                    setNewData(newData)
                    onOpen()
                }
            })}>
                <Stack spacing={4}>
                    <FormControl isRequired>
                        <FormLabel>Username</FormLabel>
                        <Input id="username" type="text" defaultValue={userData.username} {...changeForm.register("username")} />
                    </FormControl>
                    <FormControl isRequired>
                        <FormLabel>Email address</FormLabel>
                        <Input id="email" type="email" defaultValue={userData.email} {...changeForm.register("email")} />
                    </FormControl>
                    <Box textAlign="right">
                        <Button alignSelf="right" colorScheme="primary" type="submit" isDisabled={!fieldsChanged()}>Save</Button>
                    </Box>
                </Stack>
            </form>
        </>}
    </SettingsLayout>
})

export default Profile
