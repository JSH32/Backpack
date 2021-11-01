import * as React from "react"
import "./style.scss"

import FileSvg from "assets/icons/file.svg"
import { FileCard } from "./file"
import { SearchResult } from "api"
import { SearchBar } from "components/searchbar"
import ReactPaginate from "react-paginate"

export const FileSearch: React.FC<{
    onSearch: (page: number, query?: string) => Promise<SearchResult>
}> = ({ onSearch }) => {
    const [searchResult, setSearchResult] = React.useState<SearchResult | null>(null)
    const [queryString, setQueryString] = React.useState<string>(null)

    React.useEffect(() => {
        onSearch(1, null)
            .then(setSearchResult)
            .catch(() => setSearchResult(null))
    }, [])

    const searchCallback = React.useCallback(query => {
        setQueryString(query)

        // Search callback should go back to page 1
        onSearch(1, query)
            .then(setSearchResult)
            .catch(() => setSearchResult(null))
    }, [])

    const paginationCallback = React.useCallback(event => {
        console.log(event.selected + 1)
        onSearch(event.selected + 1, queryString)
            .then(setSearchResult)
            .catch(() => setSearchResult(null))
    }, [queryString])

    return <div className="file-search">
        <SearchBar onSearch={searchCallback}/>
        { searchResult === null ? <div className="no-result">
            <FileSvg/>
            <div className="message">
                <h2>No files found</h2>
                <p>No files matched your query</p>
            </div>
        </div> : <>
        <div className="file-list">
            { searchResult.files.map(file => <FileCard key={file.id} file={file}/>) }
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