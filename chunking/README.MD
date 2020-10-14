Each cell gets a numeric orderable ID.
When a cell is processed it locks all its neighbours.

Problem: Deadlock when two cells lock the same neighbours or if two neighbours are locking each other:

Suboptimal (all are) solution: Lock arbitration.

Locks are owned by a central arbitrator.
Acquiring locks is only possible through the arbitrator.
The arbitrator checks if all locks are available.
If they are then all locks are acquired at once.
If they are not then no locks are acquired and the thread will wait until all locks are free.

Downsides:
Global lock for all threads. (limits maximum possible parallelism)
Locks have to be acquired ahead of time. (meaning 4 out of 5 locked chunks are not processed at once)
