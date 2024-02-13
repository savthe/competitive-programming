#include <algorithm>
#include <iostream>
#include <vector>

using namespace std;

void solve() {
  int n, m;
  cin >> n >> m;

  vector<int64_t> a(n);
  vector<int64_t> b(m);

  for (int i = 0; i < n; ++i) cin >> a[i];
  for (int i = 0; i < m; ++i) cin >> b[i];

  sort(a.begin(), a.end());
  sort(b.begin(), b.end());

  // Maximum sum.
  int64_t s = 0;

  // Set two pairs of pointers. (la, ra) - first and last elements of vector 'a', (lb, rb) - same
  // for vector 'b'.
  int la = 0, ra = n - 1;
  int lb = 0, rb = m - 1;

  // While there is at least one element in 'a' for which we didn't find 'a' pair in 'b' yet.
  while (la <= ra) {
    // Find 4 differences: first/last element in 'a' and first/last element in 'b'.
    auto lalb = std::abs(a[la] - b[lb]);
    auto larb = std::abs(a[la] - b[rb]);
    auto ralb = std::abs(a[ra] - b[lb]);
    auto rarb = std::abs(a[ra] - b[rb]);

    // Chose the pair with largest difference.
    if (lalb >= larb && lalb >= ralb && lalb >= rarb) {
      // First element in 'a' and first element in 'b' gives largest difference.
      s += lalb;
      ++la;
      ++lb;
    } else if (larb >= lalb && larb >= ralb && larb >= rarb) {
      // First element in 'a' and last element in 'b' gives largest difference.
      s += larb;
      ++la;
      --rb;
    } else if (ralb >= lalb && ralb >= larb && ralb >= rarb) {
      s += ralb;
      --ra;
      ++lb;
    } else if (rarb >= lalb && rarb >= larb && rarb >= ralb) {
      s += rarb;
      --ra;
      --rb;
    }
  }
  std::cout << s << std::endl;
}

int main() {
  ios_base::sync_with_stdio(false);
  cin.tie(0);
  int t;
  cin >> t;
  while (t--) {
    solve();
  }
}
