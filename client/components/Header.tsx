import { toJS } from "mobx";
import { observer } from "mobx-react-lite";
import * as React from "react";
import Router from "next/router";
import store from "helpers/store";
import Image from "next/image"
import { logout } from "helpers/api";

const SunIcon = "/assets/icons/sun.svg";
const MoonIcon = "/assets/icons/moon.svg";
const UploadIcon = "/assets/icons/upload.svg";
const SettingsIcon = "/assets/icons/settings.svg";
const LogOutIcon = "/assets/icons/log-out.svg";
const KeyIcon = "/assets/icons/key.svg";

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
  Divider,
} from "@chakra-ui/react";

import RouteLink from "next/link";

const NAV_ITEMS: NavItem[] = [
  {
    label: "Uploads",
    subLabel: "Access your file uploads",
    icon: UploadIcon,
    to: "/user/uploads",
  },
  {
    label: "Settings",
    subLabel: "Account user settings",
    icon: SettingsIcon,
    to: "/user/settings",
  },
  {
    label: "Tokens",
    subLabel: "Manage tokens and applications",
    icon: KeyIcon,
    to: "/user/tokens",
  },
];

interface NavItem {
  label: string;
  subLabel?: string;
  to?: string;
  icon: string;
}

const Header: React.FC = () => {
  const { colorMode, toggleColorMode } = useColorMode();

  const User = observer(() => {
    const onLogout = async () => {
      logout().then(() => {
        store.setUserInfo();
        Router.push("/");
      });
    };

    const userData = toJS(store.userData);
    return !userData ? (
      <>
        <Button as="a" variant="link">
          <RouteLink href="/user/login">Sign in</RouteLink>
        </Button>
      </>
    ) : (
      <>
        <Popover trigger="hover" placement="bottom-end">
          <PopoverTrigger>
            <Button as="a" variant="link">
              {userData.username}
            </Button>
          </PopoverTrigger>
          <PopoverContent border={0} boxShadow="xl" p={4} rounded="xl" minW="m">
            <Stack>
              {NAV_ITEMS.map((item, index) => (
                <UserNavCard key={`${index}\'s Nav Card`} item={item} />
              ))}
              <UserNavCard
                lastItem={true}
                item={{
                  label: "Log Out",
                  subLabel: "Sign out of your account",
                  icon: LogOutIcon,
                }}
                onClick={onLogout}
              />
            </Stack>
          </PopoverContent>
        </Popover>
      </>
    );
  });

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
          <Text fontSize="2xl" fontWeight="bold">
            <RouteLink href="/">Backpack</RouteLink>
          </Text>
        </Flex>

        <Stack
          flex={{ base: 1, md: 0 }}
          justify="flex-end"
          direction="row"
          spacing={6}
        >
          <Button onClick={toggleColorMode} variant="ghost">
            {colorMode === "light" ? (
              <Image src={MoonIcon} height={95} width={95} alt={"MoonIcon"} />
            ) : (
              <Image src={SunIcon} height={95} width={95} alt={"SunIcon"} />
            )}
          </Button>
          <User />
        </Stack>
      </Flex>
    </Box>
  );
};
export default Header;

const UserNavCard: React.FC<{
  item: NavItem;
  onClick?: () => void;
  lastItem?: boolean;
}> = ({ item, onClick, lastItem }) => {
  return (
    <>
      {lastItem ? <Divider /> : <></>}
      <RouteLink href={item.to ? item.to : "#"} passHref>
        <Link
          onClick={onClick}
          role="group"
          display="block"
          p={2}
          rounded="md"
          _hover={{ bg: useColorModeValue("purple.50", "gray.900") }}
        >
          <Stack direction="row" align="center">
            <Box>
              <Text
                color={lastItem ? "red.300" : "grey.200"}
                transition="all .3s ease"
                _groupHover={{ color: lastItem ? "red.500" : "purple.400" }}
                fontWeight={500}
              >
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
              flex={1}
            >
              <Image
                color={lastItem ? "red.500" : "purple.400"}
                width={5}
                height={5}
                src={item.icon}
                alt={""}
              />
            </Flex>
          </Stack>
        </Link>
      </RouteLink>
    </>
  );
};
