import * as React from "react"
import "./style.scss"

import FeatherSVG from "assets/icons/search.svg"
import FileSvg from "assets/icons/file.svg"
import { FileCard } from "./file"
import { SearchResult } from "api"

export const FileSearch: React.FC<{
    onSearch: (page: number, query?: string) => Promise<SearchResult>
}> = ({ onSearch }) => {
    const [searchResult, setSearchResult] = React.useState<SearchResult | null>(null)
    
    React.useEffect(() => {
        onSearch(1, null)
            .then((v) => {
                setSearchResult(v)
                console.log(v)
            })
            .catch(() => setSearchResult(null))
    }, [])

    const formSubmit = (event: React.FormEvent<HTMLFormElement>) => {
        event.preventDefault()
        const form = event.target as HTMLFormElement
        const search = (form.elements.namedItem("search") as HTMLInputElement).value

        onSearch(1, search)
            .then(setSearchResult)
            .catch(() => setSearchResult(null))
    }
    
    return <div className="file-search">
        <div className="search-bar">
            <span className="icon">
                <FeatherSVG/>
            </span>
            <form onSubmit={formSubmit}>
                <input name="search"/>
            </form>
        </div>
        { searchResult === null ? <div className="no-result">
            <FileSvg/>
            <p>No files were found</p>
        </div> :
        <div className="file-list">
            { searchResult.files.map(file => <FileCard file={file}/>) }
        </div> }
    </div>
}