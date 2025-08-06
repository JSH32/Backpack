// /* eslint-disable react/no-children-prop */
// import * as React from "react"
// import { SearchIcon } from "@chakra-ui/icons"
// import { useForm } from "react-hook-form"
// import { Pagination } from "../Pagination"
// import FileCard from "./FileCard"
// import FolderCard from "./FolderCard"
// import { AlbumUpdateData } from "./FolderCard"
// import {
//   Box,
//   Flex,
//   Heading,
//   Input,
//   InputGroup,
//   InputLeftElement,
//   Icon,
//   Spinner,
//   Text,
//   Divider
// } from "@chakra-ui/react"
// import { UploadData, AlbumData, Page_UploadData, Page_AlbumData } from "@/client"

// interface FileSearchResult {
//   albums?: AlbumData[]
//   files: UploadData[]
//   totalPages: number
//   currentPage: number
// }

// export const FileSearch: React.FC<{
//   onSearchFiles: (page: number, query?: string) => Promise<Page_UploadData>
//   onSearchAlbums?: (page: number, query?: string) => Promise<Page_AlbumData>
//   onDeleteFile: (fileId: string) => Promise<void>
//   onDeleteAlbum?: (albumId: string) => Promise<void>
//   onFileDetails: (fileId: string) => void
//   onAlbumOpen?: (albumId: string) => void
//   onAlbumUpdate?: (albumId: string, updates: AlbumUpdateData) => Promise<void>
//   showAlbums?: boolean
//   getUsernameForAlbum?: (userId: string) => Promise<string>
//   getFileCountForAlbum?: (albumId: string) => Promise<number>
// }> = ({
//   onSearchFiles,
//   onSearchAlbums,
//   onDeleteFile,
//   onDeleteAlbum,
//   onFileDetails,
//   onAlbumOpen,
//   onAlbumUpdate,
//   showAlbums = false,
//   getUsernameForAlbum,
//   getFileCountForAlbum
// }) => {
//     const [searchResult, setSearchResult] = React.useState<FileSearchResult | null>(null)
//     const [queryString, setQueryString] = React.useState<string>("")
//     const [currentPage, setCurrentPage] = React.useState(1)
//     const [initialLoading, setInitialLoading] = React.useState(true)
//     const [albumUsernames, setAlbumUsernames] = React.useState<Record<string, string>>({})
//     const [albumFileCounts, setAlbumFileCounts] = React.useState<Record<string, number>>({})

//     // Load initial data
//     React.useEffect(() => {
//       loadData(1, "")
//     }, [])

//     // Load data when page changes
//     React.useEffect(() => {
//       loadData(currentPage, queryString)
//     }, [currentPage])

//     const loadData = async (page: number, query: string) => {
//       try {
//         const promises: Promise<any>[] = [onSearchFiles(page, query)]

//         if (showAlbums && onSearchAlbums && page === 1) {
//           promises.push(onSearchAlbums(1, query))
//         }

//         const results = await Promise.all(promises)
//         const fileResult = results[0] as Page_UploadData
//         const albumResult = showAlbums && results[1] ? results[1] as Page_AlbumData : null

//         const unifiedResult: FileSearchResult = {
//           files: fileResult.items,
//           totalPages: fileResult.pages,
//           currentPage: page
//         }

//         if (albumResult && page === 1) {
//           unifiedResult.albums = albumResult.items

//           // Load usernames and file counts for albums
//           if (getUsernameForAlbum) {
//             const usernamePromises = albumResult.items.map(async (album) => {
//               try {
//                 const username = await getUsernameForAlbum(album.userId)
//                 return { albumId: album.id, username }
//               } catch {
//                 return { albumId: album.id, username: "Unknown" }
//               }
//             })

//             const usernameResults = await Promise.all(usernamePromises)
//             const usernameMap: Record<string, string> = {}
//             usernameResults.forEach(({ albumId, username }) => {
//               usernameMap[albumId] = username
//             })
//             setAlbumUsernames(usernameMap)
//           }

//           if (getFileCountForAlbum) {
//             const countPromises = albumResult.items.map(async (album) => {
//               try {
//                 const count = await getFileCountForAlbum(album.id)
//                 return { albumId: album.id, count }
//               } catch {
//                 return { albumId: album.id, count: 0 }
//               }
//             })

