use crate::structure::{Edge, GValue, Vertex};
use crate::Token;
use crate::GremlinError;
use std::collections::{BTreeMap, HashMap};
use std::convert::TryInto;

/// Represent a Map<[GKey](struct.GKey),[GValue](struct.GValue)> which has ability to allow for non-String keys.
/// TinkerPop type [here](http://tinkerpop.apache.org/docs/current/dev/io/#_map)
#[derive(Debug, PartialEq, Clone)]
pub struct Map(HashMap<GKey, GValue>);

//This would be used to take a given Struct which has return types of type(s) GValue and Keys of type GKey and convert it into type T 
//This in most cases would not be the final stage of conversion - this will just convert to a basic Struct - from there you might want to do your individual type parsing on types such as chrono, uuid etc
pub trait TryFromGremlinMap<T> : Default {
    fn try_from_gremlin_map(hm: HashMap<String, GValue>) -> Result<T, GremlinError>;
}

impl Map {
    pub(crate) fn empty() -> Map {
        Map(HashMap::default())
    }
}

impl From<HashMap<GKey, GValue>> for Map {
    fn from(val: HashMap<GKey, GValue>) -> Self {
        Map(val)
    }
}

impl From<HashMap<String, GValue>> for Map {
    fn from(val: HashMap<String, GValue>) -> Self {
        let map = val.into_iter().map(|(k, v)| (GKey::String(k), v)).collect();
        Map(map)
    }
}

impl TryInto<HashMap<String, GValue>> for Map {
    type Error = GremlinError;

    fn try_into(self) -> Result<HashMap<String, GValue>, Self::Error> {
        let mut hashmap = HashMap::new();
        for val in self.into_iter() {
            hashmap.insert(val.0.try_into()?, val.1);
        };
        Ok(hashmap)
    }
}

impl TryInto<String> for GKey {
    type Error = GremlinError;

    fn try_into(self) -> Result<String, Self::Error> {
        match self {
            GKey::String(s) => Ok(s),
            _  => Err(GremlinError::MapError("GKey is not of type string".to_string()))
        }
    }
}

impl From<BTreeMap<String, GValue>> for Map {
    fn from(val: BTreeMap<String, GValue>) -> Self {
        let map = val.into_iter().map(|(k, v)| (GKey::String(k), v)).collect();
        Map(map)
    }
}

impl Map {
    pub(crate) fn remove<T>(&mut self, key: T) -> Option<GValue>
    where
        T: Into<GKey>,
    {
        self.0.remove(&key.into())
    }
    /// Iterate all key-value pairs
    pub fn iter(&self) -> impl Iterator<Item = (&GKey, &GValue)> {
        self.0.iter()
    }

    pub fn into_iter(self) -> impl Iterator<Item = (GKey, GValue)> {
        self.0.into_iter()
    }

    ///Returns a reference to the value corresponding to the key.
    pub fn get<T>(&self, key: T) -> Option<&GValue>
    where
        T: Into<GKey>,
    {
        self.0.get(&key.into())
    }

    /// Returns the number of elements in the map.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns true if the map contains no elements.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl<T: Into<GKey>> std::ops::Index<T> for Map {
    type Output = GValue;

    fn index(&self, key: T) -> &GValue {
        self.0.get(&key.into()).expect("no entry found for key")
    }
}
impl std::iter::FromIterator<(String, GValue)> for Map {
    fn from_iter<I: IntoIterator<Item = (String, GValue)>>(iter: I) -> Self {
        Map(iter
            .into_iter()
            .map(|(k, v)| (GKey::String(k), v))
            .collect())
    }
}

/// Possible key types in a [Map](struct.Map)
#[allow(clippy::large_enum_variant)]
#[derive(Debug, PartialEq, Clone, Eq, Hash)]
pub enum GKey {
    String(String),
    Token(Token),
    Vertex(Vertex),
    Edge(Edge),
}

impl From<&str> for GKey {
    fn from(val: &str) -> Self {
        GKey::String(String::from(val))
    }
}

impl From<String> for GKey {
    fn from(val: String) -> Self {
        GKey::String(val)
    }
}

impl From<&Vertex> for GKey {
    fn from(val: &Vertex) -> Self {
        GKey::Vertex(val.clone())
    }
}

impl From<&Edge> for GKey {
    fn from(val: &Edge) -> Self {
        GKey::Edge(val.clone())
    }
}
