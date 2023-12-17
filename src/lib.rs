pub struct Order {
    /// Time the order was created (inclusive).
    pub created_at: u64,
    /// Time the order ended (exclusive).
    pub executed_or_cancelled_at: u64,
    /// Number of shares in the order.
    pub number_of_shares: u64,
}

/// A query for the number of outstanding shares at a particular time.
pub struct Query {
    /// The time to query.
    pub time: u64,
}

#[derive(Clone, Default, PartialEq, Eq, Debug)]
pub struct QueryResult {
    /// Number of outstanding shares at the time of the query.
    pub outstanding_shares: u64,
}

/// For N orders and P queries, this has a growth order something like:
///
/// -   P log P (to sort the queries
/// -   N * 2 log P (to find the start and end of the range corresponding to each querty
/// -   To actually add up the queries that match some order takes time that is worst-case P but
///     best case 0, and the average case depends on how large the time ranges covered by the orders
///     are. If you figure it's something like log P, then the overal time of this method is:
/// P log P + N log P (average case)
pub fn query_binsearch(orders: &[Order], queries: &[Query]) -> Vec<QueryResult> {
    /// Mapping from a query time to the index in the result that it actually corresponds to.
    ///
    /// This is used so that the QueryResults list has the same order as the input even after
    /// sorting queries.
    struct PendingQuery {
        /// The time being queried.
        time: u64,
        /// The index in the query_results where results of this query get added up.
        output_idx: usize,
    }
    // Create a list of query results in the same order as the original queries.
    let mut query_results = vec![QueryResult::default(); queries.len()];
    let sorted_queries = {
        let mut queries: Vec<_> = queries
            .iter()
            .enumerate()
            .map(|(idx, query)| PendingQuery {
                time: query.time,
                output_idx: idx,
            })
            .collect();
        queries.sort_by_key(|query| query.time);
        queries
    };

    for order in orders {
        // partition_point gives an index such that for 0..idx the function is true and for idx..len
        // the function is false.
        // To find the first matching query we partition into
        // - queries that happen before the order's start time
        // - queries that happen after or at the order's start time (the idx we return).
        let first_matching_query = sorted_queries.partition_point(|query| query.time < order.created_at);
        // To fnd the last matching query, we partition into
        // - queries that happen before the order's end time
        // - queries that happen after or at the order's end time (the idx we return).
        let last_matching_query = sorted_queries.partition_point(|query| query.time < order.executed_or_cancelled_at);
        // Now the set of queries that fall within this order is given by: first..last (exclusive
        // range, since last_matching_query points to the first query of the queries after the order
        // time. Not that this might be an empty range if no queries fall in the range.
        for sorted_idx in first_matching_query..last_matching_query {
            let matching_query = &sorted_queries[sorted_idx];
            query_results[matching_query.output_idx].outstanding_shares += order.number_of_shares;
        }
    }
    query_results
}

/// For N orders and P queries, this has a growth order of N * P.
pub fn query_naive(orders: &[Order], queries: &[Query]) -> Vec<QueryResult> {
    let mut results = Vec::with_capacity(queries.len());
    for query in queries {
        let mut result = QueryResult::default();
        for order in orders {
            if (order.created_at..order.executed_or_cancelled_at).contains(&query.time) {
                result.outstanding_shares += order.number_of_shares;
            }
        }
        results.push(result);
    }
    results
}
