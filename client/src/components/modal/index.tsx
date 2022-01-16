import * as React from "react"
import XSvg from "../../assets/icons/x.svg"

import "./style.scss"

export const Modal: React.FC<{ 
    onClose: () => void
}> = ({ children, onClose }) => {
    return <div className="modal">
        <div className="modal-main">
            <a className="modal-exit" onClick={onClose}><XSvg/></a>
            {children}
        </div>
    </div>
}