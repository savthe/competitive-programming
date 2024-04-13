#include <cstdint>
#include <iostream>
using namespace std;

using ull = uint64_t;

void solve() {
  ull a, b, m;
  cin >> a >> b >> m;
  // Найдется момент времени, в который обе установки выстрелят одновременно, поэтому считаем
  // количество целых отрезков длины a и длины b в отрезке длины m, плюс еще 2 салюта, которые
  // были в небе в момент общего выстрела.
  cout << 2 + m/a + m/b << endl;
}

int main() {
  int t;
  cin >> t;
  while (t--) {
    solve();
  }
}
