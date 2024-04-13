#include <iostream>
#include <vector>

using namespace std;

using ll = int64_t;

void solve() {
  ll n, m;
  cin >> n >> m;
  vector<ll> a(n + 1), b(n + 1);
  for (ll i = 1; i <= n; ++i) cin >> a[i];
  for (ll i = 1; i <= n; ++i) cin >> b[i];

  // Динамика. a[i] -- наименьшая стоимость остановиться в i-й точке. b[i] -- наименьшая стоимость
  // оказаться в i-й точке, возможно, не останавливаясь.

  b[n] = min(a[n], b[n]);
  for (ll i = n - 1; i >= 1; --i) {
    // a[i] -> стоимость поменяться с i-м человеком местами + наименьшая стоимость оказаться в 
    // i+1-й точке.
    a[i] += b[i + 1];

    // b[i] -> наименьшая стоимость из стоимости оказаться в i-й точке и проехать через i-ю точку.
    b[i] = min(a[i], b[i] + b[i + 1]);
  }

  ll s = a[1];
  for (int i = 1; i <= m; ++i) {
    s = min(s, a[i]);
  }
  cout << s << endl;
}

int main() {
  ios_base::sync_with_stdio(false);
  cin.tie(0);

  ll t;
  cin >> t;
  while (t--) {
    solve();
  }
}
