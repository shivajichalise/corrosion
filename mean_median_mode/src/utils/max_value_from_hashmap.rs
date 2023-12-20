use std::collections::HashMap;

pub fn max_value_from_hashmap<K, V>(map: &HashMap<K, V>) -> Option<&V>
where
    V: Ord,
{
    map.values().max()
}
