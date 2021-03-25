import * as React from "react"

import {
  BrowserRouter as Router,
  Route
} from "react-router-dom"

import { Home } from "routes/home"
import { Header } from "components/header"

// Global stylesheet
import "./style.css"

export const App: React.FunctionComponent = () => (
  <div>
    <Header/>
    <Router>
      <div>
        <Route path="/home" component={Home}/>
        <Route path="/" component={Home}/>
      </div>
    </Router>
  </div>
)