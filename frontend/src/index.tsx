import * as React from "react"
import * as ReactDOM from "react-dom"
import { App } from "./components/app"
import CssBaseline from "@material-ui/core/CssBaseline"

import { 
  createMuiTheme,
  ThemeProvider
} from "@material-ui/core/styles"

const theme = createMuiTheme({
  palette: {
    type: "dark"
  }
})

ReactDOM.render(
  <ThemeProvider theme={theme}>
    <CssBaseline/>
    <App/>
  </ThemeProvider>,
  document.getElementById("root")
)