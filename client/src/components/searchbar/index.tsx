import * as React from "react"
import "./style.scss"

import SearchSVG from "assets/icons/search.svg"

export const SearchBar: React.FC<{
    onSearch: (query: string) => void
}> = ({ onSearch }) => {
    const callback = React.useCallback((event: React.FormEvent<HTMLFormElement>) => {
        event.preventDefault()
        const form = event.target as HTMLFormElement
        onSearch((form.elements.namedItem("search") as HTMLInputElement).value)
    }, [])

    return <div className="search-bar">
        <span className="icon">
            <SearchSVG/>
        </span>
        <form onSubmit={callback}>
            <input name="search"/>
        </form>
    </div>
}