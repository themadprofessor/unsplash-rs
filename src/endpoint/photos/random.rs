use futures::Future;
use hyper::{client::connect::Connect, Client, Uri};
use itertools::*;

use super::{Orientation, Photo};
use error::*;

lazy_static! {
    /// URI of the endpoint to get random photos from Unsplash.
    pub static ref RANDOM_URI: Uri = format!("{}{}", ::API_URL, "photos/random").parse().unwrap();
}

/// Request builder for creating a Random request.
#[derive(Debug, Default)]
pub struct Random {
    featured: Option<bool>,
    username: Option<String>,
    w: Option<usize>,
    h: Option<usize>,
    orientation: Option<Orientation>,
}

/// Session type to handle returning a list of random photos.
#[derive(Debug, Default)]
pub struct RandomCount {
    rand: Random,
    count: usize,
}

/// Session type to handle restricting the photos to a query.
#[derive(Debug, Default)]
pub struct RandomQuery {
    rand: Random,
    query: String,
}

/// Session type to handle returning a list of random photos restricted by a
/// query.
#[derive(Debug, Default)]
pub struct RandomQueryCount {
    rand: RandomQuery,
    count: usize,
}

/// Session type to handle restricting the photos to a set of collections.
#[derive(Debug, Default)]
pub struct RandomCollection {
    rand: Random,
    collection: String,
}

/// Session type to handle returning a list of random photos restricted by a
/// set of collections.
#[derive(Debug, Default)]
pub struct RandomCollectionCount {
    rand: RandomCollection,
    count: usize,
}

/// Serialization type
#[derive(Debug, Default, Serialize)]
struct RandomSerialize {
    featured: Option<bool>,
    username: Option<String>,
    w: Option<usize>,
    h: Option<usize>,
    orientation: Option<Orientation>,
    collection: Option<String>,
    query: Option<String>,
}

/// Serialization type with a count
#[derive(Debug, Default, Serialize)]
struct RandomCountSerialize {
    featured: Option<bool>,
    username: Option<String>,
    w: Option<usize>,
    h: Option<usize>,
    orientation: Option<Orientation>,
    collection: Option<String>,
    query: Option<String>,
    count: usize,
}

impl Random {
    /// Restrict the photos to only featured photos.
    pub fn featured(mut self, feat: bool) -> Self {
        self.featured.replace(feat);
        self
    }

    /// Restrict the photos to only photos by the given user.
    pub fn username(mut self, username: String) -> Self {
        self.username.replace(username);
        self
    }

    /// Restrict the photos to only photos with the given width.
    pub fn w(mut self, w: usize) -> Self {
        self.w.replace(w);
        self
    }

    /// Restrict the photos to only photos with the given height.
    pub fn h(mut self, h: usize) -> Self {
        self.h.replace(h);
        self
    }

    /// Restrict the photos to only photos with the given orientation.
    pub fn orientation(mut self, orientation: Orientation) -> Self {
        self.orientation.replace(orientation);
        self
    }

    /// Restrict the photos to only photos which match the given query. NOTE:
    /// only the number of photos can be set after this is called.
    pub fn query(self, query: String) -> RandomQuery { RandomQuery { rand: self, query } }

    /// Restrict the photos to only photos which are within the given
    /// collections. NOTE: only the number of photos can be set this is
    /// called.
    pub fn collection<I>(self, collection: I) -> RandomCollection
    where
        I: IntoIterator<Item = String>,
    {
        RandomCollection { rand: self, collection: collection.into_iter().join(",") }
    }

    /// Specify the the number of photos to get. NOTE: nothing can be set after
    /// this is called.
    pub fn count(self, count: usize) -> RandomCount {
        assert_ne!(count, 0, "Cannot get 0 images!");
        RandomCount { rand: self, count }
    }

    /// Get the a random photo.
    ///
    /// # Errors
    /// - Request wrapping a Hyper error is raised if there is an error
    /// handling the HTTP Stream.
    /// - MalformedResponse
    ///     - wrapping a JSON error is raised if the JSON returned from
    /// Unsplash is invalid.
    pub fn get<C>(
        self,
        client: &Client<C>,
        access_key: &str,
    ) -> impl Future<Item = Photo, Error = Error>
    where
        C: Connect + 'static,
    {
        let serial = RandomSerialize {
            featured: self.featured,
            username: self.username,
            w: self.w,
            h: self.h,
            orientation: self.orientation,
            collection: None,
            query: None,
        };
        ::endpoint::get(serial, client, access_key, RANDOM_URI.clone())
    }
}

impl RandomQuery {
    /// Specify the the number of photos to get. NOTE: nothing can be set after
    /// this is called.
    pub fn count(self, count: usize) -> RandomQueryCount {
        assert_ne!(count, 0, "Cannot get 0 images!");
        RandomQueryCount { rand: self, count }
    }

