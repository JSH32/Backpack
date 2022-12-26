import { AlbumData } from "@/client"
import { Flex, Spinner } from "@chakra-ui/react"
import { Resource } from "components/Resource"
import api from "helpers/api"
import { useRouter } from "next/router"
import * as React from "react"

const Album: React.FC = () => {
    const router = useRouter()
    const { id } = router.query

    const [albumData, setAlbumData] = React.useState<AlbumData | null>(null)
    const [loaded, setLoaded] = React.useState(false)

    React.useEffect(() => {
        api.album.info(id as string)
            .then(setAlbumData)
            .finally(() => setLoaded(true))
    }, [])

    return loaded && albumData ? <>
        <Resource 
            title={albumData.name}
            id={albumData.id}>
            <p>{albumData.description}</p>
        </Resource>
    </> : <></>
}

export default Album