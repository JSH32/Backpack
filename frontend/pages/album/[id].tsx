import { AlbumData } from "@/client"
import { Box, chakra, Flex, Spinner, Text } from "@chakra-ui/react"
import { Resource } from "components/Resource"
import { Result } from "components/Result"
import api from "helpers/api"
import { Page } from "layouts/Page"
import { useRouter } from "next/router"
import * as React from "react"

const Album: React.FC = () => {
  const router = useRouter()
  const { id } = router.query

  const [albumData, setAlbumData] = React.useState<AlbumData | null>(null)
  const [isError, setIsError] = React.useState(false)

  React.useEffect(() => {
    api.album.info(id as string)
      .then(setAlbumData)
      .catch(() => setIsError(true))
  }, [])

  return (
    <Box>
      {isError ? (
        <Page title="Invalid File">
          <Result type="error" title="Invalid resource">
            <Text>
              <chakra.span fontWeight="bold">{id}</chakra.span> was an invalid resource
            </Text>
          </Result>
        </Page>
      ) : (
        albumData ? (
          <Resource title={albumData.name} id={albumData.id}>
            <p>{albumData.description}</p>
          </Resource>
        ) : (
          <Page>
            <Flex h="100vh" justify="center" align="center">
              <Spinner size="xl" />
            </Flex>
          </Page>
        )
      )}
    </Box>
  )
}

export default Album