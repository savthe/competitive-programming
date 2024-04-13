#include <algorithm>
#include <iostream>
#include <vector>

using namespace std;

using ll = int64_t;

void solve() {
  int n, x;
  cin >> n >> x;
  vector<int> p(n + 1);
  for (int i = 1; i <= n; ++i) cin >> p[i];

  // Возвращает индекс элемента, на котором остановится предложенный бинпоиск.
  auto bin_search = [&]() {
    int l = 1;
    int r = n + 1;
    for (;;) {
      if (r - l == 1) break;

      const int m = (r + l) / 2;

      if (p[m] <= x)
        l = m;
      else
        r = m;
    }
    return l;
  };

  // В общем случае: меняем местами искомый элемент и последний, запускаем
  // бинпоиск и узнаем, где он остановится, меняем местами элемент на найденной
  // позиции с последним.

  // Если бинпоиск уже заканчивается на искомом элементе.
  if (p[bin_search()] == x) {
    cout << 0 << endl;
  } else {
    int xpos = find(p.begin(), p.end(), x) - p.begin();
    vector<pair<int, int>> result;
    // Если искомый элемент не последний.
    if (xpos < n) {
      swap(p[xpos], p[n]);
      result.push_back({xpos, n});
      xpos = n;
    }

    // Определяем, где остановится бинпоиск.
    int k = bin_search();
    if (p[k] != x) {
      result.push_back({xpos, k});
    }

    cout << result.size() << endl;
    for (auto [a, b] : result) cout << a << ' ' << b << endl;
  }
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
