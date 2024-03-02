#include <iostream>
#include <set>
#include <vector>

using namespace std;

constexpr int M = 'z' - 'a' + 1;

// Fenwick tree.
vector<int64_t> fwt;

// Adds a to a single element. Keeps up others for computing prefix sum if needed.
void fw_add(int index, int a) {
  for(; index < fwt.size(); index += index & -index) {
    fwt[index] += a;
  }
}

// Computes sum in prefix [1, index]
int64_t fw_sum(int index) {
  int64_t s = 0;
  for(; index > 0; index -= index & -index) {
    s += fwt[index];
  }
  return s;
}

// The Fenwick tree above works for query types: 
//    1. Increase single element.
//    2. Compute prefix sum.
// We require another type of queries: 
//    1. add(l, r, x): increase each element in segment [l, r] by value x. 
//    2. at(i): get value of an element with index i.
// To do this we implement the following logic on top of Fenwick tree:
// To increase elements in segment [l, r] by x: we do the following: 
// increase l-th element in tree by x, decrease r+1-th element by x.
void add(int l, int r, int x) {
  fw_add(l, x);
  fw_add(r + 1, -x);
}

// To get the value at index, we compute prefix sum to the index-th element. 
int at(int index) {
  return fw_sum(index) % M;
}

void solve() {
  fwt.clear();

  int n, m; cin >> n >> m;
  // Assign sufficient elements with a little buffer.
  fwt.assign(n + 10, 0);

  for (int i = 1; i <= n; ++i) {
    char c; cin >> c;
    // Set value i to c - 'a'.
    add(i, i, c - 'a');
  }

  // NOTE we consider only palindromes of length 2 or 3 (for instance "aa", "aba"). All other
  // palindromes will contain them. Below we talk only about only 2 or 3-length palindromes.
  
  // Tests if we have a palindrome of length 2 or 3.
  auto test = [&](int index) {
    if (index >= 1 && index + 1 <= n && at(index) == at(index + 1)) return true;
    if (index >= 1 && index + 2 <= n && at(index) == at(index + 2)) return true;
    return false;
  };

  // Keep all indexes of found palindromes.
  set<int> palis;

  // Checks if there is palindrome starting at index. If so, adds it to palis. If not, removes.
  auto update = [&](int index) {
    if (test(index)) palis.insert(index);
    else palis.erase(index);
  };
  
  // Check if there is palindrome starting at i-th position. If so, store i in set.
  for (int i = 1; i <= n; ++i) update(i);

  // Reading queries.
  for (int i = 0; i < m; ++i) {

    int cmd; cin >> cmd;
    if (cmd == 1) {
      int l, r, x; cin >> l >> r >> x;
      add(l, r, x);
      // Palindromes within segment won't change. On the ends of segment the new palindromes may
      // emerge or disappear.
      update(l - 1);
      update(l - 2);
      update(r - 1);
      update(r - 2);
      update(r);
    }
    else {
      int l, r; cin >> l >> r;
      bool verdict = false;

      // Find first palindrome with index at least l.
      auto i = palis.lower_bound(l);
      if (i != palis.end()) {
        // Palindrome found. Check if it lies within the [l, r] segment.
        verdict = (at(*i) == at(*i + 1) && *i + 1 <= r) || (at(*i) == at(*i + 2) && *i + 2 <= r);
      }

      cout << (verdict ? "NO" : "YES") << endl;
    }
  }
}

int main() {
  int t; cin >> t;
  while(t--) {
    solve();
  }
}

