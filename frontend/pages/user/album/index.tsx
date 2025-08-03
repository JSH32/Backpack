import * as React from "react"

import { convertBytes } from "helpers/util"
import { Icon } from "@chakra-ui/icons"
import { FileSearch } from "components/FileSearch"
import { Page } from "layouts/Page"
import Router from "next/router"
import PlusIcon from "assets/icons/plus.svg"
import { Authenticated } from "components/Authenticated"
import {
  Box,
  Divider,
  Flex,
  Heading,
  Text,
  Stack,
  Stat,
  StatLabel,
  StatNumber,
  ToastId,
  useToast,
  Button,
  useDisclosure,
  Modal,
  ModalOverlay,
  ModalContent,
  ModalHeader,
  ModalCloseButton,
  ModalBody,
  FormControl,
  FormLabel,
  Input,
  ModalFooter,
  Checkbox
} from "@chakra-ui/react"
import api from "helpers/api"
import { useForm } from "react-hook-form"

const Albums: React.FC = () => {
  const toast = useToast()
  const { isOpen, onOpen, onClose } = useDisclosure()
  const albumForm = useForm()

  return <Authenticated>
    <Page title="Albums">
      <Flex mt="7em" justify="center">
        <Box w={{ base: "90vw", md: "70vw" }} maxW="1200px">
          <Stack spacing={4}>
            <Heading>Albums</Heading>
            <Text as="h2" fontSize="sm" color="gray.400">
              Create albums to share with others
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
            <Modal isOpen={isOpen} onClose={onClose}>
              <ModalOverlay />
              <ModalContent>
                <form onSubmit={albumForm.handleSubmit((data) => console.log(data))}>
                  <ModalHeader>Create Album</ModalHeader>
                  <ModalCloseButton />
                  <ModalBody >
                    <FormControl>
                      <FormLabel>Name</FormLabel>
                      <Input
                        placeholder={"Album Name"}
                        {...albumForm.register("name")}
                      />
                    </FormControl>
                    <FormControl mt={3}>
                      <FormLabel>Description</FormLabel>
                      <Input
                        placeholder={"Album Description"}
                        {...albumForm.register("description")}
                      />

                    </FormControl>
                    <FormControl mt={3}>
                      <Checkbox {...albumForm.register("public")}>Public</Checkbox>
                    </FormControl>
                  </ModalBody>
                  <ModalFooter>
                    <Button variant='ghost' mr={3} onClick={onClose}>
                      Cancel
                    </Button>
                    <Button colorScheme="primary" type="submit">Create</Button>
                  </ModalFooter>
                </form>
              </ModalContent>
            </Modal>
          </Stack>
        </Box>
      </Flex>
      {/* <Flex mt="7em" minH="100vh" justify="center" mb={5}>
                <Box w={{ base: "90vw", md: "70vw" }} maxW="1200px">
                    <Stack spacing={4}>
                        <Heading>Uploads</Heading>
                        <input type="file" ref={shadowUploader} onChange={uploadCallback} style={{ display: "none" }} multiple />
                        <Button
                            onClick={uploadButtonCallback}
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
                            <Icon w={5} h={5} as={UploadIcon} />
                        </Button>
                        <Divider />
                        <Box>
                            <Stat>
                                <StatLabel>Usage</StatLabel>
                                <StatNumber>{usage}</StatNumber>
                            </Stat>
                        </Box>
                        <Divider />
                        <FileSearch
                            key={searchReload}
                            onSearch={(page, query) => api.upload.list(page.toString(), "@me", query)}
                            onDelete={deleteCallback}
                            onFileDetails={fileId => Router.push(`/user/uploads/${fileId}`)} />
                    </Stack>
                </Box>
            </Flex> */}
    </Page>
  </Authenticated >
}

export default Albums
