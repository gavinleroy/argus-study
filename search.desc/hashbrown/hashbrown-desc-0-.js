searchState.loadedDescShard("hashbrown", 0, "This crate is a Rust port of Google’s high-performance …\nThe memory allocator returned an error\nError due to the computed capacity exceeding the collection…\nKey equivalence trait.\nA hash map implemented with quadratic probing and SIMD …\nA hash set implemented as a <code>HashMap</code> where the value is <code>()</code>.\nLow-level hash table with explicit hashing.\nThe error type for <code>try_reserve</code> methods.\nChecks if this value is equivalent to the given key.\nReturns the argument unchanged.\nA hash map implemented with quadratic probing and SIMD …\nA hash set implemented as a <code>HashMap</code> where the value is <code>()</code>.\nA hash table implemented with quadratic probing and SIMD …\nCalls <code>U::from(self)</code>.\nExperimental and unsafe <code>RawTable</code> API. This module is only …\nThe layout of the allocation request that failed.\nDefault hasher for <code>HashMap</code>.\nA draining iterator over the entries of a <code>HashMap</code> in …\nA view into a single entry in a map, which may either be …\nA view into a single entry in a map, which may either be …\nA draining iterator over entries of a <code>HashMap</code> which don’…\nA hash map implemented with quadratic probing and SIMD …\nAn owning iterator over the entries of a <code>HashMap</code> in …\nAn owning iterator over the keys of a <code>HashMap</code> in arbitrary …\nAn owning iterator over the values of a <code>HashMap</code> in …\nAn iterator over the entries of a <code>HashMap</code> in arbitrary …\nA mutable iterator over the entries of a <code>HashMap</code> in …\nAn iterator over the keys of a <code>HashMap</code> in arbitrary order. …\nAn occupied entry.\nAn occupied entry.\nAn occupied entry.\nA view into an occupied entry in a <code>HashMap</code>. It is part of …\nA view into an occupied entry in a <code>HashMap</code>. It is part of …\nThe error returned by <code>try_insert</code> when the key already …\nA builder for computing where in a <code>HashMap</code> a key-value …\nA builder for computing where in a <code>HashMap</code> a key-value …\nA view into a single entry in a map, which may either be …\nA view into an occupied entry in a <code>HashMap</code>. It is part of …\nA view into a vacant entry in a <code>HashMap</code>. It is part of the …\nA vacant entry.\nA vacant entry.\nA vacant entry.\nA view into a vacant entry in a <code>HashMap</code>. It is part of the …\nA view into a vacant entry in a <code>HashMap</code>. It is part of the …\nAn iterator over the values of a <code>HashMap</code> in arbitrary …\nA mutable iterator over the values of a <code>HashMap</code> in …\nReturns a reference to the underlying allocator.\nProvides in-place mutable access to an occupied entry …\nProvides in-place mutable access to an occupied entry …\nProvides in-place mutable access to an occupied entry …\nProvides shared access to the key and owned access to the …\nProvides shared access to the key and owned access to the …\nProvides shared access to the key and owned access to the …\nReturns the number of elements the map can hold without …\nClears the map, removing all key-value pairs. Keeps the …\nReturns <code>true</code> if the map contains a value for the specified …\nCreates an empty <code>HashMap&lt;K, V, S, A&gt;</code>, with the <code>Default</code> …\nClears the map, returning all key-value pairs as an …\nGets the given key’s corresponding entry in the map for …\nThe entry in the map that was already occupied.\nGets the given key’s corresponding entry by reference in …\nInserts all new key-values from the iterator to existing …\nInserts all new key-values from the iterator to existing …\nInserts all new key-values from the iterator to existing …\nDrains elements which are true under the given predicate, …\nReturns the argument unchanged.\nExamples\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCreates a <code>RawEntryMut</code> from the given hash and matching …\nAccess an immutable entry by hash and matching function.\nCreates a <code>RawEntryMut</code> from the given key.\nAccess an immutable entry by key.\nCreates a <code>RawEntryMut</code> from the given key and its hash.\nAccess an immutable entry by a key and its hash.\nReturns a reference to the value corresponding to the key.\nGets a reference to the value in the entry.\nGets a reference to the value in the entry.\nGets a reference to the value in the entry.\nReturns the key-value pair corresponding to the supplied …\nGets a reference to the key and value in the entry.\nReturns the key-value pair corresponding to the supplied …\nGets a mutable reference to the key and value in the entry.\nAttempts to get mutable references to <code>N</code> values in the map …\nAttempts to get mutable references to <code>N</code> values in the map …\nAttempts to get mutable references to <code>N</code> values in the map …\nAttempts to get mutable references to <code>N</code> values in the map …\nReturns a mutable reference to the value corresponding to …\nGets a mutable reference to the value in the entry.\nGets a mutable reference to the value in the entry.\nGets a mutable reference to the value in the entry.\nReturns a reference to the map’s <code>BuildHasher</code>.\nReturns a reference to the value corresponding to the …\nInserts a key-value pair into the map.\nSets the value of the entry, and returns a …\nSets the value of the entry, and returns the entry’s old …\nSets the value of the entry with the VacantEntry’s key, …\nSets the value of the entry, and returns an OccupiedEntry.\nSets the value of the entry, and returns the entry’s old …\nSets the value of the entry with the VacantEntry’s key, …\nSets the value of the entry, and returns an …\nSets the value of the entry, and returns the entry’s old …\nSets the value of the entry with the VacantEntryRef’s …\nSets the value of the entry with the VacantEntry’s key, …\nSets the value of the entry, and returns the entry’s old …\nInsert a key-value pair into the map without checking if …\nSet the value of an entry with a custom hasher function.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCreates an iterator over the entries of a <code>HashMap</code> in …\nCreates a consuming iterator, that is, one that moves each …\nCreates an iterator over the entries of a <code>HashMap</code> in …\nConverts the entry into a mutable reference to the key in …\nTake ownership of the key.\nTake ownership of the key.\nConverts the OccupiedEntry into a mutable reference to the …\nCreates a consuming iterator visiting all the keys in …\nConverts the OccupiedEntry into a mutable reference to the …\nConverts the OccupiedEntry into a mutable reference to the …\nConverts the OccupiedEntryRef into a mutable reference to …\nCreates a consuming iterator visiting all the values in …\nReturns <code>true</code> if the map contains no elements.\nAn iterator visiting all key-value pairs in arbitrary …\nAn iterator visiting all key-value pairs in arbitrary …\nGets a reference to the key in the entry.\nReturns a reference to this entry’s key.\nGets a reference to the key in the entry.\nGets a reference to the key that would be used when …\nReturns a reference to this entry’s key.\nGets a reference to the key in the entry.\nGets a reference to the key that would be used when …\nGets a mutable reference to the key in the entry.\nAn iterator visiting all keys in arbitrary order. The …\nReturns the number of elements in the map.\nCreates an empty <code>HashMap</code>.\nCreates an empty <code>HashMap</code> using the given allocator.\nEnsures a value is in the entry by inserting the default …\nEnsures a value is in the entry by inserting the default …\nEnsures a value is in the entry by inserting the default …\nEnsures a value is in the entry by inserting the default …\nEnsures a value is in the entry by inserting the default …\nEnsures a value is in the entry by inserting the result of …\nEnsures a value is in the entry by inserting the result of …\nEnsures a value is in the entry by inserting the result of …\nEnsures a value is in the entry by inserting, if empty, …\nEnsures a value is in the entry by inserting, if empty, …\nCreates a raw immutable entry builder for the HashMap.\nCreates a raw entry builder for the HashMap.\nReturns a reference to the <code>RawTable</code> used underneath <code>HashMap</code>…\nReturns a mutable reference to the <code>RawTable</code> used …\nRemoves a key from the map, returning the value at the key …\nTakes the value out of the entry, and returns it.\nTakes the value out of the entry, and returns it. Keeps …\nTakes the value out of the entry, and returns it. Keeps …\nRemoves a key from the map, returning the stored key and …\nTake the ownership of the key and value from the map.\nTake the ownership of the key and value from the map. …\nTake the ownership of the key and value from the map. …\nReplaces the entry, returning the old key and value. The …\nReplaces the entry, returning the old key and value. The …\nProvides shared access to the key and owned access to the …\nProvides shared access to the key and owned access to the …\nProvides shared access to the key and owned access to the …\nReplaces the key in the hash map with the key used to …\nReplaces the key in the hash map with the key used to …\nReserves capacity for at least <code>additional</code> more elements to …\nRetains only the elements specified by the predicate. …\nShrinks the capacity of the map with a lower limit. It …\nShrinks the capacity of the map as much as possible. It …\nTries to insert a key-value pair into the map, and returns …\nTries to reserve capacity for at least <code>additional</code> more …\nThe value which was not inserted, because the entry was …\nAn iterator visiting all values in arbitrary order. The …\nAn iterator visiting all values mutably in arbitrary order.\nCreates an empty <code>HashMap</code> with the specified capacity.\nCreates an empty <code>HashMap</code> with the specified capacity, …\nCreates an empty <code>HashMap</code> with the specified capacity, …\nCreates an empty <code>HashMap</code> with the specified capacity using …\nCreates an empty <code>HashMap</code> which will use the given hash …\nCreates an empty <code>HashMap</code> which will use the given hash …\nA lazy iterator producing elements in the difference of …\nA draining iterator over the items of a <code>HashSet</code>.\nA view into a single entry in a set, which may either be …\nA draining iterator over entries of a <code>HashSet</code> which don’…\nA hash set implemented as a <code>HashMap</code> where the value is <code>()</code>.\nA lazy iterator producing elements in the intersection of …\nAn owning iterator over the items of a <code>HashSet</code>.\nAn iterator over the items of a <code>HashSet</code>.\nAn occupied entry.\nA view into an occupied entry in a <code>HashSet</code>. It is part of …\nA lazy iterator producing elements in the symmetric …\nA lazy iterator producing elements in the union of <code>HashSet</code>…\nA vacant entry.\nA view into a vacant entry in a <code>HashSet</code>. It is part of the …\nReturns a reference to the underlying allocator.\nReturns the intersection of <code>self</code> and <code>rhs</code> as a new …\nReturns the union of <code>self</code> and <code>rhs</code> as a new <code>HashSet&lt;T, S&gt;</code>.\nReturns the symmetric difference of <code>self</code> and <code>rhs</code> as a new …\nReturns the number of elements the set can hold without …\nClears the set, removing all values.\nReturns <code>true</code> if the set contains a value.\nCreates an empty <code>HashSet&lt;T, S&gt;</code> with the <code>Default</code> value for …\nVisits the values representing the difference, i.e., the …\nClears the set, returning all elements in an iterator.\nGets the given value’s corresponding entry in the set …\nDrains elements which are true under the given predicate, …\nReturns the argument unchanged.\nExamples\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns a reference to the value in the set, if any, that …\nReturns a reference to this entry’s value.\nGets a reference to the value in the entry.\nGets a reference to the value that would be used when …\nInserts the given <code>value</code> into the set if it is not present, …\nInserts an owned copy of the given <code>value</code> into the set if …\nInserts a value computed from <code>f</code> into the set if the given …\nReturns a reference to the set’s <code>BuildHasher</code>.\nAdds a value to the set.\nSets the value of the entry, and returns an OccupiedEntry.\nSets the value of the entry with the VacantEntry’s value.\nInsert a value the set without checking if the value …\nVisits the values representing the intersection, i.e., the …\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCreates a consuming iterator, that is, one that moves each …\nTake ownership of the value.\nReturns <code>true</code> if <code>self</code> has no elements in common with <code>other</code>. …\nReturns <code>true</code> if the set contains no elements.\nReturns <code>true</code> if the set is a subset of another, i.e., <code>other</code>…\nReturns <code>true</code> if the set is a superset of another, i.e., …\nAn iterator visiting all elements in arbitrary order. The …\nReturns the number of elements in the set.\nCreates an empty <code>HashSet</code>.\nCreates an empty <code>HashSet</code>.\nEnsures a value is in the entry by inserting if it was …\nReturns a reference to the <code>RawTable</code> used underneath <code>HashSet</code>…\nReturns a mutable reference to the <code>RawTable</code> used …\nRemoves a value from the set. Returns whether the value was\nTakes the value out of the entry, and returns it. Keeps …\nAdds a value to the set, replacing the existing value, if …\nReplaces the entry, returning the old value. The new value …\nReserves capacity for at least <code>additional</code> more elements to …\nRetains only the elements specified by the predicate.\nShrinks the capacity of the set with a lower limit. It …\nShrinks the capacity of the set as much as possible. It …\nReturns the difference of <code>self</code> and <code>rhs</code> as a new …\nVisits the values representing the symmetric difference, …\nRemoves and returns the value in the set, if any, that is …\nTries to reserve capacity for at least <code>additional</code> more …\nVisits the values representing the union, i.e., all the …\nCreates an empty <code>HashSet</code> with the specified capacity.\nCreates an empty <code>HashSet</code> with the specified capacity, using\nCreates an empty <code>HashSet</code> with the specified capacity, using\nCreates an empty <code>HashSet</code> with the specified capacity.\nCreates a new empty hash set which will use the given …\nCreates a new empty hash set which will use the given …\nType representing the absence of an entry, as returned by …\nA draining iterator over the items of a <code>HashTable</code>.\nA view into a single entry in a table, which may either be …\nA draining iterator over entries of a <code>HashTable</code> which don…\nLow-level hash table with explicit hashing.\nAn owning iterator over the entries of a <code>HashTable</code> in …\nAn iterator over the entries of a <code>HashTable</code> in arbitrary …\nA mutable iterator over the entries of a <code>HashTable</code> in …\nAn occupied entry.\nA view into an occupied entry in a <code>HashTable</code>. It is part …\nA vacant entry.\nA view into a vacant entry in a <code>HashTable</code>. It is part of …\nReturns a reference to the underlying allocator.\nProvides in-place mutable access to an occupied entry …\nReturns the number of elements the table can hold without …\nClears the table, removing all values.\nClears the set, returning all elements in an iterator.\nReturns an <code>Entry</code> for an entry in the table with the given …\nDrains elements which are true under the given predicate, …\nReturns a reference to an entry in the table with the …\nReturns an <code>OccupiedEntry</code> for an entry in the table with …\nReturns a mutable reference to an entry in the table with …\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nGets a reference to the value in the entry.\nAttempts to get mutable references to <code>N</code> values in the map …\nAttempts to get mutable references to <code>N</code> values in the map …\nGets a mutable reference to the value in the entry.\nSets the value of the entry, replacing any existing value …\nInserts a new element into the table with the hash that …\nInserts an element into the <code>HashTable</code> with the given hash …\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nConverts the OccupiedEntry into a mutable reference to the …\nConverts the OccupiedEntry into a mutable reference to the …\nConverts the VacantEntry into a mutable reference to the …\nConverts the AbsentEntry into a mutable reference to the …\nReturns <code>true</code> if the set contains no elements.\nAn iterator visiting all elements in arbitrary order. The …\nAn iterator visiting all elements in arbitrary order, with …\nReturns the number of elements in the table.\nCreates an empty <code>HashTable</code>.\nCreates an empty <code>HashTable</code> using the given allocator.\nEnsures a value is in the entry by inserting if it was …\nEnsures a value is in the entry by inserting the result of …\nTakes the value out of the entry, and returns it along …\nReserves capacity for at least <code>additional</code> more elements to …\nRetains only the elements specified by the predicate.\nShrinks the capacity of the table with a lower limit. It …\nShrinks the capacity of the table as much as possible. It …\nTries to reserve capacity for at least <code>additional</code> more …\nCreates an empty <code>HashTable</code> with the specified capacity.\nCreates an empty <code>HashTable</code> with the specified capacity …\nA reference to a hash table bucket containing a <code>T</code>.\nA reference to an empty bucket into which an can be …\nIterator which consumes elements without freeing the table …\nIterator which consumes a table and returns elements.\nIterator which returns a raw pointer to every full bucket …\nIterator over occupied buckets that could match a given …\nA raw hash table with an unsafe API.\nReturn the information about memory allocated by the table.\nReturns a reference to the underlying allocator.\nReturns a unique mutable reference to the <code>value</code>.\nAcquires the underlying raw pointer <code>*mut T</code> to <code>data</code>.\nReturns a shared immutable reference to the <code>value</code>.\nReturns a pointer to an element in the table.\nReturns the index of a bucket from a <code>Bucket</code>.\nReturns the number of buckets in the table.\nReturns the number of elements the map can hold without …\nRemoves all elements from the table without freeing the …\nMarks all table buckets as empty without dropping their …\nVariant of <code>clone_from</code> to use when a hasher is available.\nCopies <code>size_of&lt;T&gt;</code> bytes from <code>other</code> to <code>self</code>. The source and …\nReturns pointer to one past last <code>data</code> element in the table …\nReturns pointer to start of data table.\nReturns an iterator which removes all elements from the …\nReturns an iterator which removes all elements from the …\nErases an element from the table, dropping it in place.\nFinds and erases an element from the table, dropping it in …\nSearches for an element in the table.\nSearches for an element in the table. If the element is …\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nGets a reference to an element in the table.\nAttempts to get mutable references to <code>N</code> entries in the …\nGets a mutable reference to an element in the table.\nInserts a new element into the table, and returns its raw …\nInserts a new element into the table, and returns a …\nInserts a new element into the table in the given slot, …\nInserts a new element into the table, without growing the …\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nReturns an iterator which consumes all elements from the …\nChecks whether the bucket at <code>index</code> is full.\nReturns <code>true</code> if the table contains no elements.\nReturns an iterator over every element in the table. It is …\nReturns an iterator over occupied buckets that could match …\nReturns the number of elements in the table.\nCreates a new empty hash table without allocating any …\nCreates a new empty hash table without allocating any …\nRefresh the iterator so that it reflects an insertion into …\nRefresh the iterator so that it reflects a removal from …\nRemoves an element from the table, returning it.\nFinds and removes an element from the table, returning it.\nTemporary removes a bucket, applying the given function to …\nEnsures that at least <code>additional</code> items can be inserted …\nShrinks the table to fit <code>max(self.len(), min_size)</code> …\nAttempts to insert a new element without growing the table …\nTries to ensure that at least <code>additional</code> items can be …\nAttempts to allocate a new hash table with at least enough …\nAttempts to allocate a new hash table using the given …\nAllocates a new hash table with at least enough capacity …\nAllocates a new hash table using the given allocator, with …")