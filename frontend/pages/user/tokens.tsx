import * as React from "react"

import {
  Box,
  Divider,
  Flex,
  Heading,
  Stack,
  Button,
  Icon,
  Link,
  Text,
  useToast,
  Modal,
  ModalOverlay,
  ModalContent,
  ModalHeader,
  ModalFooter,
  ModalBody,
  ModalCloseButton,
  FormControl,
  FormLabel,
  Input,
  useDisclosure,
  Menu,
  MenuButton,
  MenuItem,
  MenuList,
  Portal,
  VStack,
  Center,
  Spinner
} from "@chakra-ui/react"

import type { NextPage } from "next"
import { Authenticated } from "components/Authenticated"
import { Page } from "layouts/Page"
import { useForm } from "react-hook-form"
import { DataList, DataListCell, DataListHeader, DataListRow } from "components/DataList"
import { Pagination } from "components/Pagination"
import PlusIcon from "assets/icons/plus.svg"
import TrashIcon from "assets/icons/trash.svg"
import KeyIcon from "assets/icons/key.svg"
import ClipboardIcon from "assets/icons/clipboard.svg"
import MoreVerticalIcon from "assets/icons/more-vertical.svg"
import { timeAgo, copyText } from "helpers/util"
import { ApplicationData, ApplicationPage } from "@/client"
import api from "helpers/api"

const Tokens: NextPage = () => {
  const toast = useToast()
  const { isOpen, onOpen, onClose } = useDisclosure()
  const [applications, setApplications] = React.useState<ApplicationPage | null>()
  const tokenForm = useForm()
  const [loadingTokens, setLoadingTokens] = React.useState(true)

  const onDeleteApplication = (id: string) => {
    api.application.delete(id)
      .then(() => {
        getApplicationPage(1)
        toast({
          title: "Success",
          description: "Application deleted",
          status: "success",
          duration: 5000,
          isClosable: true
        })
      })
      .catch(error => {
        toast({
          title: "Error",
          description: error.body.message,
          status: "error",
          duration: 5000,
          isClosable: true
        })
      })
  }

  const getApplicationPage = React.useCallback((page: number) => {
    api.application.list(page)
      .then(data => {
        setApplications(data)
        setLoadingTokens(false)
      })
      .catch(() => {
        setApplications(null)
        setLoadingTokens(false)
      })
  }, [])

  const onCopyToken = React.useCallback((id: string) => {
    api.application.token(id)
      .then(res => {
        copyText(res.token)
        toast({
          title: "Token copied",
          description: "Token copied to clipboard",
          status: "success",
          duration: 5000,
          isClosable: true
        })
      })
      .catch(err => {
        toast({
          title: "Error",
          description: err.body.message,
          status: "error",
          duration: 5000,
          isClosable: true
        })
      })
  }, [])

  const createApplication = React.useCallback((form: any) => {
    api.application.create(form)
      .then(res => {
        toast({
          title: "Success",
          description: `Application created with name: ${res.name}`,
          status: "success",
          duration: 5000,
          isClosable: true
        })
        closeForm()
        getApplicationPage(1)
      })
      .catch(error => {
        toast({
          title: "Error",
          description: error.body.message,
          status: "error",
          duration: 5000,
          isClosable: true
        })
      })
  }, [applications])

  const closeForm = React.useCallback(() => {
    tokenForm.resetField("name")
    onClose()
  }, [tokenForm.reset])

  // Initial load page 1.
  React.useEffect(() => {
    getApplicationPage(1)
  }, [])

  return (
    <Authenticated>
      <Page title="Tokens">
        <Flex mt="7em" justify="center">
          <Box w={{ base: "90vw", md: "700px" }}>
            <Stack spacing={4}>
              <Heading>Tokens</Heading>
              <Text as="h2" fontSize="sm" color="gray.400">
                Tokens allow developers to use the Backpack API on behalf of a user account.
              </Text>

              <Button
                onClick={onOpen}
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
                zIndex={4}>
                <Icon w={5} h={5} as={PlusIcon} />
              </Button> 
              <Modal isOpen={isOpen} onClose={closeForm}>
                <ModalOverlay />
                <ModalContent>
                  <form onSubmit={tokenForm.handleSubmit(createApplication)}>
                    <ModalHeader>Create Token</ModalHeader>
                    <ModalCloseButton />
                    <ModalBody pb={6}>
                      <FormControl>
                        <FormLabel>Name</FormLabel>
                        <Input
                          placeholder={"Application Name"}
                          {...tokenForm.register("name")}
                        />
                      </FormControl>
                    </ModalBody>
                    <ModalFooter>
                      <Button variant='ghost' mr={3} onClick={closeForm}>
                        Cancel
                      </Button>
                      <Button colorScheme="primary" type="submit">Create</Button>
                    </ModalFooter>
                  </form>
                </ModalContent>
              </Modal>
              <Divider />
              { applications !== null ? <DataList>
                <DataListHeader>
                  <DataListCell
                    colName="name"
                    isVisible={{ base: true, lg: true }}>
                    <span>Name</span>
                  </DataListCell>
                  <DataListCell
                    colName="id"
                    isVisible={{ base: false, lg: true }}>
                    <span>ID</span>
                  </DataListCell>
                  <DataListCell
                    colName="lastAccessed"
                    isVisible={{ base: true, lg: true }}>
                    <span>Last Accessed</span>
                  </DataListCell>
                  <DataListCell colWidth="3rem" colName="actions" align="flex-end" />
                </DataListHeader>
                {applications?.items.map(application => (
                  <DataListRow key={application.id}>
                    <DataListCell colName="name">
                      <Text>{application.name}</Text>
                    </DataListCell>
                    <DataListCell colName="id">
                      <Text>{application.id}</Text>
                    </DataListCell>
                    <DataListCell colName="lastAccessed">
                      <Text>{timeAgo.format(new Date(application.lastAccessed))}</Text>
                    </DataListCell>
                    <DataListCell colName="actions" >
                      <Menu placement="bottom-end">
                        <MenuButton as={Button} variant="ghost">
                          <Icon as={MoreVerticalIcon} />
                        </MenuButton>
                        <Portal>
                          <MenuList>
                            <MenuItem icon={<Icon mt="6px" as={ClipboardIcon} />} onClick={() => onCopyToken(application.id)}>Copy</MenuItem>
                            <MenuItem icon={<Icon mt="6px" as={TrashIcon} />} onClick={() => onDeleteApplication(application.id)}>Delete</MenuItem>
                          </MenuList>
                        </Portal>
                      </Menu>
                    </DataListCell>
                  </DataListRow>
                ))}
              </DataList> : <DataList>
                <Center h="9rem">
                  {loadingTokens ? <Spinner size="lg"/> : <VStack color="gray.500">
                    <Icon as={KeyIcon} w="30px" h="30px"/>
                    <Heading as="h2" size="2lg">
                      No Tokens found. <Link color="primary.400" onClick={onOpen}>Create one</Link>
                    </Heading>
                  </VStack>}
                </Center>  
              </DataList>}
              {applications && (
                <Flex justifyContent="center" mt={5}>
                  <Pagination
                    pages={applications.pages}
                    currentPage={applications.page}
                    range={3}
                    onPageSelect={(page) => getApplicationPage(page)}
                  />
                </Flex>
              )}
            </Stack>
          </Box>
        </Flex>
      </Page>
    </Authenticated>
  )
}

export default Tokens
