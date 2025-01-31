// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use bee_block::{output::OutputId, rand::output::rand_output_id};
use bee_ledger::types::CreatedOutput;
use bee_ledger_types::rand::output::rand_created_output;
use bee_storage::{
    access::{AsIterator, Batch, BatchBuilder, Delete, Exist, Fetch, Insert, MultiFetch, Truncate},
    backend,
};

pub trait StorageBackend:
    backend::StorageBackend
    + Exist<OutputId, CreatedOutput>
    + Fetch<OutputId, CreatedOutput>
    + for<'a> MultiFetch<'a, OutputId, CreatedOutput>
    + Insert<OutputId, CreatedOutput>
    + Delete<OutputId, CreatedOutput>
    + BatchBuilder
    + Batch<OutputId, CreatedOutput>
    + for<'a> AsIterator<'a, OutputId, CreatedOutput>
    + Truncate<OutputId, CreatedOutput>
{
}

impl<T> StorageBackend for T where
    T: backend::StorageBackend
        + Exist<OutputId, CreatedOutput>
        + Fetch<OutputId, CreatedOutput>
        + for<'a> MultiFetch<'a, OutputId, CreatedOutput>
        + Insert<OutputId, CreatedOutput>
        + Delete<OutputId, CreatedOutput>
        + BatchBuilder
        + Batch<OutputId, CreatedOutput>
        + for<'a> AsIterator<'a, OutputId, CreatedOutput>
        + Truncate<OutputId, CreatedOutput>
{
}

pub fn output_id_to_created_output_access<B: StorageBackend>(storage: &B) {
    let (output_id, created_output) = (rand_output_id(), rand_created_output());

    assert!(!Exist::<OutputId, CreatedOutput>::exist(storage, &output_id).unwrap());
    assert!(
        Fetch::<OutputId, CreatedOutput>::fetch(storage, &output_id)
            .unwrap()
            .is_none()
    );
    let results = MultiFetch::<OutputId, CreatedOutput>::multi_fetch(storage, &[output_id])
        .unwrap()
        .collect::<Vec<_>>();
    assert_eq!(results.len(), 1);
    assert!(matches!(results.get(0), Some(Ok(None))));

    Insert::<OutputId, CreatedOutput>::insert(storage, &output_id, &created_output).unwrap();

    assert!(Exist::<OutputId, CreatedOutput>::exist(storage, &output_id).unwrap());
    assert_eq!(
        Fetch::<OutputId, CreatedOutput>::fetch(storage, &output_id)
            .unwrap()
            .unwrap(),
        created_output
    );
    let results = MultiFetch::<OutputId, CreatedOutput>::multi_fetch(storage, &[output_id])
        .unwrap()
        .collect::<Vec<_>>();
    assert_eq!(results.len(), 1);
    assert!(matches!(results.get(0), Some(Ok(Some(v))) if v == &created_output));

    Delete::<OutputId, CreatedOutput>::delete(storage, &output_id).unwrap();

    assert!(!Exist::<OutputId, CreatedOutput>::exist(storage, &output_id).unwrap());
    assert!(
        Fetch::<OutputId, CreatedOutput>::fetch(storage, &output_id)
            .unwrap()
            .is_none()
    );
    let results = MultiFetch::<OutputId, CreatedOutput>::multi_fetch(storage, &[output_id])
        .unwrap()
        .collect::<Vec<_>>();
    assert_eq!(results.len(), 1);
    assert!(matches!(results.get(0), Some(Ok(None))));

    let mut batch = B::batch_begin();
    let mut output_ids = Vec::new();
    let mut created_outputs = Vec::new();

    for _ in 0..10 {
        let (output_id, created_output) = (rand_output_id(), rand_created_output());
        Insert::<OutputId, CreatedOutput>::insert(storage, &output_id, &created_output).unwrap();
        Batch::<OutputId, CreatedOutput>::batch_delete(storage, &mut batch, &output_id).unwrap();
        output_ids.push(output_id);
        created_outputs.push((output_id, None));
    }

    for _ in 0..10 {
        let (output_id, created_output) = (rand_output_id(), rand_created_output());
        Batch::<OutputId, CreatedOutput>::batch_insert(storage, &mut batch, &output_id, &created_output).unwrap();
        output_ids.push(output_id);
        created_outputs.push((output_id, Some(created_output)));
    }

    storage.batch_commit(batch, true).unwrap();

    let iter = AsIterator::<OutputId, CreatedOutput>::iter(storage).unwrap();
    let mut count = 0;

    for result in iter {
        let (output_id, created_output) = result.unwrap();
        assert!(created_outputs.contains(&(output_id, Some(created_output))));
        count += 1;
    }

    assert_eq!(count, 10);

    let results = MultiFetch::<OutputId, CreatedOutput>::multi_fetch(storage, &output_ids)
        .unwrap()
        .collect::<Vec<_>>();

    assert_eq!(results.len(), output_ids.len());

    for ((_, created_output), result) in created_outputs.into_iter().zip(results.into_iter()) {
        assert_eq!(created_output, result.unwrap());
    }

    Truncate::<OutputId, CreatedOutput>::truncate(storage).unwrap();

    let mut iter = AsIterator::<OutputId, CreatedOutput>::iter(storage).unwrap();

    assert!(iter.next().is_none());
}
