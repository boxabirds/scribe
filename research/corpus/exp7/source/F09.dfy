  function MergeSortedWith<T(!new)>(left: seq<T>, right: seq<T>, lessThanOrEq: (T, T) -> bool) : (result :seq<T>)
    requires SortedBy(lessThanOrEq, left)
    requires SortedBy(lessThanOrEq, right)
    requires TotalOrdering(lessThanOrEq)
    ensures multiset(left + right) == multiset(result)
    ensures SortedBy(lessThanOrEq, result)
  {
    if |left| == 0 then
      right
    else if |right| == 0 then
      left
    else if lessThanOrEq(left[0], right[0]) then
      LemmaNewFirstElementStillSortedBy(left[0], MergeSortedWith(left[1..], right, lessThanOrEq), lessThanOrEq);
      assert left == [left[0]] + left[1..];

      [left[0]] + MergeSortedWith(left[1..], right, lessThanOrEq)

    else
      LemmaNewFirstElementStillSortedBy(right[0], MergeSortedWith(left, right[1..], lessThanOrEq), lessThanOrEq);
      assert right == [right[0]] + right[1..];

      [right[0]] + MergeSortedWith(left, right[1..], lessThanOrEq)
  }
