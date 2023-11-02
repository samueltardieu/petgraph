use crate::{base::MaybeOwned, GraphStorage};

// TODO: Entry API

pub trait AttributeStorage<K, V> {
    fn get(&self, id: &K) -> Option<&V>;
    fn get_mut(&mut self, id: &K) -> Option<&mut V>;
    fn index(&self, id: &K) -> &V {
        self.get(id).unwrap()
    }
    fn index_mut(&mut self, id: &K) -> &mut V {
        self.get_mut(id).unwrap()
    }

    fn set(&mut self, id: &K, value: V) -> Option<V>;
    fn remove(&mut self, id: &K) -> Option<V>;

    type Iter<'a>: Iterator<Item = (MaybeOwned<'a, K>, &'a V)>
    where
        K: 'a,
        V: 'a,
        Self: 'a;

    fn iter(&self) -> Self::Iter<'_>;
}

pub trait AttributeGraphId<S>: Sized
where
    S: GraphStorage,
{
    type Store<'a, V>: AttributeStorage<Self, V>
    where
        S: 'a;

    fn attribute_store<V>(storage: &S) -> Self::Store<'_, V>;
}
