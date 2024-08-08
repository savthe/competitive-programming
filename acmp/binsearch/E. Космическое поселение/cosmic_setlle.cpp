#include <iostream>

using namespace std;

using ull = uint64_t;
ull n, a, b, w, h; 

bool test(ull d) {
    auto x = w / (a + 2 * d);
    auto y = h / (b + 2 * d);
    return x*y >= n;
}

ull solve() {
    ull low = 0, high = 1e18;
    while (low < high) {
        auto d = (low + high + 1) / 2;
        if (!test(d)) {
            high = d - 1;
        }
        else {
            low = d;
        }
    }

    return low;
}

int main() {
    cin >> n >> a >> b >> w >> h;
    auto s1 = solve();
    swap(w, h);
    auto s2 = solve();

    cout << max(s1, s2) << endl;
}

