import * as React from "react"
import {
  Box,
  Divider,
  Table,
  Th,
  Flex,
  Heading,
  Stack,
  Button,
  Icon,
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
  Thead,
  Tr,
  Td,
  Tbody
} from "@chakra-ui/react"
import type { NextPage } from "next"
import { Authenticated } from "components/Authenticated"
import PlusIcon from "assets/icons/plus.svg"
import {
  applicationCreate,
  ApplicationData,
  getAllApplications,
  getApplication,
  getApplicationToken,
  deleteApplication
} from "helpers/api"
import { Page } from "layouts/Page"

const Tokens: NextPage = () => {
  const toast = useToast()
  const { isOpen, onOpen, onClose } = useDisclosure()
  const [applications, setApplications] = React.useState<ApplicationData[]>([])
  const [applicationName, setApplicationName] = React.useState("")

  React.useEffect(() => {
    getAllApplications()
      .then((data) => setApplications(data))
      .catch(() => [])
  }, [])

  return (
    <Authenticated allowUnverified>
      <Page title="Tokens">
        <Flex mt="7em" justify="center">
          <Box w={{ base: "90vw", md: "700px" }}>
            <Stack spacing={4}>
              <Heading>Tokens</Heading>
              {applications.length <= 4 && (
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
                zIndex={4}
              >
                <Icon w={5} h={5} as={PlusIcon} />
              </Button>
              )}
              <Modal isOpen={isOpen} onClose={onClose}>
                <ModalOverlay />
                <ModalContent>
                  <ModalHeader>Create Token</ModalHeader>
                  <ModalCloseButton />
                  <ModalBody pb={6}>
                    <FormControl>
                      <FormLabel>Name</FormLabel>
                      <Input
                        focusBorderColor={"primary.600"}
                        placeholder={"Application Name"}
                        onChange={(val) => setApplicationName(val.target.value)}
                      />
                    </FormControl>
                  </ModalBody>
                  <ModalFooter>
                    <Button
                      onClick={() => {
                        applicationCreate(applicationName)
                          .then((data) => {
                            toast({
                              title: "Success",
                              description: `Application created with the id: ${data.id}`,
                              status: "success",
                              duration: 9000,
                              isClosable: true
                            })
                            onClose()
                            setApplications(applications.concat(data))
                          })
                          .catch((err) => {
                            toast({
                              title: "Error",
                              description: `${err.response?.data?.message ??
                                "An unknown error occured"
                                }`,
                              status: "error",
                              duration: 9000,
                              isClosable: true
                            })
                          })
                      }}
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
                      Add
                    </Button>
                  </ModalFooter>
                </ModalContent>
              </Modal>
              <Divider />
              {!applications || applications.length === 0 ? (
                <Box color="gray.500" textAlign="center">
                  <Heading size="xl" mt={6} mb={2}>
                    :(
                  </Heading>
                  <Heading as="h2" size="lg">
                    No Tokens found
                  </Heading>
                </Box>
              ) : (
                <Stack direction={{ base: "column", md: "row" }} spacing={8}>
                  <Table
                    wordBreak="break-all"
                    sx={{ "font-variant-numeric": "unset;" }}
                  >
                    <Thead>
                      <Tr>
                        <Td>Name</Td>
                        <Th>ID</Th>
                        <Th>Last Accessed</Th>
                      </Tr>
                    </Thead>
                    <Tbody>
                      {applications.map((application) => (
                          <Tr
                            key={application.id}
                            _hover={{ bg: "primary.400" }}
                          >
                            <Td>{application.name}</Td>
                            <Td>{application.id}</Td>
                            <Td>15days ago</Td>
                          </Tr>
                      ))}
                    </Tbody>
                  </Table>
                </Stack>
              )}
            </Stack>
          </Box>
        </Flex>
        </Page>
    </Authenticated>
  )
}

export default Tokens
