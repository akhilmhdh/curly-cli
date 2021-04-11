pub enum RouteID {
    HomeScreen,
}

pub struct Route {
    pub id: RouteID,
}

/**
 * Why not default impl
 * The issue is it will be removed when scopes goes out on reference
 * So a persisting const
 */
const DEFAULT_ROUTE: Route = Route {
    id: RouteID::HomeScreen,
};

pub struct Navigation {
    routes: Vec<Route>,
}

impl Default for Navigation {
    fn default() -> Self {
        Navigation {
            routes: vec![DEFAULT_ROUTE],
        }
    }
}

impl Navigation {
    pub fn get_current_route(&self) -> &Route {
        self.routes.last().unwrap_or(&DEFAULT_ROUTE)
    }

    pub fn pop_naviation(&mut self) -> Option<Route> {
        self.routes.pop()
    }
}