//             const countResults = await Promise.all(countPromises)
//             const countMap: Record<string, number> = {}
//             countResults.forEach(({ albumId, count }) => {
//               countMap[albumId] = count
//             })
//             setAlbumFileCounts(countMap)
//           }
//         }

//         setSearchResult(unifiedResult)
//       } catch (error) {
//         setSearchResult(null)
//       } finally {
//         setInitialLoading(false)
//       }
//     }

//     const searchCallback = React.useCallback((form: any) => {
//       setQueryString(form.query)
//       setCurrentPage(1)
//       loadData(1, form.query)
//     }, [])

//     const deleteFile = React.useCallback((fileId: string) => {
//       onDeleteFile(fileId)
//         .then(() => loadData(currentPage, queryString))
//         .catch(() => setSearchResult(null))
//     }, [currentPage, queryString])

//     const deleteAlbum = React.useCallback((albumId: string) => {
//       if (!onDeleteAlbum) return

//       onDeleteAlbum(albumId)
//         .then(() => loadData(1, queryString))
//         .catch(() => setSearchResult(null))
//     }, [queryString, onDeleteAlbum])

//     const updateAlbum = React.useCallback(async (album: AlbumData, updates: AlbumUpdateData) => {
//       if (!onAlbumUpdate) return

//       await onAlbumUpdate(album.id, updates)
//       loadData(currentPage, queryString)
//     }, [currentPage, queryString, onAlbumUpdate])

//     const { register, handleSubmit } = useForm()

//     const hasNoResults = searchResult &&
//       (!searchResult.albums || searchResult.albums.length === 0) &&
//       searchResult.files.length === 0

//     return (
//       <Box>
//         <form onSubmit={handleSubmit(searchCallback)}>
//           <InputGroup>
//             <InputLeftElement
//               color="gray.500"
//               children={<Icon as={SearchIcon} />}
//             />
//             <Input
//               variant="filled"
//               {...register("query")}
//               placeholder={showAlbums ? "Search for albums and files" : "Search for files"}
//             />
//           </InputGroup>
//         </form>

//         {initialLoading ? (
//           <Flex justify="center" align="center" mt={6}>
//             <Spinner size="lg" />
//           </Flex>
//         ) : searchResult === null ? (
//           <Box color="gray.500" textAlign="center">
//             <Heading size="xl" mt={6} mb={2}>
//               :(
//             </Heading>
//             <Heading as="h2" size="lg">
//               Error loading content
//             </Heading>
//             <Text>There was an error loading your content</Text>
//           </Box>
//         ) : hasNoResults ? (
//           <Box color="gray.500" textAlign="center">
//             <Heading size="xl" mt={6} mb={2}>
//               :(
//             </Heading>
//             <Heading as="h2" size="lg">
//               No content found
//             </Heading>
//             <Text>
//               {queryString
//                 ? "No albums or files matched your search"
//                 : "No albums or files found"
//               }
//             </Text>
//           </Box>
//         ) : (
//           <Flex justifyContent="center" gap="20px" wrap="wrap" mt={6}>
//             {/* Show albums on first page */}
//             {searchResult.albums && currentPage === 1 &&
//               searchResult.albums.map((album: AlbumData) => (
//                 <FolderCard
//                   key={`album-${album.id}`}
//                   album={album}
//                   fileCount={albumFileCounts[album.id] || 0}
//                   uploaderUsername={albumUsernames[album.id]}
//                   onOpen={(album: AlbumData) => onAlbumOpen?.(album.id)}
//                   onDelete={(album: AlbumData) => deleteAlbum(album.id)}
//                   onUpdate={onAlbumUpdate ? updateAlbum : undefined}
//                 />
//               ))
//             }

//             {/* Show files */}
//             {searchResult.files.map((file: UploadData) => (
//               <FileCard
//                 key={`file-${file.id}`}
//                 file={file}
//                 onDetails={(file: UploadData) => onFileDetails(file.id)}
//                 onDelete={(file: UploadData) => deleteFile(file.id)}
//               />
//             ))}
//           </Flex>
//         )}

//         {(searchResult?.totalPages || 0) > 1 && (
//           <Flex justifyContent="center" mt={5}>
//             <Pagination
//               pages={searchResult?.totalPages || 1}
//               currentPage={currentPage}
//               range={3}
//               onPageSelect={setCurrentPage}
//             />
//           </Flex>
//         )}
//       </Box>
//     )
//   }

// export default FileSearch