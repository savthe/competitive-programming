// https://acmp.ru/asp/do/index.asp?main=task&id_course=1&id_section=3&id_topic=37&id_problem=212

#include <iostream>

using namespace std;

int main() {
  uint64_t n; cin >> n;
  int c = 0;

  while (n > 1) {
    c++;
    // Делим пополам и каждый раз выбираем бОльшую половину (деление с округлением вверх).
    n = (n + 1) / 2;
  }

  cout << c << endl;
}

