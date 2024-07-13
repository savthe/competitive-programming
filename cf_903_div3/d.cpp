#include <iostream>
#include <unordered_map>

using namespace std;

using ll = int64_t;

void solve() {
  int n; cin >> n;
  //cnt[p] = c; p is a prime, c is a cumulative number of occurrences of p in factorization of
  //elements of array a.
  unordered_map<int, int> cnt;

  for (int i = 0; i < n; ++i) {
    int a; cin >> a;
    // Divide a by all numbers [2, sqrt(a)], each divisor is removed, thus we get only prime
    // divisors.
    for (int d = 2; a > 1 && d * d <= a; ++d) {
      // c - counter of d divisors in a.
      int c = 0;

      // While d is divisor, count it and remove from a.
      while (a % d == 0) {
        c++;
        a /= d;
      }

      if (c > 0) cnt[d] += c;
    }

    // Since d is less that sqrt(a), we could miss one prime divisor. For example, if a = 10, then
    // d = 2, 3. We left with 5 unseen. There may be only one prime divisor greater than sqrt(a).
    if (a > 1)  cnt[a]++; 
  }

  // Check that every number of occurrences of each divisor can be divided into n elements of array.
  for (auto [d, c] : cnt) {
    if (c % n) {
      cout << "NO" << endl;
      return;
    }
  }

  cout << "YES" << endl;
}

int main() {
  ios_base::sync_with_stdio(false); cin.tie(0);

  int t; cin >> t;
  while(t--) {
    solve();
  }
}