    /// Get the a random photo which matches the query.
    ///
    /// # Errors
    /// - Request wrapping a Hyper error is raised if there is an error
    /// handling the HTTP Stream.
    /// - MalformedResponse
    ///     - wrapping a JSON error is raised if the JSON returned from
    /// Unsplash is invalid.
    pub fn get<C>(
        self,
        client: &Client<C>,
        access_key: &str,
    ) -> impl Future<Item = Photo, Error = Error>
    where
        C: Connect + 'static,
    {
        let serial = RandomSerialize {
            featured: self.rand.featured,
            username: self.rand.username,
            w: self.rand.w,
            h: self.rand.h,
            orientation: self.rand.orientation,
            collection: None,
            query: Some(self.query),
        };
        ::endpoint::get(
            serial,
            &client,
            format!("Client-ID: {}", access_key).as_ref(),
            RANDOM_URI.clone(),
        )
    }
}

impl RandomCollection {
    /// Specify the the number of photos to get. NOTE: nothing can be set after
    /// this is called.
    pub fn count(self, count: usize) -> RandomCollectionCount {
        assert_ne!(count, 0, "Cannot get 0 images!");
        RandomCollectionCount { rand: self, count }
    }

    /// Get the a random photo which is in the collections.
    ///
    /// # Errors
    /// - Request wrapping a Hyper error is raised if there is an error
    /// handling the HTTP Stream.
    /// - MalformedResponse
    ///     - wrapping a JSON error is raised if the JSON returned from
    /// Unsplash is invalid.
    pub fn get<C>(
        self,
        client: &Client<C>,
        access_key: &str,
    ) -> impl Future<Item = Photo, Error = Error>
    where
        C: Connect + 'static,
    {
        let serial = RandomSerialize {
            featured: self.rand.featured,
            username: self.rand.username,
            w: self.rand.w,
            h: self.rand.h,
            orientation: self.rand.orientation,
            collection: Some(self.collection),
            query: None,
        };
        ::endpoint::get(
            serial,
            client,
            format!("Client-ID: {}", access_key).as_ref(),
            RANDOM_URI.clone(),
        )
    }
}

impl RandomCount {
    /// Get the random photos.
    ///
    /// # Errors
    /// - Request wrapping a Hyper error is raised if there is an error
    /// handling the HTTP Stream.
    /// - MalformedResponse
    ///     - wrapping a JSON error is raised if the JSON returned from
    /// Unsplash is invalid.
    pub fn get<C>(
        self,
        client: &Client<C>,
        access_key: &str,
    ) -> impl Future<Item = Vec<Photo>, Error = Error>
    where
        C: Connect + 'static,
    {
        let serial = RandomCountSerialize {
            featured: self.rand.featured,
            username: self.rand.username,
            w: self.rand.w,
            h: self.rand.h,
            orientation: self.rand.orientation,
            collection: None,
            query: None,
            count: self.count,
        };
        ::endpoint::get(
            serial,
            client,
            format!("Client-ID: {}", access_key).as_ref(),
            RANDOM_URI.clone(),
        )
    }
}

impl RandomQueryCount {
    /// Get the random photos which matches the query.
    ///
    /// # Errors
    /// - Request wrapping a Hyper error is raised if there is an error
    /// handling the HTTP Stream.
    /// - MalformedResponse
    ///     - wrapping a JSON error is raised if the JSON returned from
    /// Unsplash is invalid.
    pub fn get<C>(
        self,
        client: &Client<C>,
        access_key: &str,
    ) -> impl Future<Item = Vec<Photo>, Error = Error>
    where
        C: Connect + 'static,
    {
        let serial = RandomCountSerialize {
            featured: self.rand.rand.featured,
            username: self.rand.rand.username,
            w: self.rand.rand.w,
            h: self.rand.rand.h,
            orientation: self.rand.rand.orientation,
            collection: None,
            query: Some(self.rand.query),
            count: self.count,
        };
        ::endpoint::get(
            serial,
            client,
            format!("Client-ID: {}", access_key).as_ref(),
            RANDOM_URI.clone(),
        )
    }
}

impl RandomCollectionCount {
    /// Get the random photos which are in the collections.
    ///
    /// # Errors
    /// - Request wrapping a Hyper error is raised if there is an error
    /// handling the HTTP Stream.
    /// - MalformedResponse
    ///     - wrapping a JSON error is raised if the JSON returned from
    /// Unsplash is invalid.
    pub fn get<C>(
        self,
        client: &Client<C>,
        access_key: &str,
    ) -> impl Future<Item = Vec<Photo>, Error = Error>
    where
        C: Connect + 'static,
    {
        let serial = RandomCountSerialize {
            featured: self.rand.rand.featured,
            username: self.rand.rand.username,
            w: self.rand.rand.w,
            h: self.rand.rand.h,
            orientation: self.rand.rand.orientation,
            collection: Some(self.rand.collection),
            query: None,
            count: self.count,
        };
        ::endpoint::get(
            serial,
            client,
            format!("Client-ID: {}", access_key).as_ref(),
            RANDOM_URI.clone(),
        )
    }
}
