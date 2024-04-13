#include <iostream>
#include <vector>

using namespace std;

using ll = int64_t;

void solve() {
  int n;
  cin >> n;
  vector<int> v(n);
  // Счетчик тех, кто хочет жить справа во всем массиве.
  int rights = 0;
  for (int i = 0; i < n; ++i) {
    char c;
    cin >> c;
    v[i] = c - '0';
    rights += v[i] == 1;
  }

  // Рассматриваем возможное разделение на две стороны, начиная с самой левой
  // позиции, lefts -- количество желающих жить слева, среди живущих на левой
  // стороне.
  int lefts = 0;

  // m - позиция для дороги.
  int m = -1;

  // Если во всем массиве правожелающих хотя бы половина, то начальная позиция
  // для дороги -- самая левая (то есть все дома будут справа).
  if (rights >= (n + 1) / 2) m = 0;

  for (int i = 0; i < n; ++i) {
    // i -- позиция дороги, так что дома v[0]...v[i] - слева.
    lefts += v[i] == 0;
    rights -= v[i] == 1;

    // Количество домов слева и справа.
    int nleft = i + 1;
    int nright = n - nleft;

    // Если слева достаточно левожелающих, а справа -- правожелающих
    if (lefts >= (nleft + 1) / 2 && rights >= (nright + 1) / 2) {
      if (m == -1) m = nleft;

      // a, b -- количество домов слева и справа при разделении m.
      int a = m, b = n - m;
      if (std::abs(a - b) > std::abs(nleft - nright)) m = nleft;
    }
  }
  cout << m << endl;
}

int main() {
  int t;
  cin >> t;
  while (t--) {
    solve();
  }
}
