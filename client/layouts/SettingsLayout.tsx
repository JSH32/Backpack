import { ChevronDownIcon } from "@chakra-ui/icons"
import { Alert, Text, AlertIcon, Box, Button, Divider, Heading, HStack, Icon, Menu, MenuButton, MenuGroup, MenuItem, MenuList, Stack, useColorModeValue, VStack, Flex } from "@chakra-ui/react"
import store from "helpers/store"
import { toJS } from "mobx"
import { observer } from "mobx-react-lite"
import { useRouter } from "next/router"
import React from "react"
import { Page } from "layouts/Page"

import UserIcon from "assets/icons/user.svg"
import LockIcon from "assets/icons/lock.svg"
import { Authenticated } from "components/Authenticated"

interface SettingsTab {
    name: string,
    icon: React.FC,
    path: string
}

export const ProfileTab = {
    name: "Profile",
    icon: UserIcon,
    path: "/user/settings/profile"
}

export const PasswordTab = {
    name: "Password",
    icon: LockIcon,
    path: "/user/settings/password"
}

export const Tabs = [
    ProfileTab,
    PasswordTab
]

export const SettingsLayout: React.FC<{
    tab: SettingsTab,
    children?: JSX.Element | JSX.Element[]
}> = observer(({ tab, children }) => {
    const router = useRouter()
    
    const userData = toJS(store.userData)

    return <Authenticated allowUnverified>
        <Page title="Settings">
            <Flex mt="7em" justify="center">
                <Box w={{ base: "90vw", md: "700px" }}>
                    <Stack spacing={4}>
                        <Heading>Settings</Heading>
                        {!userData || userData.verified ? <></> :
                            <Alert status="warning">
                                <AlertIcon />
                                <Text>Please check your email at <Text as="span" fontWeight="bold">{userData.email}</Text> to verify your account</Text>
                            </Alert>}
                        <Divider />
                        <Stack direction={{ base: "column", md: "row" }} spacing={8}>
                            <Box>
                                <Box display={{ base: "flex", md: "none" }}>
                                    <Menu matchWidth={true} autoSelect={false}>
                                        <MenuButton textAlign="left" w="full" as={Button} rightIcon={<ChevronDownIcon />}>
                                            <HStack>
                                                <Icon color="primary.300" as={tab.icon} />
                                                <Text>{tab.name}</Text>
                                            </HStack>
                                        </MenuButton>
                                        <MenuList>
                                            <MenuGroup title="My Account">
                                                {Tabs.map(tabData => <MenuItem
                                                    icon={<Icon as={tabData.icon} color={tab.name === tabData.name ? "primary.300" : useColorModeValue("gray.600", "gray.400")} />}
                                                    w="100%"
                                                    bg={tab.name === tabData.name ? 
                                                        useColorModeValue("gray.50", "gray.800") : 
                                                        useColorModeValue("white", "gray.700" )}
                                                    key={tabData.name}
                                                    onClick={() => router.push(tabData.path)}>
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
                                    {Tabs.map(tabData => <Button
                                        key={tabData.name}
                                        onClick={() => router.push(tabData.path)}
                                        variant={tabData.name === tab.name ? "solid" : "ghost"}
                                        justifyContent="flex-start"
                                        leftIcon={<Icon color="primary.300" as={tabData.icon} />}>
                                        {tabData.name}
                                    </Button>)}
                                </VStack>
                            </Box>
                            <VStack align="left" w="full">
                                <Text
                                    fontWeight="bold"
                                    fontSize="xl">
                                    {tab.name}</Text>
                                <Box
                                    rounded="lg"
                                    bg={useColorModeValue("white", "gray.700")}
                                    boxShadow="lg"
                                    p={8}>
                                    {children}
                                </Box>
                            </VStack>
                        </Stack>
                    </Stack>
                </Box>
            </Flex>
        </Page>
    </Authenticated>
})
