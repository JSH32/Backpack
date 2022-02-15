import { Page } from "components/page"
import * as React from "react"

import UserIcon from "assets/icons/user.svg"
import LockIcon from "assets/icons/lock.svg"

import {
    Box,
    Icon,
    Text,
    Flex,
    Heading,
    Divider,
    Stack,
    VStack,
    Button,
    useColorModeValue,
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
    ListItem,
    InputRightElement,
    InputGroup,
    Alert,
    AlertIcon,
    chakra,
    Menu,
    MenuButton,
    MenuItem,
    MenuList,
    HStack,
    MenuGroup
} from "@chakra-ui/react"

import {
    ChevronDownIcon,
    ViewIcon,
    ViewOffIcon
} from "@chakra-ui/icons"

import { useHistory, useParams } from "react-router-dom"
import { useForm } from "react-hook-form"
import { observer } from "mobx-react-lite"
import { toJS } from "mobx"
import store from "../../store"
import { updateSettings } from "api"

interface SettingsTab {
    name: string,
    icon: React.FC,
    component: React.FC
}

const ViewButton: React.FC<{
    active: boolean,
    onToggle: (active: boolean) => void
}> = ({ active, onToggle }) => {
    return <Button
        variant="ghost"
        onClick={() => onToggle(!active)}>
        {active ? <ViewIcon /> : <ViewOffIcon />}
    </Button>
}

