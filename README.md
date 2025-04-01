## How to run

    cargo build
    cargo run

## Example usage

    $ printf '[1,5,2,1,4]' | xh 'http://localhost:8080/add_batch/asdf'
    $ xh 'http://localhost:8080/stats/asdf/1'
    {
        "min": 1.0,
        "max": 5.0,
        "last": 4.0,
        "avg": 2.6,
        "var": 2.639999999999999
    }

## Notes

### REST API

Application listens on port 8080.  Endpoints are implemented as:

    POST /add_batch/{symbol}
        BODY: [<comma separated float numbers>]

    GET /stats/{symbol}/{k}

### NaN

NaN values are problematic to handle.
So in case of calculating average and variance I treat them as zeros.
In case of min and max I ignore them, they still count towards the
"last 1e{k} data points" but they cannot be min nor max.

### Variance formula

There are two variances: one that divides over N, one that divides over (N-1).
I don't know which I was supposed to use so I chose former.

### Complexity

/stats endpoint time complexity is O(log n) for finding min and max and
O(1) for finding average and variance.  So in total O(log n).

### Finding min/max

It is possible to improve finding min/max by using interval tree.
Find min/max complexity would still be O(log n), and inserting would in
worst case still be O(log n), but in average case inserting would go down to O(1).
And even if complexity stays the same interval tree would better utilize
memory: less memory used, and less fragmentation.

Maybe it is even possible to use RMQ and make find min/max O(1).
It is 13 years since I last used interval tree or RMQ and figuring out
how to implement one now would take me more than 3 hours probably.

### Calculating avg and variance: overflow and precision loss

Average and variance calculations use prefix sum arrays which is correct
as long as there is no overflow or precision loss.  Since we are dealing
with floats there will be some precision loss and results won't be exact.
We can already see that in the example above.
It would probably be better to use something like Decimal128.
But I will stop with what I got since I already spent over 3 hours on this.

### Tests

I should definitely add some tests, but unfortunately I don't have time.
