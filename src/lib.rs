#![feature(error_generic_member_access)]

pub mod route;

pub use route::Route;

pub struct Routes<T> {
    routes: Vec<Route<T>>,
}

impl<T> Routes<T> {
    pub fn new() -> Routes<T> {
        Routes {
            routes: Vec::new(),
        }
    }

    pub fn add(&mut self, route: Route<T>) {
        self.routes.push(route);
    }

    pub fn find_route(&self, _uri: &str) -> Option<Route<T>> {
        todo!()
    }
}

impl<T> Default for Routes<T> {
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
