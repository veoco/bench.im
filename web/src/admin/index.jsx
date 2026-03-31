import { useEffect } from "react";
import { Route, useLocation, useRoute, Switch } from "wouter";

import IndexPage from "./index_page";
import LoginPage from "./login_page";
import EditMachinePage from "./edit_machine_page";
import DeleteMachinePage from "./delete_machine_page";
import EditTargetPage from "./edit_target_page";
import DeleteTargetPage from "./delete_target_page";

export default function Admin({ isLogin, setIsLogin }) {
  const [location, setLocation] = useLocation();

  const [isLoginPage] = useRoute("/login");

  useEffect(() => {
    if (!isLoginPage && !isLogin) {
      setLocation("/login");
    }
  }, [isLogin, isLoginPage]);

  return (
    <Switch>
      <Route path="/login">
        <LoginPage isLogin={isLogin} setIsLogin={setIsLogin} />
      </Route>
      <Route path="/" component={IndexPage} />
      <Route path="/machines/new">
        <EditMachinePage params={{}} />
      </Route>
      <Route path="/machines/:mid" component={EditMachinePage} />
      <Route path="/machines/:mid/delete" component={DeleteMachinePage} />
      <Route path="/targets/new">
        <EditTargetPage params={{}} />
      </Route>
      <Route path="/targets/:tid" component={EditTargetPage} />
      <Route path="/targets/:tid/delete" component={DeleteTargetPage} />
    </Switch>
  )
}