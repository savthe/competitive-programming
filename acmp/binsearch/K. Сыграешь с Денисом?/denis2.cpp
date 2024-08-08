#include <iostream>
using namespace std;

using ll = int64_t;
int main() {
    ll low = 1, high = 1e9;
    while (low < high) {
        auto m = (low + high) / 2;
        cout << "? " << m << endl;
        ll rep; cin >> rep;
        if (rep == m) low = m + 1;
        else high = m - rep;
    }
    cout << "! " << low << endl;
}

