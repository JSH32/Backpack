import { toJS } from "mobx"
import { observer } from "mobx-react-lite"
import * as React from "react"
import Router from "next/router"
import store from "helpers/store"

import SunIcon from "/assets/icons/sun.svg"
import MoonIcon from "/assets/icons/moon.svg"
import UploadIcon from "/assets/icons/upload.svg"
import SettingsIcon from "/assets/icons/settings.svg"
import LogOutIcon from "/assets/icons/log-out.svg"
import KeyIcon from "/assets/icons/key.svg"

import {
  Text,
  Box,
  Button,
  Flex,
  Icon,
  Stack,
  useColorModeValue,
  useColorMode,
  Menu,
  MenuButton,
  MenuList,
  MenuItem,
  MenuDivider
} from "@chakra-ui/react"

import RouteLink from "next/link"
import { useAppInfo } from "helpers/info"

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
    to: "/user/settings/profile"
  },
  {
    label: "Tokens",
    subLabel: "Manage tokens and applications",
    icon: KeyIcon,
    to: "/user/tokens"
  }
]

interface NavItem {
  label: string;
  subLabel?: string;
  to: string;
  icon: any;
}

const Header: React.FC = () => {
  const { colorMode, toggleColorMode } = useColorMode()
  const appInfo = useAppInfo()

  const User = observer(() => {
    const onLogout = async () => {
      localStorage.removeItem("token")
      store.setUserInfo(undefined)
      Router.push("/")
    }

    const userData = toJS(store.userData)
    return !userData ? (
      <Button variant="link">
        <RouteLink href="/user/login">Sign in</RouteLink>
      </Button>
    ) : (
      <Flex alignItems="center">
        <Menu matchWidth={true} autoSelect={false}>
          <MenuButton
            as={Button}
            rounded="full"
            variant="link"
            cursor="pointer"
            minW={0}
          >
            {userData.username}
          </MenuButton>

          <MenuList>
            {NAV_ITEMS.map(item => <MenuItem 
              key={item.label} 
              icon={<Icon as={item.icon} color="gray.400" fontSize="md" mt="5px"/>} 
              onClick={() => Router.push(item.to)}>
                {item.label}
              </MenuItem>)}

            <MenuDivider/>

            <MenuItem icon={<Icon
              as={LogOutIcon} 
              color="gray.400" 
              fontSize="md" 
              mt="5px"/>} 
              onClick={onLogout}>
                Logout
              </MenuItem>
          </MenuList>
        </Menu>
      </Flex>
    )
  })

  return (
    <Box pos="fixed" w="100%" bg="white.500" zIndex={3}>
      <Flex
        py={{ base: 2 }}
        px={{ base: 4 }}
        borderBottom={1}
        minH="60px"
        borderStyle="solid"
        borderColor={useColorModeValue("gray.200", "gray.900")}
        bg={useColorModeValue("gray.50", "gray.800")}
        align="center"
      >
        <Flex flex={{ base: 1 }} justify={{ md: "start" }}>
          <Text fontSize="2xl" fontWeight="bold" bgGradient="linear(to-r, primary.400, primary.300)" bgClip="text">
            <RouteLink href="/">{appInfo?.appName}</RouteLink>
          </Text>
        </Flex>

        <Stack
          flex={{ base: 1, md: 0 }}
          justify="flex-end"
          direction="row"
          spacing={6}
        >
          <Button onClick={toggleColorMode} variant="ghost">
              {colorMode === "light" ? <Icon as={MoonIcon} /> : <Icon as={SunIcon} />}
          </Button>
          <User/>
        </Stack>
      </Flex>
    </Box>
  )
}

export default Header