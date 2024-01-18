pub mod route;

pub use route::{Parser, Route};

#[derive(Debug, Clone, PartialEq)]
pub struct Routes {
    routes: Vec<Route>,
}

impl Routes {
    pub fn new() -> Routes {
        Routes { routes: Vec::new() }
    }

    pub fn add(&mut self, route: Route) {
        self.routes.push(route);
    }

    pub fn find(&self, path: &str) -> Option<&Route> {
        self.routes.iter().find(|&route| route.check(path))
    }
}

impl Default for Routes {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_routes() {
        let mut routes: Routes = Routes::new();
        let parser = Parser::default();

        let user_route = parser
            .route("user-by-id", "/user/<id:int>/")
            .expect("route should parse");
        let club_route = parser
            .route("club-by-id", "/club/<id:uuid>/")
            .expect("route should parse");
        let game_route = parser
            .route("game", "/game/<slug>/")
            .expect("route should parse");

        routes.add(user_route.clone());
        routes.add(club_route.clone());
        routes.add(game_route.clone());

        assert_eq!(None, routes.find("/user/abc/"));
        assert_eq!(Some(&user_route), routes.find("/user/123/"));
        assert_eq!(None, routes.find("/club//"));
        assert_eq!(None, routes.find("/club/abc/"));
        assert_eq!(None, routes.find("/club/123/"));
        assert_eq!(
            Some(&club_route),
            routes.find("/club/36be8705-6c31-45d7-9321-d56cc07b50d9/")
        );
        assert_eq!(Some(&game_route), routes.find("/game/123/"));
        assert_eq!(Some(&game_route), routes.find("/game//"));
        assert_eq!(Some(&game_route), routes.find("/game/abc/"));
        assert_eq!(None, routes.find("/game/123"));
    }
}
