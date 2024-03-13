#include <iostream>
#include <vector>

using namespace std;

using ll = int64_t;

void solve() {
  int n; cin >> n;
  vector<int> a(n + 1);
  for (int i = 1; i <= n; ++i) cin >> a[i];

  vector<int> dp(n + 1, n);
  // dp[i] = minimal excluded elements in prefix [0..i] for this prefix to be block sequence.
  // Values for yet unknown prefixes are maxed.
  dp[0] = 0;
  for (int i = 0; i <= n; ++i) {
    // To make [0..i] a block sequence, we can take [0..i-1] block sequence and exclude a[i].
    if (i > 0) dp[i] = min(dp[i], dp[i - 1] + 1);
    
    // At this point dp[i] is calculated. If a[i+1], a[i+2], ..., a[k], where k = i + 1 + a[i+1] is a 
    // block sequence starting at a[i+1], then we can say that dp[k] is at least dp[i].
    // Note that we can get to dp[k] from multiple positions prior to k. So we take min.
    if (i + 1 <= n) {
      const int k = i + 1 + a[i+1];
      if (k <= n) dp[k] = min(dp[k], dp[i]);
    }
  }

  cout << dp[n] << endl;
}

int main() {
  ios_base::sync_with_stdio(false); cin.tie(0);

  int t; cin >> t;
  while(t--) {
    solve();
  }
}
