#include <iostream>
#include <vector>

using namespace std;

using ll = int64_t;

void solve() {
  int n, q;
  cin >> n >> q;
  vector<ll> a(n);
  for (int i = 0; i < n; ++i) cin >> a[i];

  // For d < kPivot we'll use prefix sums because number of terms in sum might
  // be large. 316 is approximately a square root of 100'000 - a maximum value
  // of n. Please pay attention at (*) and (**) below, we'll clearify the square
  // root later.
  static constexpr int kPivot = 316;

  // Let r = i % d.
  // p[d][i] = a[r] + a[r + d] + a[r + 2d] + ... + a[i - d] + a[i].   (1)
  vector<vector<ll>> p(kPivot, vector<ll>(n));

  // u[d][i] = a[r] + 2a[r + d] + 3a[r + 2d] + ... + h*a[i - d] + (h + 1)*a[i],
  // where h = i / d.(2)
  vector<vector<ll>> u(kPivot, vector<ll>(n));

  for (int d = 1; d < kPivot; ++d) {
    for (int i = 0; i < n; ++i) {
      const int h = i / d;
      p[d][i] = a[i];
      u[d][i] = (h + 1) * a[i];
      // Add sum of previous prefix if there is any.
      if (i >= d) {
        p[d][i] += p[d][i - d];
        u[d][i] += u[d][i - d];
      }
    }
  }
  // (*) Complexity: kPivot * n.

  while (q--) {
    int s, d, k;
    cin >> s >> d >> k;
    --s;
    ll result = 0;
    // If d is at least equals to pivot, compute sum directly.
    if (d >= kPivot) {
      for (int i = 0; i < k; ++i) result += a[s + i * d] * (i + 1);
    } else {
      const int h = s / d;
      // Take a sum (2) up to the right endpoint i = s + (k - 1)*d which is:
      // a[r] + 2a[r + d] + ... + ((s + (k - 1)*d) / d + 1) * a[s + (k - 1)*d] =
      // a[r] + 2a[r + d] + ... + (s/d + k) * a[s + (k - 1)*d]. (3)
      result += u[d][s + (k - 1) * d];

      // We need only k-segment, so subtract sum:
      // a[r] + 2a[r + d] + ... + (s - d)/d + 1)*a[s - d] = a[r] + 2a[r + d] +
      // ... + (s/d)*a[s - d].
      if (s >= d) result -= u[d][s - d];

      // Now result = (s/d + 1)*a[s] + (s/d + 2)*a[s + d] + ... + (s/d + k)*a[s
      // + (k - 1)*d]. Let's subtract from it the following sum: (s/d) * (a[s] +
      // a[s + d] + a[s + 2d] + ... + a[s + (k - 1)*d]).
      result -= h * (p[d][s + (k - 1) * d]);
      if (s >= d) result += h * p[d][s - d];
    }
    // (**) Complexity. 'else' block is O(1). Assume worst case, q = n, s = 0, d
    // >= kPivot and k is such that we take maximum terms: a[0], a[d], ...,
    // a[almost n]. Then we'll compute sum in 'if' block n times where in each
    // sum we take each d-th term, resulting in approximately n/d terms. As d >=
    // kPivot, the complexity in worst case is n * n / kPivot.

    cout << result << ' ';

    // Why kPivot is square root of n? Chose kPivot such that complexities (*)
    // and (**) are the same: kPivot * n = n * n / kPivot => kPivot = sqrt(n).
  }
  cout << endl;
}

int main() {
  // This makes cin significally faster.
  ios_base::sync_with_stdio(false);
  cin.tie(nullptr);

  int t;
  cin >> t;
  while (t--) {
    solve();
  }
}
