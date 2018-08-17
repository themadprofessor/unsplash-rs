//! Photos endpoint
//!
//! Access to the endpoint is through the [Photo](struct.Photos.html) struct.

use chrono::{DateTime, FixedOffset};
use endpoint::me::User;
use futures::Future;
use hyper::{client::connect::Connect, Client};

use std::fmt;

mod list;
mod random;

use error::*;

pub use self::{list::List, random::Random};

/// Access type to Unsplash's Photos endpoint.
#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub struct Photos;

/// A type for a url returned from a photo's download endpoint.
#[derive(Debug, Serialize, Deserialize)]
pub struct Url {
    url: String,
}

/// A Photo from Unsplash.
#[derive(Debug, Serialize, Deserialize)]
pub struct Photo {
    /// Photo ID.
    pub id: String,
    /// Photo creation date.
    pub created_at: DateTime<FixedOffset>,
    /// Last time photo was updated.
    pub updated_at: DateTime<FixedOffset>,
    /// Width of photo
    pub width: usize,
    /// Height of photo
    pub height: usize,
    /// Photo color
    pub color: String,
    /// Number of likes the photo has
    pub likes: usize,
    /// Has the photo been liked by the current user (false if not logged in).
    pub liked_by_user: bool,
    /// Description of the photo.
    pub description: Option<String>,
    /// User who posted the photo.
    pub user: User,
    /// Collections the photo is in.
    pub current_user_collections: Vec<Collection>,
    /// Urls to the photo in various sizes.
    pub urls: Urls,
    /// Links to the photo.
    pub links: PhotoLinks,
}

/// A collection of photos on Unsplsash
#[derive(Debug, Serialize, Deserialize)]
pub struct Collection {
    /// Collection ID
    pub id: usize,
    /// Collection's title
    pub title: String,
    /// Date when collection was published.
    pub published_at: DateTime<FixedOffset>,
    /// Last date when the collection was updated.
    pub updated_at: DateTime<FixedOffset>,
    /// Is the collection curated.
    pub curated: bool,
}

/// Urls of a photo in various sizes.
#[derive(Debug, Serialize, Deserialize)]
pub struct Urls {
    /// URL to the raw photo.
    pub raw: String,
    /// URL to the full size photo.
    pub full: String,
    /// URL to the regular size photo.
    pub regular: String,
    /// URL to the small size photo.
    pub small: String,
    /// URL to the thumbnail size photo.
    pub thumb: String,
}

/// Links to a photo.
#[derive(Debug, Serialize, Deserialize)]
pub struct PhotoLinks {
    /// API link to the photo.
    #[serde(rename = "self")]
    pub self_link: String,
    /// Link to the photo.
    pub html: String,
    /// Link to the photo download.
    pub download: String,
    /// API link to the photo download.
    pub download_location: String,
}

/// Ordering of results from Unsplash
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum Order {
    /// Latest comes first.
    /// Default if unspecified.
    Latest,
    /// Oldest comes first.
    Oldest,
    /// Most popular comes first.
    Popular,
}

/// Orientation of a photo
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum Orientation {
    /// Portrait orientation.
    Portrait,
    /// Landscape orientation.
    Landscape,
    /// Squarish shape.
    Squarish,
}

impl Photo {
    /// Gets the download URL for this photo.
    ///
    /// Unsplash requires user of its API to download photos from the URL
    /// returned by the photo's download endpoint (/photo/<id>/download).
    /// The URL of the download endpoint is
    /// accessable from a Photo object (photo.links.download_location).
    pub fn get_download_url<C>(
        &self,
        client: &Client<C>,
        access_key: &str,
    ) -> impl Future<Item = Url, Error = Error>
    where
        C: Connect + 'static,
    {
        ::endpoint::get(
            (),
            &client,
            format!("Client-ID: {}", access_key).as_ref(),
            self.links.download_location.parse().unwrap(),
        )
    }
}

impl fmt::Display for Url {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> { f.write_str(&self.url) }
}

impl AsRef<str> for Url {
    fn as_ref(&self) -> &str { self.url.as_ref() }
}

impl Default for Order {
    fn default() -> Self { Order::Latest }
}

impl Photos {
    /// Get a list of photos from Unsplash
    pub fn list() -> List { List::default() }

    /// Get a random photo/some random photos from Unsplash
    pub fn random() -> Random { Random::default() }
}
