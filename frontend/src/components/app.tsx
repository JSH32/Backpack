import * as React from "react"

import {
  BrowserRouter as Router,
  Switch,
  Route,
} from "react-router-dom"

import { Home } from "routes/home"
import { Header } from "./header"

export const App = () => (
  <div>
    <Header/>
    <Router>
      <div>
        <Route path="/home" component={Home}/>
        <Route path="/" component={Home}/>
      </div>
    </Router>
  </div>
);