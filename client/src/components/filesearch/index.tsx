import * as React from "react"
import "./style.scss"

import FileSvg from "assets/icons/file.svg"
import TrashSvg from "assets/icons/trash.svg"
import OpenSvg from "assets/icons/open.svg"
import { FileCard } from "./file"
import { FileData, SearchResult } from "api"
import { SearchBar } from "components/searchbar"
import ReactPaginate from "react-paginate"
import { Modal } from "components/modal"
import { convertBytes, dateToString, getExtension, isExtImage } from "bpkutil"

export const FileSearch: React.FC<{
    onSearch: (page: number, query?: string) => Promise<SearchResult>
    onDelete: (fileId: number) => Promise<void>
}> = ({ onSearch, onDelete }) => {
    const [searchResult, setSearchResult] = React.useState<SearchResult | null>(null)
    const [queryString, setQueryString] = React.useState<string>(null)

    const [contextAnchorPoint, setContextAnchorPoint] = React.useState({ x: 0, y: 0 })
    const [showContext, setShowContext] = React.useState(false)
    const [contextFileData, setContextFileData] = React.useState<FileData>(null)
    const [fileModalOpen, setFileModalOpen] = React.useState(false)

    React.useEffect(() => {
        onSearch(1, null)
            .then(setSearchResult)
            .catch(() => setSearchResult(null))

        // Listen on clicks to remove context menu
        const clickListener = () => setShowContext(false)
        document.addEventListener("click", clickListener)

        const visibilityListener = () => setShowContext(false)
        document.addEventListener("visibilitychange", visibilityListener)

        return () => {
            document.removeEventListener("click", clickListener)
            document.removeEventListener("visibilitychange", visibilityListener)
        }
    }, [])

    const searchCallback = React.useCallback(query => {
        setQueryString(query)

        // Search callback should go back to page 1
        onSearch(1, query)
            .then(setSearchResult)
            .catch(() => setSearchResult(null))
    }, [])

    const paginationCallback = React.useCallback(event => {
        onSearch(event.selected + 1, queryString)
            .then(setSearchResult)
            .catch(() => setSearchResult(null))
    }, [queryString])

    const handleContextMenu = React.useCallback((event, fileData) => {
        event.preventDefault()
        // This prevents default event from being called since default event hides the context menu
        event.stopPropagation()
        setContextFileData(fileData)
        setContextAnchorPoint({ x: event.pageX, y: event.pageY })
        setShowContext(true)
    }, [setContextAnchorPoint, setShowContext])

    const deleteFile = React.useCallback(fileId => {
        console.log(fileId)
        onDelete(fileId)
            .then(() => onSearch(1, queryString))
            .then(setSearchResult)
            .catch(() => setSearchResult(null))
    }, [])

    const onHashClick = React.useCallback((event) => {
        const originalText = event.target.innerHTML

        navigator.clipboard.writeText(contextFileData.hash)
        event.target.innerHTML = "Text copied to clipboard"

        // Change text back to original
        setTimeout(() => event.target.innerHTML = originalText, 1000)
    }, [contextFileData])

    return <div className="file-search">
        { fileModalOpen ? <div className="file-modal"><Modal onClose={() => setFileModalOpen(false)}>
            <h1>{contextFileData.name}</h1>
            { isExtImage(getExtension(contextFileData.name)) ? 
            <div className="modal-image"><img src={contextFileData.url}/></div> : <></> }
            <table>
                <tr>
                    <td>ID</td>
                    <td>{contextFileData.id}</td>
                </tr>
                <tr>
                    <td>Name</td>
                    <td>{contextFileData.name}</td>
                </tr>
                <tr>
                    <td>Original Name</td>
                    <td>{contextFileData.originalName}</td>
                </tr>
                <tr>
                    <td>Size</td>
                    <td>{convertBytes(contextFileData.size)}</td>
                </tr>
                <tr>
                    <td>Date Uploaded</td>
                    <td>{dateToString(contextFileData.uploaded)}</td>
                </tr>
                <tr>
                    <td>URL</td>
                    <td><a href={contextFileData.url} target="_blank">{contextFileData.url}</a></td>
                </tr>
                <tr>
                    <td>Hash</td>
                    <td><a onClick={onHashClick}>Click to copy hash</a></td>
                </tr>
            </table>
        </Modal></div> : <></>}
        <SearchBar onSearch={searchCallback}/>
        { searchResult === null ? <div className="no-result">
            <FileSvg/>
            <div className="message">
                <h2>No files found</h2>
                <p>No files matched your query</p>
            </div>
        </div> : <>
        { showContext ? <ul className="context-menu" style={{
                top: contextAnchorPoint.y, 
                left: contextAnchorPoint.x
        }}>
            <li className="context-item" onClick={() => setFileModalOpen(true)}>
                <FileSvg/>
                Details
            </li>
            <li className="context-item" onClick={() => window.open(contextFileData.url)}>
                <OpenSvg/>
                Open in tab
            </li>
            <li className="context-item" onClick={() => deleteFile(contextFileData.id)}>
                <TrashSvg/>
                Delete
            </li>
        </ul> : <></>}
        <div className="file-list">
            { searchResult.files.map(file => 
                <FileCard onClick={e => handleContextMenu(e, file)} key={file.id} file={file}/>) }
        </div>
        { searchResult.pages > 1 ? <div className="pagination">
            <ReactPaginate
                pageCount={searchResult.pages}
                forcePage={searchResult.page - 1}
                pageRangeDisplayed={5}
                marginPagesDisplayed={1}
                onPageChange={paginationCallback}
                breakLabel="..."
                activeLinkClassName="selected"/>
        </div> : null }
        </>}
    </div>
}