const tabs: Record<string, SettingsTab> = {
    profile: {
        name: "Profile",
        icon: UserIcon,
        component: observer(() => {
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

            return !userData ? <></> : <>
                <Modal isOpen={isOpen} onClose={closeFinalForm}>
                    <ModalOverlay />
                    <ModalContent>
                        <form onSubmit={finalSubmitForm.handleSubmit(form => {
                            updateSettings(newData, form.password)
                                .then(newData => {
                                    store.setAppInfo(newData)
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
                                        description: error.response.data.message,
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
                                    {Object.keys(newData).map(field => <ListItem>{field}</ListItem>)}
                                    <FormControl mt={"20px"} isRequired>
                                        <FormLabel>Current password</FormLabel>
                                        <Input id="password" type="password" {...finalSubmitForm.register("password")} />
                                    </FormControl>
                                </UnorderedList>
                            </ModalBody>
                            <ModalFooter>
                                <Button variant='ghost' mr={3} onClick={closeFinalForm}>
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
            </>
        })
    },
    password: {
        name: "Password",
        icon: LockIcon,
        component: () => {
            const { register, handleSubmit, reset } = useForm()
            const [loading, setLoading] = React.useState(false)
            const toast = useToast()

            const [viewStatus, setViewStatus] = React.useState<any>({
                current: false,
                new: false,
                confirmNew: false
            })

            const setViewButtonValue = React.useCallback((field, status) => {
                setViewStatus({ ...viewStatus, [field]: status })
            }, [viewStatus])

            const onSubmit = React.useCallback(form => {
                if (form.newPassword !== form.confirmNewPassword) {
                    toast({
                        title: "Passwords do not match",
                        status: "error",
                        duration: 5000,
                        isClosable: true
                    })

                    return
                }

                setLoading(true)
                updateSettings({ newPassword: form.newPassword }, form.currentPassword)
                    .then(() => {
                        toast({
                            title: "Success",
                            description: "Password changed successfully",
                            status: "success",
                            duration: 5000,
                            isClosable: true
                        })

                        reset()
                    })
                    .catch(error => {
                        toast({
                            title: "Error",
                            description: error.response.data.message,
                            status: "error",
                            duration: 5000,
                            isClosable: true
                        })
                    })
                    .finally(() => setLoading(false))
            }, [loading])

            return <form onSubmit={handleSubmit(onSubmit)}>
                <Stack spacing={4}>
                    <FormControl isRequired>
                        <FormLabel>Current Password</FormLabel>
                        <InputGroup>
                            <Input
                                {...register("currentPassword", { required: "Current Password is required" })}
                                id="currentPassword"
                                type={viewStatus.current ? "text" : "password"} />
                            <InputRightElement h="full">
                                <ViewButton
                                    active={viewStatus.current}
                                    onToggle={active => setViewButtonValue("current", active)} />
                            </InputRightElement>
                        </InputGroup>
                    </FormControl>
                    <FormControl isRequired>
                        <FormLabel>New Password</FormLabel>
                        <InputGroup>
                            <Input
                                {...register("newPassword", { required: "New Password is required" })}
                                id="newPassword"
                                type={viewStatus.new ? "text" : "password"} />
                            <InputRightElement h="full">
                                <ViewButton
                                    active={viewStatus.new}
                                    onToggle={active => setViewButtonValue("new", active)} />
                            </InputRightElement>
                        </InputGroup>
                    </FormControl>
                    <FormControl isRequired>
                        <FormLabel>Confirm New Password</FormLabel>
                        <InputGroup>
                            <Input
                                {...register("confirmNewPassword", { required: "New Password confirmation is required" })}
                                id="confirmNewPassword"
                                type={viewStatus.confirmNew ? "text" : "password"} />
                            <InputRightElement h="full">
                                <ViewButton
                                    active={viewStatus.confirmNew}
                                    onToggle={active => setViewButtonValue("confirmNew", active)} />
                            </InputRightElement>
                        </InputGroup>
                    </FormControl>
                    <Box textAlign="right">
                        <Button alignSelf="right" colorScheme="primary" type="submit" isLoading={loading}>Change Password</Button>
                    </Box>
                </Stack>
            </form>
        }
    }
}

export const UserSettings: React.FC = observer(() => {
    const { tab } = useParams<any>()
    const history = useHistory()

    const userData = toJS(store.userData)

    if (tab === undefined || !Object.keys(tabs).includes(tab)) {
        history.push(`/user/settings/${Object.keys(tabs)[0]}`)
        return <></>
    }

    return <Page>
        <Flex mt="7em" justify="center">
            <Box w={{ base: "90vw", md: "700px" }}>
                <Stack spacing={4}>
                    <Heading>Settings</Heading>
                    {!userData || userData.verified ? <></> :
                        <Alert status="warning">
                            <AlertIcon />
                            Please check your email at <chakra.span fontWeight="bold" ml={1} mr={1}>{userData.email}</chakra.span> to verify your account
                        </Alert>}
                    <Divider />
                    <Stack direction={{ base: "column", md: "row" }} spacing={8}>
                        <Box>
                            <Box display={{ base: "flex", md: "none" }}>
                                <Menu matchWidth={true} autoSelect={false}>
                                    <MenuButton textAlign="left" w="full" as={Button} rightIcon={<ChevronDownIcon />}>
                                        <HStack>
                                            <Icon color="primary.300" as={tabs[tab].icon} />
                                            <Text>{tabs[tab].name}</Text>
                                        </HStack>
                                    </MenuButton>
                                    <MenuList>
                                        <MenuGroup title="My Account">
                                            {Object.entries(tabs).map(([name, tabData]) => <MenuItem
                                                icon={<Icon as={tabData.icon} color={tab === name ? "primary.300" : useColorModeValue("gray.600", "gray.400")} />}
                                                w="100%"
                                                bg={tab === name ? 
                                                    useColorModeValue("gray.50", "gray.800") : 
                                                    useColorModeValue("white", "gray.700" )}
                                                key={name}
                                                onClick={() => history.push(`/user/settings/${name}`)}>
                                                {tabData.name}
                                            </MenuItem>)}
                                        </MenuGroup>
                                    </MenuList>
                                </Menu>
                            </Box>
                            <VStack align="left" w="150px" display={{ base: "none", md: "flex" }}>
                                <Text
                                    fontSize="sm"
                                    fontWeight="medium"
                                    ml="4"
                                    color={useColorModeValue("gray.600", "gray.400")}>My Account</Text>
                                {Object.entries(tabs).map(([name, tabData]) => <Button
                                    key={name}
                                    onClick={() => history.push(`/user/settings/${name}`)}
                                    variant={tab === name ? "solid" : "ghost"}
                                    justifyContent="flex-start"
                                    leftIcon={<Icon color="primary.300" as={tabData.icon} />}>
                                    {tabData.name}</Button>)}
                            </VStack>
                        </Box>
                        <VStack align="left" w="full">
                            <Text
                                fontWeight="bold"
                                fontSize="xl">
                                {tabs[tab].name}</Text>
                            <Box
                                rounded="lg"
                                bg={useColorModeValue("white", "gray.700")}
                                boxShadow="lg"
                                p={8}>
                                {React.createElement(tabs[tab].component)}
                            </Box>
                        </VStack>
                    </Stack>
                </Stack>
            </Box>
        </Flex>
    </Page>
})