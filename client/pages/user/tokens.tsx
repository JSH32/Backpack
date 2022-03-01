import * as React from "react"
import {
  Box,
  Divider,
  Flex,
  Heading,
  Stack,
  Button,
  Icon,
  useToast
} from "@chakra-ui/react"
import type { NextPage } from "next"
import { Authenticated } from "components/Authenticated"
import { Page } from "layouts/Page"
import PlusIcon from "assets/icons/plus.svg"
import { applicationCreate } from "helpers/api"

const Tokens: NextPage = () => {
  const toast = useToast()

  const plusButtonCallback = React.useCallback(() => {
    applicationCreate("test").then(data => {
        toast({
            title: "Success",
            description: `Created application ${data.name}`,
            status: "success",
            duration: 5000,
            isClosable: true
        })
    }).catch(err => {
        toast({
            title: "Error",
            description: `Failed to create application: ${err.message}`,
            status: "error",
            duration: 5000,
            isClosable: true
        })
    })
  }, [])
  return (
    <Authenticated allowUnverified>
      <Page>
        <Flex mt="7em" justify="center">
          <Box w={{ base: "90vw", md: "700px" }}>
            <Stack spacing={4}>
              <Heading>Tokens</Heading>
              <Button
                onClick={plusButtonCallback}
                bg="primary.500"
                _hover={{
                  bg: "primary.600"
                }}
                color="white"
                position="fixed"
                bottom="3em"
                right="3em"
                borderRadius="50px"
                w="60px"
                h="60px"
                zIndex={4}
              >
                <Icon w={5} h={5} as={PlusIcon} />
              </Button>
              <Divider />
              <Stack direction={{ base: "column", md: "row" }} spacing={8}>
                <Box></Box>
              </Stack>
            </Stack>
          </Box>
        </Flex>
      </Page>
    </Authenticated>
  )
}

export default Tokens
