use itertools::Itertools;
use maplit::hashmap;
use std::collections::HashMap;

#[tokio::main]
async fn main() {
    let content = utilities::get_example(5).await;
    let mut s: HashMap<i64, Vec<i64>> = hashmap! {};
    let mut p: HashMap<i64, Vec<i64>> = hashmap! {};
    let mut updates = vec![];
    let mut updates_started = false;
    for line in content.lines() {
        if line.is_empty() {
            updates_started = true;
        } else if !updates_started {
            let mut c_iter = line.split('|');
            let first: i64 = c_iter.next().unwrap().parse().unwrap();
            let second: i64 = c_iter.next().unwrap().parse().unwrap();
            s.entry(first).or_default().push(second);
            p.entry(second).or_default().push(first);
        } else {
            let mut l = vec![];
            line.split(',').for_each(|x| {
                let x: i64 = x.parse().unwrap();
                s.entry(x).or_default();
                p.entry(x).or_default();
                l.push(x);
            });
            updates.push(l);
        }
    }

    // topo sort
    let mut sorted = vec![];
    let mut rem = p.clone();
    loop {
        if rem.is_empty() {
            break;
        }
        let next = *rem.iter().find(|(s, ps)| ps.is_empty()).unwrap().0;
        rem.remove(&next);
        rem.values_mut()
            .for_each(|ps| ps.retain(|elem| elem != &next));
        sorted.push(next);
    }

    let mut result = 0i64;
    for mut update in updates {
        let is_valid = update
            .iter()
            .tuple_combinations()
            .all(|(a, b)| !p[a].contains(b));
        if is_valid {
            // let mid = update[update.len() / 2];
            // result += mid;
        } else {
            update.sort_by_key(|a| sorted.iter().position(|x| x == a).unwrap());
            println!("{:?}", update);
            let mid = update[update.len() / 2];
            result += mid;
        }
    }

    println!("Solution: {}", result);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_something() {}
}
