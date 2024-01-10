#![feature(error_generic_member_access)]

pub mod route;

pub use route::Route;

pub struct Routes {
    routes: Vec<Route>,
}

impl Routes {
    pub fn new() -> Routes {
        Routes {
            routes: Vec::new(),
        }
    }

    pub fn add(&mut self, route: Route) {
        self.routes.push(route);
    }

    pub fn find_route(&self, _uri: &str) -> Option<Route> {
        todo!()
    }
}

impl Default for Routes {
    fn default() -> Self {
        Self::new()
    }
}


//#[cfg(test)]
//mod tests {
//    use super::*;
//
//    #[test]
//    fn test_routes() {
//        let mut routes: Routes<u32> = Routes::new();
//
//        let user_route_spec = "/user/<id>/";
//        let user_route = Route::parse(&user_route_spec).expect("route should parse");
//
//        routes.add(user_route);
//    }
//}
