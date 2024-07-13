#include <algorithm>
#include <iostream>
#include <vector>
#include <unordered_set>

using namespace std;

using ll = int64_t;

void solve() {
  int n; cin >> n;

  vector<ll> v(n + 1);
  for (int i = 1; i <= n; ++i) cin >> v[i];

  vector<ll> p(n + 1);
  for (int i = 1; i <= n; ++i) cin >> p[i];

  vector<int> indexes(n + 1);
  for (int i = 0; i <=n; ++i) indexes[i] = i;

  // Сортируем indexes по убыванию значений v. Будем срывать грибы в порядке убывания волшебности.
  // Так мы всегда будем последним срывать гриб, который окажется минимальным в корзинке.
  sort(indexes.begin(), indexes.end(), [&](int i, int j) { return v[i] > v[j]; });

  // Вы корзине (bucket) храним индексы грибов, которые нарвали для эликсира.
  unordered_set <int> bucket;
  
  // Номер следующего срываемого гриба.
  int pick_index = 0;

  // Максимальная сила эликсира.
  ll max_power = 0;

  // Наименьшее количество грибов, которые дают эликсир с силой max_power.
  int min_shrooms = n;

  // take -- количество грибов, которые мы хотим сорвать для эликсира. Будем искать минимальное
  // количество грибов с максимальной силой.
  for (int take = 1; take <= n; ++take) {
    // Нужно ли удалить еще один гриб из кандидатов в эликсир.
    if (take >= 2) {
      const int remove = p[take - 1];
      v[remove] = 0;
      bucket.erase(remove);
    }

    // Пока в корзинке меньше грибов, чем take и под дубом еще есть грибы.
    while (bucket.size() < take && pick_index <= n) {
      // Индекс срываемого гриба.
      const int index = indexes[pick_index];
      
      // Если гриб хороший (с еще не зануленной волшебностью).
      if (v[index] > 0) bucket.insert(index);

      // Если получился эликсир лучше, чем раньше.
      if (bucket.size() * v[index] > max_power) {
        max_power = bucket.size() * v[index];
        min_shrooms = bucket.size();
      }

      ++pick_index;
    }
  }

  cout << max_power << ' ' << min_shrooms << endl;
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
