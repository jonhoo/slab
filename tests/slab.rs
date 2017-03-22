extern crate slab;

use slab::*;

#[test]
fn store_get_remove_one() {
    let mut slab = Slab::new();
    assert!(slab.is_empty());

    let key = slab.store(10);

    assert_eq!(slab[key], 10);
    assert_eq!(slab.get(key), Some(&10));
    assert!(!slab.is_empty());
    assert!(slab.contains(key));

    assert_eq!(slab.remove(key), 10);
    assert!(!slab.contains(key));
    assert!(slab.get(key).is_none());
}

#[test]
fn store_get_many() {
    let mut slab = Slab::with_capacity(10);

    for i in 0..10 {
        let key = slab.store(i + 10);
        assert_eq!(slab[key], i + 10);
    }

    assert_eq!(slab.capacity(), 10);

    // Storing another one grows the slab
    let key = slab.store(20);
    assert_eq!(slab[key], 20);

    // Capacity grows by 2x
    assert_eq!(slab.capacity(), 20);
}

#[test]
fn store_get_remove_many() {
    let mut slab = Slab::with_capacity(10);
    let mut keys = vec![];

    for i in 0..10 {
        for j in 0..10 {
            let val = (i * 10) + j;

            let key = slab.store(val);
            keys.push((key, val));
            assert_eq!(slab[key], val);
        }

        for (key, val) in keys.drain(..) {
            assert_eq!(val, slab.remove(key));
        }
    }

    assert_eq!(10, slab.capacity());
}

#[test]
fn store_with_slot() {
    let mut slab = Slab::with_capacity(1);
    let key;

    {
        let slot = slab.slot();
        key = slot.key();
        slot.store(123);
    }

    assert_eq!(123, slab[key]);
}

#[test]
fn get_slot_without_using() {
    let mut slab = Slab::<usize>::with_capacity(1);
    let key = slab.slot().key();
    assert_eq!(key, slab.slot().key());
}

#[test]
#[should_panic]
fn invalid_get_panics() {
    let slab = Slab::<usize>::with_capacity(1);
    slab[0];
}

#[test]
#[should_panic]
fn double_remove_panics() {
    let mut slab = Slab::<usize>::with_capacity(1);
    let key = slab.store(123);
    slab.remove(key);
    slab.remove(key);
}

#[test]
#[should_panic]
fn invalid_remove_panics() {
    let mut slab = Slab::<usize>::with_capacity(1);
    slab.remove(0);
}

#[test]
fn slab_get_mut() {
    let mut slab = Slab::new();
    let key = slab.store(1);

    slab[key] = 2;
    assert_eq!(slab[key], 2);

    *slab.get_mut(key).unwrap() = 3;
    assert_eq!(slab[key], 3);
}

#[test]
fn reserve_does_not_allocate_if_available() {
    let mut slab = Slab::with_capacity(10);
    let mut keys = vec![];

    for i in 0..10 {
        keys.push(slab.store(i));
    }

    for key in keys {
        slab.remove(key);
    }

    slab.reserve(10);
    assert_eq!(10, slab.capacity());
}

#[test]
fn reserve_exact_does_not_allocate_if_available() {
    let mut slab = Slab::with_capacity(10);
    let mut keys = vec![];

    for i in 0..10 {
        keys.push(slab.store(i));
    }

    for key in keys {
        slab.remove(key);
    }

    slab.reserve_exact(10);
    assert_eq!(10, slab.capacity());
}

#[test]
fn retain() {
    let mut slab = Slab::with_capacity(2);

    let key1 = slab.store(0);
    let key2 = slab.store(1);

    slab.retain(|x| *x % 2 == 0);

    assert_eq!(slab.len(), 1);
    assert_eq!(slab[key1], 0);
    assert!(!slab.contains(key2));

    // Ensure consistency is retained
    let key = slab.store(123);
    assert_eq!(key, key2);

    assert_eq!(2, slab.len());
    assert_eq!(2, slab.capacity());

    // Inserting another element grows
    let key = slab.store(345);
    assert_eq!(key, 2);

    assert_eq!(4, slab.capacity());
}

#[test]
fn iter() {
    let mut slab = Slab::new();

    for i in 0..4 {
        slab.store(i);
    }

    let vals: Vec<_> = slab.iter().map(|r| *r).collect();
    assert_eq!(vals, vec![0, 1, 2, 3]);

    slab.remove(1);

    let vals: Vec<_> = slab.iter().map(|r| *r).collect();
    assert_eq!(vals, vec![0, 2, 3]);
}

#[test]
fn iter_mut() {
    let mut slab = Slab::new();

    for i in 0..4 {
        slab.store(i);
    }

    for e in slab.iter_mut() {
        *e = *e + 1;
    }

    let vals: Vec<_> = slab.iter().map(|r| *r).collect();
    assert_eq!(vals, vec![1, 2, 3, 4]);

    slab.remove(2);

    for e in slab.iter_mut() {
        *e = *e + 1;
    }

    let vals: Vec<_> = slab.iter().map(|r| *r).collect();
    assert_eq!(vals, vec![2, 3, 5]);
}

#[test]
fn clear() {
    let mut slab = Slab::new();

    for i in 0..4 {
        slab.store(i);
    }

    // clear full
    slab.clear();

    let vals: Vec<_> = slab.iter().map(|r| *r).collect();
    assert!(vals.is_empty());

    assert_eq!(0, slab.len());
    assert_eq!(4, slab.capacity());

    for i in 0..2 {
        slab.store(i);
    }

    let vals: Vec<_> = slab.iter().map(|r| *r).collect();
    assert_eq!(vals, vec![0, 1]);

    // clear half-filled
    slab.clear();

    let vals: Vec<_> = slab.iter().map(|r| *r).collect();
    assert!(vals.is_empty());
}