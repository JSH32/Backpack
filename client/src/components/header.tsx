import { toJS } from "mobx"
import { observer } from "mobx-react-lite"
import * as React from "react"
import { NavLink, useHistory } from "react-router-dom"
import store from "../store"

import SunIcon from "../assets/icons/sun.svg"
import MoonIcon from "../assets/icons/moon.svg"

import UploadIcon from "assets/icons/upload.svg"
import SettingsIcon from "assets/icons/settings.svg"
import LogOutIcon from "assets/icons/log-out.svg"
import KeyIcon from "assets/icons/key.svg"
import { logout } from "api"

import { 
    Text,
    Link,
    Box,
    Button,
    Flex,
    Icon,
    Popover,
    PopoverContent,
    PopoverTrigger,
    Stack,
    useColorModeValue,
    useColorMode,
    Divider
} from "@chakra-ui/react"

import {
    Link as RouteLink
} from "react-router-dom"

const NAV_ITEMS: NavItem[] = [
    {
        label: "Uploads",
        subLabel: "Access your file uploads",
        icon: UploadIcon,
        to: "/user/uploads"
    },
    {
        label: "Settings",
        subLabel: "Account user settings",
        icon: SettingsIcon,
        to: "/user/settings"
    },
    {
        label: "Tokens",
        subLabel: "Manage tokens and applications",
        icon: KeyIcon,
        to: "/user/tokens"
    }
]

interface NavItem {
    label: string
    subLabel?: string
    to?: string,
    icon: React.FC<React.SVGProps<SVGElement>>
}

export const Header: React.FC = () => {
    const history = useHistory()
    const { colorMode, toggleColorMode } = useColorMode()

    const User = observer(() => {
        const onLogout = async() => {
            logout()
                .then(() => {
                    store.setUserInfo(null)
                    history.replace("/")
                })
        }

        const userData = toJS(store.userData)
        return !userData ? <>
            <Button
                as="a"
                variant="link">
                <RouteLink to="/user/login">
                    Sign in
                </RouteLink>
            </Button>
        </> : <>
            <Popover trigger="hover" placement="bottom-end">
                <PopoverTrigger>
                    <Button as="a" variant="link">
                        {userData.username}
                    </Button>
                </PopoverTrigger>
                <PopoverContent
                    border={0}
                    boxShadow="xl"
                    p={4}
                    rounded="xl"
                    minW="m">
                    <Stack>
                        {NAV_ITEMS.map(item => <UserNavCard item={item}/>)}
                        <UserNavCard lastItem={true} item={{
                            label: "Log Out",
                            subLabel: "Sign out of your account",
                            icon: LogOutIcon
                        }} onClick={onLogout}/>
                    </Stack>
                </PopoverContent>
            </Popover>
        </>
    })

    return <Box pos="fixed" w="100%" bg="white.500" zIndex={3}>
        <Flex
            py={{ base: 2 }}
            px={{ base: 4 }}
            borderBottom={1}
            minH="60px"
            borderStyle="solid"
            borderColor={useColorModeValue("gray.200", "gray.900")}
            bg={useColorModeValue("gray.50", "gray.800")}
            align="center">

            <Flex flex={{ base: 1 }} justify={{ md: "start" }}>
                <Text fontSize="2xl" fontWeight="bold"><RouteLink to="/">Backpack</RouteLink></Text>
            </Flex>

            <Stack
                flex={{ base: 1, md: 0 }}
                justify="flex-end"
                direction="row"
                spacing={6}>
                <Button onClick={toggleColorMode} variant="ghost">
                    {colorMode === "light" ? <Icon as={MoonIcon} /> : <Icon as={SunIcon} />}
                </Button>
                <User/>
            </Stack>
        </Flex>
    </Box>
}

const UserNavCard: React.FC<{ 
    item: NavItem, 
    onClick?: () => void,
    lastItem?: boolean
}> = ({ item, onClick, lastItem }) => {
    return <>
        {lastItem ? <Divider/> : <></>}
        <NavLink to={item.to ? item.to : "#"}>
            <Link
                onClick={onClick}
                role="group"
                display="block"
                p={2}
                rounded="md"
                _hover={{ bg: useColorModeValue("purple.50", "gray.900") }}>
                <Stack direction="row" align="center">
                    <Box>
                        <Text
                            color={lastItem ? "red.300" : "grey.200"}
                            transition="all .3s ease"
                            _groupHover={{ color: lastItem ? "red.500" : "purple.400" }}
                            fontWeight={500}>
                            {item.label}
                        </Text>
                        <Text fontSize="sm">{item.subLabel}</Text>
                    </Box>
                    <Flex
                        transition={"all .3s ease"}
                        transform={"translateX(-10px)"}
                        opacity={0}
                        _groupHover={{ opacity: "100%", transform: "translateX(0)" }}
                        justify={"flex-end"}
                        align={"center"}
                        flex={1}>
                        <Icon color={ lastItem ? "red.500" : "purple.400" } w={5} h={5} as={item.icon} />
                    </Flex>
                </Stack>
            </Link>
        </NavLink>
    </>
}