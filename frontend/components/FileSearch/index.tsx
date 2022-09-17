/* eslint-disable react/no-children-prop */
import * as React from "react"
import { SearchIcon } from "@chakra-ui/icons"
import { useForm } from "react-hook-form"
import { Pagination } from "../Pagination"
import FileCard from "./FileCard"
import {
  Box,
  Flex,
  Heading,
  Input,
  InputGroup,
  InputLeftElement,
  Icon,
  Spinner,
  Text
} from "@chakra-ui/react"
import { FileData, FilePage } from "@/client"

export const FileSearch: React.FC<{
  onSearch: (page: number, query?: string) => Promise<FilePage>
  onDelete: (fileId: string) => Promise<void>
  onFileDetails: (fileId: string) => void
}> = ({ onSearch, onDelete, onFileDetails }) => {
  const [searchResult, setSearchResult] =
    React.useState<FilePage | null>(null)

  const [queryString, setQueryString] = React.useState<string>("")
  const [currentPage, setCurrentPage] = React.useState(1)
  const [initialLoading, setInitialLoading] = React.useState(false)
  
  React.useEffect(() => {
    setInitialLoading(true)
    onSearch(1, "")
      .then(setSearchResult)
      .catch(() => setSearchResult(null))
      .finally(() => setInitialLoading(false))
  }, [])

  const searchCallback = React.useCallback((form: any) => {
    setQueryString(form.query)
    // Search callback should go back to page 1
    onSearch(1, form.query)
      .then(setSearchResult)
      .catch(() => setSearchResult(null))
  }, [])

  React.useEffect(() => {
    onSearch(currentPage, queryString)
      .then(setSearchResult)
      .catch(() => setSearchResult(null))
  }, [currentPage])

  const deleteFile = React.useCallback((fileId: string) => {
    onDelete(fileId)
      .then(() => onSearch(1, queryString))
      .then(setSearchResult)
      .catch(() => setSearchResult(null))
  }, [])

  const { register, handleSubmit } = useForm()

  return (
    <Box>
      <form onSubmit={handleSubmit(searchCallback)}>
        <InputGroup>
          <InputLeftElement
            color="gray.500"
            children={
              <Icon as={SearchIcon} />
            }
          />
          <Input
            variant="filled"
            {...register("query")}
            placeholder="Search for files"
          />
        </InputGroup>
      </form>

      {initialLoading ? (
        <Flex justify="center" align="center" mt={6}>
          <Spinner size="lg" />
        </Flex>
      ) : searchResult === null ? (
        <Box color="gray.500" textAlign="center">
          <Heading size="xl" mt={6} mb={2}>
            :(
          </Heading>
          <Heading as="h2" size="lg">
            No files found
          </Heading>
          <Text>There were no files matched your query</Text>
        </Box>
      ) : (
        <Flex justifyContent="center" gap="20px" wrap="wrap" mt={6}>
          {searchResult.items.map((file: FileData) => (
            <FileCard
              key={file.id}
              file={file}
              onDetails={(file: FileData) => onFileDetails(file.id)}
              onDelete={(file: FileData) => deleteFile(file.id)}
            />
          ))}
        </Flex>
      )}

      {(searchResult?.pages || 0) > 1 ? (
        <Flex justifyContent="center" mt={5}>
          <Pagination
            pages={searchResult?.pages || 1}
            currentPage={currentPage}
            range={3}
            onPageSelect={setCurrentPage}
          />
        </Flex>
      ) : (
        <></>
      )}
    </Box>
  )
}
