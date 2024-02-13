#include <iostream>

using namespace std;

void solve() {
  int h, w, xa, ya, xb, yb; cin >> h >> w >> xa >> ya >> xb >> yb;
  // Если Алиса ниже Боба, то никто никого не съест.
  if (xa >= xb) {
    cout << "draw" << endl;
    return;
  }

  if ((xb - xa) & 1) {
    // Нечетное число ходов - либо выиграет Алиса, либо ничья.

    // Делаем первый ход Алисы в сторону Боба. После этого Алиса будет только реагировать
    // на ходы Боба.

    if (ya < yb) ++ya; // Боб справа.
    else if (ya > yb) --ya; // Боб слева.
    ++xa;

    // Если Алиса с Бобом на одном столбце, то Алиса побеждает (будет копировать ходы Боба).
    if (ya == yb) { cout << "alice" << endl; return; }

    // d - количество шагов Алисы в сторону боба, чтобы дойти до границы поля.
    int d = ya - 1;
    if (ya < yb) { d = w - ya; }

    // Высоты фишек после d шагов.
    xa += d;
    xb -= d;

    // Если Алиса не ниже Боба, то либо она уже выиграла, либо начнет копировать его ходы.
    if (xa <= xb) { cout << "alice" << endl; return; }
  }
  else {
    // Четное число ходов - либо выиграет Боб, либо ничья.

    // Алиса делает первый ход, а Боб реагирует.

    if (ya == yb) { cout << "bob" << endl; return; }
    int d = yb - 1;
    if (ya > yb) { d = w - yb; }

    xa += d;
    xb -= d;
    if (xa <= xb) { cout << "bob" << endl; return; }
  }
  cout << "Draw" << endl;
}

int main() {
  ios_base::sync_with_stdio(false); cin.tie(0);
  int t; cin >> t;
  while(t--) {
    solve();
  }
}
 

