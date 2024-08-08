#include <iostream>

using namespace std;

using ll = uint64_t;
ll n, k, x, y;
void solve(ll num) {
    // количество квартир в подъезде
    ll flats_in_column = (n - n / k) * y + (n / k) * x;
    // "смещаем" квартиру в первый подъезд
    num = (num - 1) % flats_in_column + 1;
    ll low = 1;
    ll high = n;
    while (low < high) {
        auto m = (low + high) / 2;
        // f = номер последней квартиры на m-м этаже
        auto f = (m - m / k) * y + (m / k) * x;
        if (f < num) {
            low = m + 1;
        }
        else {
            high = m;
        }
    }
    cout << low << endl;
}

int main() {
    cin >> n >> k >> x >> y;
    int t; cin >> t;
    while (t--) {
        ll num; cin >> num;
        solve(num);
    }
}

